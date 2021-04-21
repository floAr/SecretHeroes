use std::cmp::Ordering;

use cosmwasm_std::{
    to_binary, Api, Binary, CanonicalAddr, CosmosMsg, Env, Extern, HandleResponse, HandleResult,
    HumanAddr, InitResponse, InitResult, Querier, QueryResult, ReadonlyStorage, StdError,
    StdResult, Storage,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use serde_json_wasm as serde_json;

use secret_toolkit::{
    snip721::{
        batch_transfer_nft_msg, private_metadata_query, register_receive_nft_msg,
        set_private_metadata_msg, set_viewing_key_msg, set_whitelisted_approval_msg,
        transfer_nft_msg, AccessLevel, Metadata, Transfer, ViewerInfo,
    },
    utils::{pad_handle_result, pad_query_result},
};

use crate::msg::{
    ContractInfo, HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, TokenInfo, WaitingHero,
};
use crate::rand::{sha_256, Prng};
use crate::state::{
    append_battle_for_addr, get_history, load, may_load, save, Config, StoreBattle,
    StoreContractInfo, StoreHero, StoreTokenInfo, StoreWaitingHero, ADMIN_KEY, CONFIG_KEY,
    PREFIX_HISTORY, PREFIX_VIEW_KEY,
};
use crate::stats::Stats;
use crate::viewing_key::{ViewingKey, VIEWING_KEY_SIZE};

pub const BLOCK_SIZE: usize = 256;

////////////////////////////////////// Init ///////////////////////////////////////
/// Returns InitResult
///
/// Initializes the arena and sets the viewing key and registers with the card contract
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
    let viewing_key = base64::encode(&prng_seed);
    let admin = deps.api.canonical_address(&env.message.sender)?;
    let mut config = Config {
        heroes: Vec::new(),
        prng_seed,
        entropy: String::default(),
        battle_cnt: 0,
        viewing_key,
        card_versions: vec![StoreContractInfo {
            code_hash: msg.card_contract.code_hash,
            address: deps.api.canonical_address(&msg.card_contract.address)?,
        }],
        fight_halt: false,
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    save(&mut deps.storage, ADMIN_KEY, &admin)?;
    let card_contract = config.card_versions.swap_remove(0);
    Ok(InitResponse {
        messages: vec![
            register_receive_nft_msg(
                env.contract_code_hash,
                Some(true),
                None,
                BLOCK_SIZE,
                card_contract.code_hash.clone(),
                msg.card_contract.address.clone(),
            )?,
            set_viewing_key_msg(
                config.viewing_key,
                None,
                BLOCK_SIZE,
                card_contract.code_hash,
                msg.card_contract.address,
            )?,
        ],
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
        HandleMsg::CreateViewingKey { entropy } => try_create_key(deps, env, &entropy),
        HandleMsg::SetViewingKey { key, .. } => try_set_key(deps, env, key),
        HandleMsg::BatchReceiveNft {
            from,
            token_ids,
            msg,
            ..
        } => try_receive(deps, env, from, &token_ids, msg),
        HandleMsg::ChickenOut {} => try_chicken(deps, env),
        HandleMsg::ChangeAdmin { address } => try_change_admin(deps, env, address),
        HandleMsg::SetBattleStatus { stop } => try_set_battle_status(deps, env, stop),
        HandleMsg::AddCardContract { card_contract } => {
            try_add_card_contract(deps, env, card_contract)
        }
    };
    pad_handle_result(response, BLOCK_SIZE)
}

/// Returns HandleResult
///
/// add a new card contract
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `card_contract` - new card ContractInfo
pub fn try_add_card_contract<S: Storage, A: Api, Q: Querier>(
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
    let address = deps.api.canonical_address(&card_contract.address)?;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    // only add the contract if we haven't seen it before
    if config
        .card_versions
        .iter()
        .find(|v| v.address == address)
        .is_none()
    {
        config.card_versions.push(StoreContractInfo {
            address,
            code_hash: card_contract.code_hash.clone(),
        });
        save(&mut deps.storage, CONFIG_KEY, &config)?;
        messages.push(register_receive_nft_msg(
            env.contract_code_hash,
            Some(true),
            None,
            BLOCK_SIZE,
            card_contract.code_hash.clone(),
            card_contract.address.clone(),
        )?);
        messages.push(set_viewing_key_msg(
            config.viewing_key,
            None,
            BLOCK_SIZE,
            card_contract.code_hash,
            card_contract.address.clone(),
        )?);
    }

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::AddCardContract {
            card_contract: card_contract.address,
        })?),
    })
}

