use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    log, to_binary, Api, Binary, CanonicalAddr, Coin, CosmosMsg, Env, Extern, HandleResponse,
    HandleResult, HumanAddr, InitResponse, InitResult, Querier, QueryResponse, QueryResult,
    StdError, StdResult, Storage,
};

use secret_toolkit::utils::{pad_handle_result, HandleCallback, InitCallback};

use crate::msg::{
    ContractInitInfo, HandleAnswer, HandleMsg, InitMsg, QueryMsg, ResponseStatus::Success,
};
use crate::rand::{sha_256, Prng};
use crate::state::{load, save, AdminInfo, Config, ContractInfo, ADMIN_INFO_KEY, CONFIG_KEY};

use serde_json_wasm as serde_json;

pub const BLOCK_SIZE: usize = 256;

#[derive(Serialize)]
pub struct NftInitConfig {
    pub public_token_supply: Option<bool>,
    pub public_owner: Option<bool>,
    pub enable_sealed_metadata: Option<bool>,
    pub unwrapped_metadata_is_private: Option<bool>,
    pub minter_may_update_metadata: Option<bool>,
    pub owner_may_update_metadata: Option<bool>,
    pub enable_burn: Option<bool>,
}

#[derive(Serialize)]
pub struct PostInitCallback {
    pub msg: Binary,
    pub contract_address: HumanAddr,
    pub code_hash: String,
    pub send: Vec<Coin>,
}

#[derive(Serialize)]
pub struct NftInitMsg {
    pub name: String,
    pub symbol: String,
    pub admin: Option<HumanAddr>,
    pub entropy: String,
    pub config: Option<NftInitConfig>,
    pub post_init_callback: Option<PostInitCallback>,
}

impl InitCallback for NftInitMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NftHandleMsg {
    Mint {
        token_id: Option<String>,
        owner: Option<HumanAddr>,
        public_metadata: Option<Metadata>,
        private_metadata: Option<Metadata>,
        memo: Option<String>,
        padding: Option<String>,
    },
    ChangeAdmin {
        address: HumanAddr,
        padding: Option<String>,
    },
}

impl HandleCallback for NftHandleMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

////////////////////////////////////// Init ///////////////////////////////////////
/// Returns InitResult
///
/// Initializes the factory and creates a prng from the entropy String
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `msg` - InitMsg passed in with the instantiation message
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> InitResult {
    let prng_seed: Vec<u8> = sha_256(base64::encode(msg.entropy).as_bytes()).to_vec();
    let admin = AdminInfo {
        admin: deps.api.canonical_address(&env.message.sender)?,
        expect_reg: true,
    };
    let card_contract = ContractInfo {
        address: CanonicalAddr::default(),
        code_hash: msg.card_contract.code_hash.clone(),
    };

    let rdm_bytes = rdm_bytes(
        &env,
        &prng_seed,
        "Give NFT contract different entropy than the minter".as_ref(),
    );
    let nft_entropy = base64::encode(&rdm_bytes);

    let config = Config {
        card_contract,
        stopped: false,
        prng_seed,
    };

    save(&mut deps.storage, CONFIG_KEY, &config)?;
    save(&mut deps.storage, ADMIN_INFO_KEY, &admin)?;

    let cosmosmsg = card_init_msg(env, msg.card_contract_label, nft_entropy, msg.card_contract)?;

    Ok(InitResponse {
        messages: vec![cosmosmsg],
        log: vec![],
    })
}

///////////////////////////////////// Handle //////////////////////////////////////
/// Returns HandleResult
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `msg` - HandleMsg passed in with the execute message
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let response = match msg {
        HandleMsg::RegisterCardContract {} => try_register_contract(deps, env),
        HandleMsg::NewCardContract {
            label,
            entropy,
            card_contract,
        } => try_new_contract(deps, env, label, entropy, card_contract),
        HandleMsg::SetStatus { stop } => try_set_status(deps, env, stop),
        HandleMsg::Mint { names } => try_mint(deps, env, names),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

pub fn try_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    names: Vec<String>,
) -> HandleResult {
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.stopped {
        return Err(StdError::generic_err(
            "The minter has been stopped.  No new cards can be minted",
        ));
    }
    if names.len() < 3 {
        return Err(StdError::generic_err(
            "You must supply at least 3 names to mint 3 cards",
        ));
    }
    let entropy = names.join("");
    let rdm_bytes = rdm_bytes(&env, &config.prng_seed, entropy.as_ref());
    let mut messages = Vec::new();
    let nft_address = deps.api.human_address(&config.card_contract.address)?;

    for (i, name) in names.into_iter().enumerate() {
        if i > 2 {
            break;
        }
        let start_byte = i * 20;
        messages.push(mint_msg(
            &rdm_bytes[start_byte..start_byte + 20],
            name,
            &env.message.sender,
            &nft_address,
            &config.card_contract.code_hash,
        )?);
    }

    config.prng_seed = rdm_bytes;
    save(&mut deps.storage, CONFIG_KEY, &config)?;

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Status { status: Success })?),
    })
}

