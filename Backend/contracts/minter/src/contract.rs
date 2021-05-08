use cosmwasm_std::{
    to_binary, Api, BankMsg, CanonicalAddr, Coin, CosmosMsg, Env, Extern, HandleResponse,
    HandleResult, HumanAddr, InitResponse, InitResult, Querier, QueryResult, ReadonlyStorage,
    StdError, StdResult, Storage, Uint128,
};

use secret_toolkit::{
    snip721::{batch_mint_nft_msg, Metadata, Mint},
    utils::{pad_handle_result, pad_query_result},
};

use crate::msg::{
    ContractInfo, HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, ResponseStatus::Success,
};
use crate::rand::{sha_256, Prng};
use crate::state::{load, save, Config, StoreContractInfo, ADMIN_KEY, CONFIG_KEY};
use crate::stats::Stats;

use serde_json_wasm as serde_json;

pub const BLOCK_SIZE: usize = 256;

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
    let admin = deps.api.canonical_address(&env.message.sender)?;
    let config = Config {
        card_versions: vec![StoreContractInfo {
            code_hash: msg.card_contract.code_hash,
            address: deps.api.canonical_address(&msg.card_contract.address)?,
        }],
        minting_halt: false,
        multi_sig: deps.api.canonical_address(&msg.multi_sig)?,
        prng_seed,
        mint_cnt: 0,
    };

    save(&mut deps.storage, CONFIG_KEY, &config)?;
    save(&mut deps.storage, ADMIN_KEY, &admin)?;

    Ok(InitResponse::default())
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
        HandleMsg::NewMultiSig { address } => try_new_multi_sig(deps, env, address),
        HandleMsg::NewCardContract { card_contract } => {
            try_new_card_contract(deps, env, card_contract)
        }
        HandleMsg::SetMintStatus { stop } => try_set_mint_status(deps, env, stop),
        HandleMsg::Mint { names } => try_mint(deps, env, names),
        HandleMsg::ChangeAdmin { address } => try_change_admin(deps, env, address),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

/// Returns HandleResult
///
/// mint a pack of cards
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `names` - list of names for the newly minted cards
pub fn try_mint<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    names: Vec<String>,
) -> HandleResult {
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.minting_halt {
        return Err(StdError::generic_err(
            "The minter has been stopped.  No new cards can be minted",
        ));
    }
    if env.message.sent_funds.len() != 1
        || env.message.sent_funds[0].amount != Uint128(1000000)
        || env.message.sent_funds[0].denom != *"uscrt"
    {
        return Err(StdError::generic_err(
            "You must pay exactly 1 SCRT to buy a pack of heroes",
        ));
    }
    if names.len() < 3 {
        return Err(StdError::generic_err(
            "You must supply at least 3 names to mint 3 cards",
        ));
    }
    let entropy = names.join("");
    let rdm_bytes = rdm_bytes(&env, &config.prng_seed, entropy.as_ref());
    let mut mints = Vec::new();

    for (i, name) in names.into_iter().enumerate() {
        if i > 2 {
            break;
        }
        let start_byte = i * 20;
        mints.push(get_mints(
            &rdm_bytes[start_byte..start_byte + 20],
            name,
            &env.message.sender,
        )?);
    }
    config.prng_seed = rdm_bytes;
    config.mint_cnt += 1;
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    let card_contract = config
        .card_versions
        .pop()
        .ok_or_else(|| StdError::generic_err("Card version history is corrupt"))?;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    messages.push(batch_mint_nft_msg(
        mints,
        None,
        BLOCK_SIZE,
        card_contract.code_hash,
        deps.api.human_address(&card_contract.address)?,
    )?);
    let amount: Vec<Coin> = vec![Coin {
        denom: "uscrt".to_string(),
        amount: Uint128(1000000),
    }];
    messages.push(CosmosMsg::Bank(BankMsg::Send {
        from_address: env.contract.address,
        to_address: deps.api.human_address(&config.multi_sig)?,
        amount,
    }));
    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Mint { status: Success })?),
    })
}