/// Returns HandleResult
///
/// set the battle status
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `stop` - true if battles shold be halted
pub fn try_set_battle_status<S: Storage, A: Api, Q: Querier>(
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
    let mut messages: Vec<CosmosMsg> = Vec::new();
    // if battle status will change
    if config.fight_halt != stop {
        // if stopping battles and there are heroes in the bullpen
        if stop && !config.heroes.is_empty() {
            let mut version_xfers: Vec<VersionTransfer> = Vec::new();
            let versions = config
                .card_versions
                .iter()
                .map(|v| v.to_humanized(&deps.api))
                .collect::<StdResult<Vec<ContractInfo>>>()?;
            for hero in config.heroes.drain(..) {
                let transfer = Transfer {
                    recipient: deps.api.human_address(&hero.owner)?,
                    token_ids: vec![hero.token_info.token_id.clone()],
                    memo: None,
                };
                // if already encountered this version, add the transfer
                if let Some(vxfers) = version_xfers
                    .iter_mut()
                    .find(|v| v.version == hero.token_info.version)
                {
                    vxfers.transfers.push(transfer);
                // otherwise create a new list of transfers for this version
                } else {
                    version_xfers.push(VersionTransfer {
                        version: hero.token_info.version,
                        transfers: vec![transfer],
                    });
                }
            }
            for vxfer in version_xfers.into_iter() {
                messages.push(batch_transfer_nft_msg(
                    vxfer.transfers,
                    None,
                    BLOCK_SIZE,
                    versions[vxfer.version as usize].code_hash.clone(),
                    versions[vxfer.version as usize].address.clone(),
                )?);
            }
        }
        config.fight_halt = stop;
        save(&mut deps.storage, CONFIG_KEY, &config)?;
    }

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetBattleStatus {
            battles_have_halted: stop,
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
/// returns the message sender's hero waiting in the bullpen
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
pub fn try_chicken<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let owner_raw = deps.api.canonical_address(&env.message.sender)?;
    if let Some(pos) = config.heroes.iter().position(|h| h.owner == owner_raw) {
        let hero = config.heroes.swap_remove(pos);
        save(&mut deps.storage, CONFIG_KEY, &config)?;
        let card_contract =
            config.card_versions[hero.token_info.version as usize].to_humanized(&deps.api)?;
        return Ok(HandleResponse {
            messages: vec![transfer_nft_msg(
                env.message.sender,
                hero.token_info.token_id,
                None,
                None,
                BLOCK_SIZE,
                card_contract.code_hash,
                card_contract.address,
            )?],
            log: vec![],
            data: Some(to_binary(&HandleAnswer::ChickenOut {
                message: format!("{} fled", hero.name),
            })?),
        });
    }
    Err(StdError::generic_err(
        "You do not have any fighters in the bullpen",
    ))
}

/// Returns HandleResult
///
/// creates a viewing key
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `entropy` - string slice of the input String to be used as entropy in randomization
pub fn try_create_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    entropy: &str,
) -> HandleResult {
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    let key = ViewingKey::new(&env, &config.prng_seed, entropy.as_ref());
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, message_sender.as_slice(), &key.to_hashed())?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ViewingKey {
            key: format!("{}", key),
        })?),
    })
}

/// Returns HandleResult
///
/// sets the viewing key to the input String
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `key` - String to be used as the viewing key
pub fn try_set_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    key: String,
) -> HandleResult {
    let vk = ViewingKey(key.clone());
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, message_sender.as_slice(), &vk.to_hashed())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ViewingKey { key })?),
    })
}

