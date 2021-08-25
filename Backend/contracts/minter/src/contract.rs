use cosmwasm_std::{
    to_binary, Api, BankMsg, CanonicalAddr, Coin, CosmosMsg, Env, Extern, HandleResponse,
    HandleResult, HumanAddr, InitResponse, InitResult, Querier, QueryResult, ReadonlyStorage,
    StdError, StdResult, Storage, Uint128,
};

use secret_toolkit::{
    snip721::{
        batch_burn_nft_msg, batch_mint_nft_msg, set_private_metadata_msg, set_viewing_key_msg,
        Burn, Metadata, Mint, ViewerInfo,
    },
    utils::{pad_handle_result, pad_query_result, Query},
};

use crate::contract_info::{ContractInfo, StoreContractInfo};
use crate::msg::{
    HandleAnswer, HandleMsg, HeroInfo, InitMsg, QueryAnswer, QueryMsg, ResponseStatus::Success,
};
use crate::rand::{extend_entropy, sha_256, Prng};
use crate::snip721::{NftDossierResponse, Snip721QueryMsg};
use crate::state::{load, save, Config, ADMIN_KEY, CONFIG_KEY, VKEY_KEY};
use crate::stats::Stats;
use crate::viewing_key::ViewingKey;

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
    let prng_seed: Vec<u8> = sha_256(base64::encode(msg.entropy.clone()).as_bytes()).to_vec();
    let vkey = ViewingKey::new(&env, &prng_seed, msg.entropy.as_ref());
    let keystr: String = format!("{}", vkey);
    save(&mut deps.storage, VKEY_KEY, &keystr)?;
    let admin = deps.api.canonical_address(&env.message.sender)?;
    save(&mut deps.storage, ADMIN_KEY, &admin)?;
    let config = Config {
        card_versions: vec![msg.card_contract.get_store(&deps.api)?],
        minting_halt: false,
        upgrade_halt: false,
        multi_sig: deps.api.canonical_address(&msg.multi_sig)?,
        prng_seed,
        mint_cnt: 0,
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;

    Ok(InitResponse {
        messages: vec![set_viewing_key_msg(
            keystr,
            None,
            BLOCK_SIZE,
            msg.card_contract.code_hash,
            msg.card_contract.address,
        )?],
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
        HandleMsg::NewMultiSig { address } => try_new_multi_sig(deps, &env.message.sender, address),
        HandleMsg::NewCardContract { card_contract } => {
            try_new_card_contract(deps, &env.message.sender, card_contract)
        }
        HandleMsg::AddLegacyCardContracts { card_contracts } => {
            try_add_legacy(deps, &env.message.sender, card_contracts)
        }
        HandleMsg::SetMintAndUpgradeStatus {
            stop_mint,
            stop_upgrade,
        } => try_set_mint_status(deps, &env.message.sender, stop_mint, stop_upgrade),
        HandleMsg::Mint { names } => try_mint(deps, env, names),
        HandleMsg::ChangeAdmin { address } => try_change_admin(deps, &env.message.sender, address),
        HandleMsg::AddMintCount { packs_minted } => {
            try_add_count(deps, &env.message.sender, packs_minted)
        }
        HandleMsg::Upgrade {
            burn,
            upgrade,
            entropy,
        } => try_upgrade(deps, env, burn, upgrade, &entropy),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

/// Returns HandleResult
///
/// burn 2 heroes to upgrade a third
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - the Env of contract's environment
/// * `burn` - list of heroes to burn
/// * `upgrade` - the hero to upgrade
/// * `entropy` - rng entropy string slice
fn try_upgrade<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    burn: Vec<HeroInfo>,
    upgrade: HeroInfo,
    entropy: &str,
) -> HandleResult {
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.upgrade_halt {
        return Err(StdError::generic_err("Upgrades have been halted"));
    }
    if burn.len() != 2 {
        return Err(StdError::generic_err(
            "You must burn exactly two heroes to do an upgrade",
        ));
    }
    let mut version_burns: Vec<VersionBurn> = Vec::new();
    let mut prng = get_prng(&env, &config.prng_seed, entropy.as_ref());
    config.prng_seed = prng.rand_bytes().to_vec();
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    let whitelist_err = format!(
        "This contract: {} has not been fully whitelisted on NFT contract:",
        &env.contract.address
    );
    let viewing_key: String = load(&deps.storage, VKEY_KEY)?;
    let viewer = ViewerInfo {
        address: env.contract.address,
        viewing_key,
    };
    let mut burn_points = 0i16;
    for hero in burn.into_iter() {
        let stored_ci = if let Some(vburn) = version_burns
            .iter_mut()
            .find(|v| v.human == hero.contract_address)
        {
            // already burned from this contract so just add the token id
            vburn.burns.token_ids.push(hero.token_id.clone());
            &vburn.stored
        } else {
            // first time burning from this contract
            let raw = deps.api.canonical_address(&hero.contract_address)?;
            let idx = config.card_versions.iter().position(|v| v.address == raw).ok_or_else(|| StdError::generic_err(format!("Can not burn heroes from an unknown guild (Unknown NFT contract address: {})", hero.contract_address)))?;
            let stored = config.card_versions.swap_remove(idx);
            version_burns.push(VersionBurn {
                human: hero.contract_address.clone(),
                burns: Burn {
                    token_ids: vec![hero.token_id.clone()],
                    memo: Some(format!("Burned to upgrade token_id {}", &upgrade.token_id)),
                },
                stored,
            });
            let last = version_burns
                .last()
                .ok_or_else(|| StdError::generic_err("We just pushed so this is impossible"))?;
            &last.stored
        };
        // get the stats and sum the hero's skill points
        let (stats, _m) = get_stats(
            &deps.querier,
            hero.token_id,
            viewer.clone(),
            stored_ci,
            hero.contract_address,
            &env.message.sender,
            &whitelist_err,
            "burn",
        )?;
        burn_points += stats.current.iter().map(|u| *u as i16).sum::<i16>();
    }

    let stored_ci = if let Some(vburn) = version_burns
        .iter()
        .find(|v| v.human == upgrade.contract_address)
    {
        &vburn.stored
    } else {
        let raw = deps.api.canonical_address(&upgrade.contract_address)?;
        &(config.card_versions.iter().find(|v| v.address == raw).ok_or_else(|| StdError::generic_err(format!("Can not upgrade heroes from an unknown guild (Unknown NFT contract address: {})", upgrade.contract_address)))?)
    };
    let (mut stats, mut priv_meta) = get_stats(
        &deps.querier,
        upgrade.token_id.clone(),
        viewer,
        stored_ci,
        upgrade.contract_address.clone(),
        &env.message.sender,
        &whitelist_err,
        "upgrade",
    )?;
    let pre_upgrade_skills = stats.current;
    let pre_sum = pre_upgrade_skills.iter().map(|u| *u as i16).sum::<i16>();
    // do the upgrade
    let power_diff = 2 * pre_sum - burn_points;
    let adjust: [i8; 23] = [
        -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2,
    ];
    let mut rand_iter = config.prng_seed.iter();
    // find 4 bytes that are less than 253
    let mut upgrade_rand: Vec<u8> = Vec::new();
    while upgrade_rand.len() < 4 {
        if let Some(rdm) = rand_iter.next() {
            if *rdm < 253 {
                upgrade_rand.push(*rdm);
            }
        } else {
            // get more random bytes
            config.prng_seed = prng.rand_bytes().to_vec();
            rand_iter = config.prng_seed.iter();
        }
    }
    let base_upgrade: i8 = if power_diff > 160 {
        -1
    } else if power_diff > 80 {
        0
    } else if power_diff > 0 {
        1
    } else if power_diff >= -80 {
        2
    } else if power_diff >= -200 {
        3
    } else {
        4
    };
    let mut upgrade_iter = upgrade_rand.iter();
    let mod_val = adjust.len();
    let post_upgrade_skills = pre_upgrade_skills
        .iter()
        .map(|u| {
            let r = upgrade_iter.next().ok_or_else(|| {
                StdError::generic_err("Can not have less than 4 random upgrade bytes")
            })?;
            let modified = base_upgrade + adjust[(*r as usize) % mod_val];
            if modified > 0 {
                let new_skill = modified as u8 + u;
                if new_skill > 100 {
                    Ok(100)
                } else {
                    Ok(new_skill)
                }
            } else {
                Ok(*u)
            }
        })
        .collect::<StdResult<Vec<u8>>>()?;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    if pre_upgrade_skills != post_upgrade_skills {
        stats.current = post_upgrade_skills.clone();
        let stats_str = serde_json::to_string(&stats)
            .map_err(|e| StdError::generic_err(format!("Error serializing card stats: {}", e)))?;
        priv_meta.image = Some(stats_str);
        messages.push(set_private_metadata_msg(
            upgrade.token_id,
            priv_meta,
            None,
            BLOCK_SIZE,
            stored_ci.code_hash.clone(),
            upgrade.contract_address,
        )?);
    }
    // burn the other 2 heroes
    for vburn in version_burns.into_iter() {
        messages.push(batch_burn_nft_msg(
            vec![vburn.burns],
            None,
            BLOCK_SIZE,
            vburn.stored.code_hash,
            vburn.human,
        )?);
    }
    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Upgrade {
            pre_upgrade_skills,
            post_upgrade_skills,
        })?),
    })
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
fn try_mint<S: Storage, A: Api, Q: Querier>(
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
    let mut prng = get_prng(&env, &config.prng_seed, entropy.as_ref());
    let rdm_bytes = prng.rand_bytes().to_vec();
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
/// add count of previous packs minted
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `sender` - a reference to the message sender
/// * `packs_minted` - number of previous packs minted
fn try_add_count<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    packs_minted: u32,
) -> HandleResult {
    let sender_raw = deps.api.canonical_address(sender)?;
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    if admin != sender_raw {
        return Err(StdError::generic_err(
            "This is an admin command and can only be run from the admin address",
        ));
    }
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    config.mint_cnt += packs_minted;
    save(&mut deps.storage, CONFIG_KEY, &config)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::AddMintCount {
            packs_added: packs_minted,
        })?),
    })
}