pub fn try_register_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let mut admin_info: AdminInfo = load(&deps.storage, ADMIN_INFO_KEY)?;
    if !admin_info.expect_reg {
        return Err(StdError::generic_err(
            "You are not permitted to call RegisterCardContract",
        ));
    }
    let admin_addr = deps.api.human_address(&admin_info.admin)?;
    admin_info.expect_reg = false;
    save(&mut deps.storage, ADMIN_INFO_KEY, &admin_info)?;

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    config.card_contract.address = deps.api.canonical_address(&env.message.sender)?;
    save(&mut deps.storage, CONFIG_KEY, &config)?;

    let change_adm_msg = NftHandleMsg::ChangeAdmin {
        address: admin_addr,
        padding: None,
    };
    let nft_address = deps.api.human_address(&config.card_contract.address)?;
    let cosmosmsg =
        change_adm_msg.to_cosmos_msg(config.card_contract.code_hash, nft_address.clone(), None)?;

    Ok(HandleResponse {
        messages: vec![cosmosmsg],
        log: vec![log("card contract address", nft_address)],
        data: Some(to_binary(&HandleAnswer::Status { status: Success })?),
    })
}

pub fn try_new_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    label: String,
    entropy: String,
    card_contract: ContractInitInfo,
) -> HandleResult {
    let mut admin_info: AdminInfo = load(&deps.storage, ADMIN_INFO_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin_info.admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    admin_info.expect_reg = true;
    save(&mut deps.storage, ADMIN_INFO_KEY, &admin_info)?;

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    config.card_contract.code_hash = card_contract.code_hash.clone();
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    let cosmosmsg = card_init_msg(env, label, entropy, card_contract)?;

    Ok(HandleResponse {
        messages: vec![cosmosmsg],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Status { status: Success })?),
    })
}

pub fn try_set_status<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    stop: bool,
) -> HandleResult {
    let admin_info: AdminInfo = load(&deps.storage, ADMIN_INFO_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin_info.admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.stopped != stop {
        config.stopped = stop;
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Status { status: Success })?),
    })
}

/////////////////////////////////////// Query /////////////////////////////////////
/// Returns QueryResult
///
/// # Arguments
///
/// * `deps` - reference to Extern containing all the contract's external dependencies
/// * `msg` - QueryMsg passed in with the query call
pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> QueryResult {
    Ok(QueryResponse::default())
}

fn mint_msg(
    bytes: &[u8],
    name: String,
    owner: &HumanAddr,
    nft_address: &HumanAddr,
    nft_code_hash: &str,
) -> StdResult<CosmosMsg> {
    let pub_meta = Metadata {
        name: Some(name.clone()),
        description: None,
        image: None,
    };
    let num_rolls = 5usize;
    let mut skills: Vec<u8> = Vec::new();
    for i in 0..4 {
        let val = bytes
            .iter()
            .skip(i * num_rolls)
            .take(num_rolls)
            .map(|b| (b % 100) + 1)
            .min()
            .unwrap_or(1);
        skills.push(val);
    }

    let skill_str = serde_json::to_string(&skills)
        .map_err(|e| StdError::generic_err(format!("Error serializing skills: {}", e)))?;
    let priv_meta = Metadata {
        name: Some(name.clone()),
        description: None,
        image: Some(skill_str),
    };
    let mint_msg = NftHandleMsg::Mint {
        token_id: Some(name),
        owner: Some(owner.clone()),
        public_metadata: Some(pub_meta),
        private_metadata: Some(priv_meta),
        memo: None,
        padding: None,
    };
    mint_msg.to_cosmos_msg(nft_code_hash.to_string(), nft_address.clone(), None)
}

fn card_init_msg(
    env: Env,
    label: String,
    entropy: String,
    card_contract: ContractInitInfo,
) -> StdResult<CosmosMsg> {
    let nft_config = NftInitConfig {
        public_token_supply: Some(true),
        public_owner: None,
        enable_sealed_metadata: None,
        unwrapped_metadata_is_private: None,
        minter_may_update_metadata: None,
        owner_may_update_metadata: None,
        enable_burn: None,
    };
    let post_init = PostInitCallback {
        msg: to_binary(&HandleMsg::RegisterCardContract {})?,
        contract_address: env.contract.address,
        code_hash: env.contract_code_hash,
        send: vec![],
    };

    let initmsg = NftInitMsg {
        name: card_contract.name,
        symbol: card_contract.symbol,
        admin: None,
        entropy,
        config: Some(nft_config),
        post_init_callback: Some(post_init),
    };
    initmsg.to_cosmos_msg(label, card_contract.code_id, card_contract.code_hash, None)
}

fn rdm_bytes(env: &Env, seed: &[u8], entropy: &[u8]) -> Vec<u8> {
    // 16 here represents the lengths in bytes of the block height and time.
    let entropy_len = 16 + env.message.sender.len() + entropy.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&env.block.time.to_be_bytes());
    rng_entropy.extend_from_slice(&env.message.sender.0.as_bytes());
    rng_entropy.extend_from_slice(entropy);

    let mut rng = Prng::new(seed, &rng_entropy);
    rng.rand_bytes().to_vec()
}