/// Returns HandleResult
///
/// adds a hero to the bullpen and starts a battle if there are 3
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `token_ids` - list of heroes sent to the bullpen. Will error if more than 1
/// * `msg` - base64 encoded entropy string
pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: HumanAddr,
    token_ids: &[String],
    msg: Option<Binary>,
) -> HandleResult {
    if token_ids.len() != 1 {
        return Err(StdError::generic_err(
            "You may only send one hero to the arena!",
        ));
    }
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.fight_halt {
        return Err(StdError::generic_err("This arena has been shut down!"));
    }
    let mut versions = config
        .card_versions
        .iter()
        .map(|v| v.to_humanized(&deps.api))
        .collect::<StdResult<Vec<ContractInfo>>>()?;
    if let Some(pos) = versions
        .iter()
        .position(|v| v.address == env.message.sender)
    {
        if let Some(version) = versions.get(pos) {
            let owner_raw = deps.api.canonical_address(&from)?;
            if config.heroes.iter().any(|h| h.owner == owner_raw) {
                return Err(StdError::generic_err(
                    "You already have a dog in this fight!",
                ));
            }
            if let Some(bin) = msg {
                let mut messages = Vec::new();
                let entropy: String = bin.to_base64();
                config.entropy.push_str(&entropy);
                let viewer = Some(ViewerInfo {
                    address: env.contract.address.clone(),
                    viewing_key: config.viewing_key.clone(),
                });
                let priv_meta = private_metadata_query(
                    &deps.querier,
                    token_ids[0].clone(),
                    viewer,
                    BLOCK_SIZE,
                    version.code_hash.clone(),
                    version.address.clone(),
                )?;
                let stats: Stats = serde_json::from_str(
                    &priv_meta
                        .image
                        .ok_or_else(|| StdError::generic_err("Missing Hero Stats!"))?,
                )
                .map_err(|e| {
                    StdError::generic_err(format!("Error parsing private metadata: {}", e))
                })?;
                let new_hero = StoreWaitingHero {
                    owner: owner_raw,
                    name: priv_meta.name.unwrap_or_else(String::new),
                    token_info: StoreTokenInfo {
                        token_id: token_ids[0].clone(),
                        version: pos as u8,
                    },
                    stats,
                };
                config.heroes.push(new_hero);
                if config.heroes.len() == 3 {
                    let mut rand_slice =
                        get_rand_slice(&env, &config.prng_seed, config.entropy.as_ref());
                    let mut rand_iter = rand_slice.iter();
                    let fight_idx = (*(rand_iter.next().unwrap()) % 4u8) as usize;
                    let mut upgrade_rand: Vec<u8> = Vec::new();
                    while upgrade_rand.len() < 4 {
                        if let Some(rdm) = rand_iter.next() {
                            if *rdm < 253 {
                                upgrade_rand.push(*rdm);
                            }
                        } else {
                            rand_slice = get_rand_slice(&env, &rand_slice, config.entropy.as_ref());
                            rand_iter = rand_slice.iter();
                        }
                    }
                    config.entropy.clear();
                    config.prng_seed = rand_slice.to_vec();
                    let mut win_score = 0u8;
                    let mut winners = Vec::new();
                    let mut ties = Vec::new();
                    let mut version_xfers: Vec<VersionTransfer> = Vec::new();
                    let mut opt_winner = None;
                    let mut totals = vec![0i16; 4];
                    for (i, hero) in config.heroes.iter().enumerate() {
                        let transfer = Transfer {
                            recipient: deps.api.human_address(&hero.owner)?,
                            token_ids: vec![hero.token_info.token_id.clone()],
                            memo: None,
                        };
                        // if already encountered this version, add the transfer
                        if let Some(vxfers) = version_xfers
                            .iter_mut()
                            .find(|v| v.version == hero.token_info.version)
                        {
                            vxfers.transfers.push(transfer);
                        // otherwise create a new list of transfers for this version
                        } else {
                            version_xfers.push(VersionTransfer {
                                version: hero.token_info.version,
                                transfers: vec![transfer],
                            });
                        }
                        totals[i] = hero.stats.current.iter().map(|u| *u as i16).sum();
                        totals[3] += totals[i];
                        let cur_score = hero.stats.current[fight_idx];
                        match cur_score.cmp(&win_score) {
                            Ordering::Greater => {
                                win_score = cur_score;
                                winners = vec![i];
                            }
                            Ordering::Equal => winners.push(i),
                            _ => (),
                        };
                    }
                    // if there was a tie
                    if winners.len() > 1 {
                        let mut max = 0i16;
                        for winner in winners {
                            match totals[winner].cmp(&max) {
                                Ordering::Greater => {
                                    max = totals[winner];
                                    ties = vec![winner];
                                }
                                Ordering::Equal => ties.push(winner),
                                _ => (),
                            };
                        }
                        winners = ties;
                    }
                    // if there was a winner
                    if winners.len() == 1 {
                        opt_winner = Some(winners[0] as u8);
                        totals[3] -= totals[winners[0]];
                    }
                    let heroes = update_skills(
                        config.heroes.drain(..).collect(),
                        &upgrade_rand,
                        &winners,
                        &totals,
                        &versions,
                        &mut messages,
                    )?;
                    for vxfer in version_xfers.into_iter() {
                        messages.push(batch_transfer_nft_msg(
                            vxfer.transfers,
                            None,
                            BLOCK_SIZE,
                            versions[vxfer.version as usize].code_hash.clone(),
                            versions[vxfer.version as usize].address.clone(),
                        )?);
                    }
                    let battle = StoreBattle {
                        battle_number: config.battle_cnt,
                        heroes,
                        skill_used: fight_idx as u8,
                        winner: opt_winner,
                        winning_skill_value: win_score,
                    };
                    let mut his_store = PrefixedStorage::new(PREFIX_HISTORY, &mut deps.storage);
                    save(&mut his_store, &config.battle_cnt.to_le_bytes(), &battle)?;
                    for hero in battle.heroes {
                        append_battle_for_addr(&mut deps.storage, config.battle_cnt, &hero.owner)?;
                    }
                    config.battle_cnt += 1;
                } else {
                    let own_version = versions.swap_remove(pos);
                    messages.push(set_whitelisted_approval_msg(
                        from,
                        Some(token_ids[0].clone()),
                        None,
                        Some(AccessLevel::ApproveToken),
                        None,
                        None,
                        None,
                        BLOCK_SIZE,
                        own_version.code_hash,
                        own_version.address,
                    )?);
                }
                save(&mut deps.storage, CONFIG_KEY, &config)?;
                let resp = HandleResponse {
                    messages,
                    log: vec![],
                    data: None,
                };
                return Ok(resp);
            }
            return Err(StdError::generic_err(
                "You forgot to provide a password (random entropy string) when entering the arena",
            ));
        }
    }
    Err(StdError::generic_err(format!(
        "This arena does not accept fighters from that guild (nft contract {})",
        env.message.sender
    )))
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
        QueryMsg::Bullpen {
            address,
            viewing_key,
        } => query_bullpen(deps, &address, viewing_key),
        QueryMsg::BattleHistory {
            address,
            viewing_key,
            page,
            page_size,
        } => query_history(deps, &address, viewing_key, page, page_size),
    };
    pad_query_result(response, BLOCK_SIZE)
}