/// Returns HandleResult
///
/// change the admin address
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `address` - the new admin address
pub fn try_change_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    address: HumanAddr,
) -> HandleResult {
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    if admin != sender_raw {
        return Err(StdError::generic_err(
            "This is an admin command and can only be run from the admin address",
        ));
    }
    let new_admin = deps.api.canonical_address(&address)?;
    if new_admin != admin {
        save(&mut deps.storage, ADMIN_KEY, &new_admin)?;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ChangeAdmin {
            new_admin: address,
        })?),
    })
}

/// Returns HandleResult
///
/// change the card contract
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `card_contract` - new card ContractInfo
pub fn try_new_card_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    card_contract: ContractInfo,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let new_address_raw = deps.api.canonical_address(&card_contract.address)?;
    let latest = config.card_versions.len() - 1;
    // if changing the version
    if config.card_versions[latest].address != new_address_raw {
        // check if reverting to a previous version
        if let Some(pos) = config
            .card_versions
            .iter()
            .position(|c| c.address == new_address_raw)
        {
            let old_version = config.card_versions.swap_remove(pos);
            config.card_versions.push(old_version);
        } else {
            // new version
            config.card_versions.push(StoreContractInfo {
                address: new_address_raw,
                code_hash: card_contract.code_hash,
            });
        }
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::NewCardContract {
            card_contract: card_contract.address,
        })?),
    })
}

/// Returns HandleResult
///
/// change the multi sig address
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `address` - the new multi sig address
pub fn try_new_multi_sig<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    address: HumanAddr,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let new_address_raw = deps.api.canonical_address(&address)?;
    if config.multi_sig != new_address_raw {
        config.multi_sig = new_address_raw;
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::NewMultiSig {
            multi_sig: address,
        })?),
    })
}

/// Returns HandleResult
///
/// set the minting status
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `stop` - true if minting shold be halted
pub fn try_set_mint_status<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    stop: bool,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.minting_halt != stop {
        config.minting_halt = stop;
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetMintStatus {
            minting_has_halted: stop,
        })?),
    })
}

/////////////////////////////////////// Query /////////////////////////////////////
/// Returns QueryResult
///
/// # Arguments
///
/// * `deps` - reference to Extern containing all the contract's external dependencies
/// * `msg` - QueryMsg passed in with the query call
pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    let response = match msg {
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::PacksMinted {} => query_packs_minted(&deps.storage),
    };
    pad_query_result(response, BLOCK_SIZE)
}

/// Returns QueryResult displaying the number of packs minted
///
/// # Arguments
///
/// * `storage` - a reference to the contract's storage
pub fn query_packs_minted<S: ReadonlyStorage>(storage: &S) -> QueryResult {
    let config: Config = load(storage, CONFIG_KEY)?;

    to_binary(&QueryAnswer::PacksMinted {
        packs_minted: config.mint_cnt,
    })
}

/// Returns QueryResult displaying the contract's config
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
pub fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    to_binary(&QueryAnswer::Config {
        card_versions: config
            .card_versions
            .iter()
            .map(|v| v.to_humanized(&deps.api))
            .collect::<StdResult<Vec<ContractInfo>>>()?,
        multi_sig_contract: deps.api.human_address(&config.multi_sig)?,
        minting_has_halted: config.minting_halt,
    })
}

fn get_mints(bytes: &[u8], name: String, owner: &HumanAddr) -> StdResult<Mint> {
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
    let stats = Stats {
        base: skills.clone(),
        current: skills,
    };
    let stats_str = serde_json::to_string(&stats)
        .map_err(|e| StdError::generic_err(format!("Error serializing card stats: {}", e)))?;
    let priv_meta = Metadata {
        name: Some(name),
        description: None,
        image: Some(stats_str),
    };
    let mint = Mint {
        token_id: None,
        owner: Some(owner.clone()),
        public_metadata: Some(pub_meta),
        private_metadata: Some(priv_meta),
        memo: None,
    };
    Ok(mint)
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