/// Returns HandleResult
///
/// change the admin address
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `sender` - a reference to the message sender
/// * `address` - the new admin address
fn try_change_admin<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    address: HumanAddr,
) -> HandleResult {
    let sender_raw = deps.api.canonical_address(sender)?;
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
/// add compatible card contract versions without changing the current contract used for minting
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `sender` - a reference to the message sender
/// * `card_contracts` - list of card contracts to add
fn try_add_legacy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    card_contracts: Vec<ContractInfo>,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let current_pos = config.card_versions.len() - 1;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    let vkey: String = load(&deps.storage, VKEY_KEY)?;
    for contract in card_contracts.into_iter() {
        let raw = contract.get_store(&deps.api)?;
        // if this contract is not already in the list
        if config
            .card_versions
            .iter()
            .find(|c| c.address == raw.address)
            .is_none()
        {
            // add to version list
            config.card_versions.push(raw);
            // set the viewing key with the new card contract
            messages.push(set_viewing_key_msg(
                vkey.clone(),
                None,
                BLOCK_SIZE,
                contract.code_hash,
                contract.address,
            )?);
        }
    }
    let new_last_pos = config.card_versions.len() - 1;
    // only save if something was added
    if current_pos != new_last_pos {
        // move current version to the last spot
        config.card_versions.swap(current_pos, new_last_pos);
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::AddLegacyCardContracts {
            card_versions: config
                .card_versions
                .into_iter()
                .map(|c| c.into_humanized(&deps.api))
                .collect::<StdResult<Vec<ContractInfo>>>()?,
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
/// * `sender` - a reference to the message sender
/// * `card_contract` - new card ContractInfo
fn try_new_card_contract<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    card_contract: ContractInfo,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }

    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let new_address_raw = deps.api.canonical_address(&card_contract.address)?;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    // if changing the version
    if config
        .card_versions
        .last()
        .ok_or_else(|| StdError::generic_err("Card version list is corrupt"))?
        .address
        != new_address_raw
    {
        // check if reverting to a previous version
        if let Some(pos) = config
            .card_versions
            .iter()
            .position(|c| c.address == new_address_raw)
        {
            // it was an old version so just swap it to the last position
            let last_pos = config.card_versions.len() - 1;
            config.card_versions.swap(pos, last_pos);
        } else {
            // new version
            config.card_versions.push(StoreContractInfo {
                address: new_address_raw,
                code_hash: card_contract.code_hash.clone(),
            });
            // set the viewing key with the new card contract
            let vkey: String = load(&deps.storage, VKEY_KEY)?;
            messages.push(set_viewing_key_msg(
                vkey,
                None,
                BLOCK_SIZE,
                card_contract.code_hash,
                card_contract.address.clone(),
            )?);
        }
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages,
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
/// * `sender` - a reference to the message sender
/// * `address` - the new multi sig address
fn try_new_multi_sig<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    address: HumanAddr,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(sender)?;
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
/// set the minting and/or upgrade status
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `sender` - a reference to the message sender
/// * `stop_mint` - true if minting should be halted
/// * `stop_upgrade` - true if upgrades should be halted
fn try_set_mint_status<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    sender: &HumanAddr,
    stop_mint: Option<bool>,
    stop_upgrade: Option<bool>,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let mut save_it = false;
    if let Some(mint) = stop_mint {
        if config.minting_halt != mint {
            config.minting_halt = mint;
            save_it = true;
        }
    }
    if let Some(upgrade) = stop_upgrade {
        if config.upgrade_halt != upgrade {
            config.upgrade_halt = upgrade;
            save_it = true;
        }
    }
    if save_it {
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetMintAndUpgradeStatus {
            minting_has_halted: config.minting_halt,
            upgrades_have_halted: config.upgrade_halt,
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
fn query_packs_minted<S: ReadonlyStorage>(storage: &S) -> QueryResult {
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
fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    to_binary(&QueryAnswer::Config {
        card_versions: config
            .card_versions
            .into_iter()
            .map(|v| v.into_humanized(&deps.api))
            .collect::<StdResult<Vec<ContractInfo>>>()?,
        multi_sig_contract: deps.api.human_address(&config.multi_sig)?,
        minting_has_halted: config.minting_halt,
        upgrades_have_halted: config.upgrade_halt,
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

/// Returns Prng
///
/// creates a new Prng
///
/// # Arguments
///
/// * `env` - a reference to the Env of contract's environment
/// * `seed` - entropy source coming from the contract creator
/// * `entropy` - additional entropy that may come from a user if needed
fn get_prng(env: &Env, seed: &[u8], entropy: &[u8]) -> Prng {
    let rng_entropy = extend_entropy(env, entropy);
    Prng::new(seed, &rng_entropy)
}

/// Returns StdResult<(Stats, Metadata)>
///
/// get the stats and private metadata of a hero and verify that the message sender is its owner
///
/// # Arguments
///
/// * `querier` - a reference to the Querier dependency of the contract
/// * `token_id` - the token ID of the hero whose stats are being queried
/// * `viewer` - the contract's address and viewing key with the token contract
/// * `raw_contract` - a reference to the StoreContractInfo of the token contract
/// * `human_contract` - the human address of the token contract
/// * `sender` - a reference to the message sender's address
/// * `whitelist_err` - error message that says the minter has not been whitelisted
/// * `action` - either "burn" or "upgrade" depending on what is being done to the hero
#[allow(clippy::too_many_arguments)]
fn get_stats<Q: Querier>(
    querier: &Q,
    token_id: String,
    viewer: ViewerInfo,
    raw_contract: &StoreContractInfo,
    human_contract: HumanAddr,
    sender: &HumanAddr,
    whitelist_err: &str,
    action: &str,
) -> StdResult<(Stats, Metadata)> {
    // get the owner and private metadata of the token
    let dossier_resp: NftDossierResponse = Snip721QueryMsg::NftDossier { token_id, viewer }.query(
        querier,
        raw_contract.code_hash.clone(),
        human_contract.clone(),
    )?;
    let owner = dossier_resp
        .nft_dossier
        .owner
        .ok_or_else(|| StdError::generic_err(format!("{} {}", whitelist_err, human_contract)))?;
    if owner != *sender {
        return Err(StdError::generic_err(format!(
            "You can not {} a hero you do not own!",
            action
        )));
    }
    let priv_meta = dossier_resp
        .nft_dossier
        .private_metadata
        .ok_or_else(|| StdError::generic_err(format!("{} {}", whitelist_err, human_contract)))?;
    let stats: Stats = serde_json::from_str(
        priv_meta
            .image
            .as_ref()
            .ok_or_else(|| StdError::generic_err("Missing Hero Stats!"))?,
    )
    .map_err(|e| StdError::generic_err(format!("Error parsing private metadata: {}", e)))?;
    Ok((stats, priv_meta))
}

// list of burns for each card version
pub struct VersionBurn {
    pub human: HumanAddr,
    pub burns: Burn,
    pub stored: StoreContractInfo,
}