pub fn query_history<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
    page: Option<u32>,
    page_size: Option<u32>,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let history = get_history(
        &deps.api,
        &deps.storage,
        &address_raw,
        page.unwrap_or(0),
        page_size.unwrap_or(30),
    )?;
    to_binary(&QueryAnswer::BattleHistory { history })
}

pub fn query_bullpen<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    let heroes_waiting = config.heroes.len() as u8;
    let your_hero = if let Some(hero) = config.heroes.into_iter().find(|h| h.owner == address_raw) {
        Some(WaitingHero {
            name: hero.name,
            token_info: TokenInfo {
                token_id: hero.token_info.token_id,
                address: deps.api.human_address(
                    &config.card_versions[hero.token_info.version as usize].address,
                )?,
            },
            stats: hero.stats,
        })
    } else {
        None
    };

    to_binary(&QueryAnswer::Bullpen {
        heroes_waiting,
        your_hero,
    })
}

fn check_key<S: ReadonlyStorage>(
    storage: &S,
    address: &CanonicalAddr,
    viewing_key: String,
) -> StdResult<()> {
    // load the address' key
    let read_key = ReadonlyPrefixedStorage::new(PREFIX_VIEW_KEY, storage);
    let load_key: [u8; VIEWING_KEY_SIZE] =
        may_load(&read_key, address.as_slice())?.unwrap_or_else(|| [0u8; VIEWING_KEY_SIZE]);
    let input_key = ViewingKey(viewing_key);
    // if key matches
    if input_key.check_viewing_key(&load_key) {
        return Ok(());
    }
    Err(StdError::generic_err(
        "Wrong viewing key for this address or viewing key not set",
    ))
}

pub fn get_rand_slice(env: &Env, seed: &[u8], entropy: &[u8]) -> [u8; 32] {
    // 16 here represents the lengths in bytes of the block height and time.
    let entropy_len = 16 + env.message.sender.len() + entropy.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&env.block.time.to_be_bytes());
    rng_entropy.extend_from_slice(&env.message.sender.0.as_bytes());
    rng_entropy.extend_from_slice(entropy);

    let mut rng = Prng::new(seed, &rng_entropy);
    rng.rand_bytes()
}

// list of transfers for each card version in the fight
pub struct VersionTransfer {
    version: u8,
    transfers: Vec<Transfer>,
}

fn update_skills(
    fighters: Vec<StoreWaitingHero>,
    rand: &[u8],
    winners: &[usize],
    totals: &[i16],
    versions: &[ContractInfo],
    messages: &mut Vec<CosmosMsg>,
) -> StdResult<Vec<StoreHero>> {
    let adjust: [i8; 23] = [
        -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2,
    ];
    let mut heroes: Vec<StoreHero> = Vec::new();
    let is_tie = winners.len() != 1;
    for (i, hero) in fighters.into_iter().enumerate() {
        let pre_battle_skills = hero.stats.current;
        let base = hero.stats.base;
        let post_battle_skills: Vec<u8>;
        // no skill changes on ties
        if is_tie {
            post_battle_skills = pre_battle_skills.clone();
        // if this is the winner, give him an upgrade
        } else if i == winners[0] {
            let power_diff = 2 * totals[i] - totals[3];
            let mut rand_iter = rand.iter();
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
            post_battle_skills = pre_battle_skills
                .iter()
                .map(|u| {
                    if let Some(r) = rand_iter.next() {
                        let modified = base_upgrade + adjust[(*r as usize) % adjust.len()];
                        if modified > 0 {
                            let new_skill = modified as u8 + u;
                            if new_skill > 100 {
                                100
                            } else {
                                new_skill
                            }
                        } else {
                            *u
                        }
                    } else {
                        *u
                    }
                })
                .collect();
        // otherwise fracture a losing hero
        } else {
            let mut base_iter = base.iter();
            post_battle_skills = pre_battle_skills
                .iter()
                .map(|u| {
                    if let Some(b) = base_iter.next() {
                        u - (u - b) / 2
                    } else {
                        *u
                    }
                })
                .collect();
        }
        if pre_battle_skills != post_battle_skills {
            let stats = Stats {
                base,
                current: post_battle_skills.clone(),
            };
            let stats_str = serde_json::to_string(&stats).map_err(|e| {
                StdError::generic_err(format!("Error serializing card stats: {}", e))
            })?;
            let metadata = Metadata {
                name: Some(hero.name.clone()),
                description: None,
                image: Some(stats_str),
            };
            messages.push(set_private_metadata_msg(
                hero.token_info.token_id.clone(),
                metadata,
                None,
                BLOCK_SIZE,
                versions[hero.token_info.version as usize].code_hash.clone(),
                versions[hero.token_info.version as usize].address.clone(),
            )?);
        }
        heroes.push(StoreHero {
            owner: hero.owner,
            name: hero.name,
            token_info: hero.token_info,
            pre_battle_skills,
            post_battle_skills,
        });
    }
    Ok(heroes)
}
