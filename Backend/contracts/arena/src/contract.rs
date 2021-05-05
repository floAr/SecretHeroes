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
    ContractInfo, HandleAnswer, HandleMsg, InitMsg, PlayerStats, QueryAnswer, QueryMsg, TokenInfo,
    WaitingHero,
};
use crate::rand::{sha_256, Prng};
use crate::state::{
    append_battle_for_addr, get_history, load, may_load, remove, save, Config, Leaderboards, Rank,
    StoreBattle, StoreContractInfo, StoreHero, StorePlayerStats, StoreTokenInfo, StoreWaitingHero,
    Tourney, TourneyStats, ADMIN_KEY, BOTS_KEY, CONFIG_KEY, LEADERBOARDS_KEY, PREFIX_ALL_STATS,
    PREFIX_HISTORY, PREFIX_TOURN_STATS, PREFIX_VIEW_KEY,
};
use crate::stats::Stats;
use crate::viewing_key::{ViewingKey, VIEWING_KEY_SIZE};

pub const BLOCK_SIZE: usize = 256;
pub const LBOARD_MAX_LEN: usize = 20;

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
        player_cnt: 0,
    };
    let leaderboards = Leaderboards {
        tourney: Tourney {
            start: env.block.time,
            leaderboard: Vec::new(),
        },
        all_time: Vec::new(),
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    save(&mut deps.storage, ADMIN_KEY, &admin)?;
    save(&mut deps.storage, LEADERBOARDS_KEY, &leaderboards)?;
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
        HandleMsg::AddBots { bots } => try_add_bots(deps, env, bots),
        HandleMsg::RemoveBots { bots } => try_remove_bots(deps, env, bots),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

/// Returns HandleResult
///
/// add a list of addresses that auto-send fighters
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `new_bots` - list of bot addresses to add
pub fn try_add_bots<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_bots: Vec<HumanAddr>,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    let mut bots: Vec<CanonicalAddr> = may_load(&deps.storage, BOTS_KEY)?.unwrap_or_else(Vec::new);
    let old_len = bots.len();
    for bot in new_bots.iter() {
        let bot_raw = deps.api.canonical_address(bot)?;
        if !bots.contains(&bot_raw) {
            bots.push(bot_raw);
        }
    }
    // only save if the list changed
    if old_len != bots.len() {
        save(&mut deps.storage, BOTS_KEY, &bots)?;
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::AddBots {
            added_bots: new_bots,
        })?),
    })
}

/// Returns HandleResult
///
/// remove a list of auto-send addresses
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `no_bots` - list of bot addresses to remove
pub fn try_remove_bots<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    no_bots: Vec<HumanAddr>,
) -> HandleResult {
    let admin: CanonicalAddr = load(&deps.storage, ADMIN_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if sender_raw != admin {
        return Err(StdError::generic_err(
            "This is an admin command. Admin commands can only be run from admin address",
        ));
    }
    let may_bots: Option<Vec<CanonicalAddr>> = may_load(&deps.storage, BOTS_KEY)?;
    if let Some(mut bots) = may_bots {
        let old_len = bots.len();
        let no_raw: Vec<CanonicalAddr> = no_bots
            .iter()
            .map(|x| deps.api.canonical_address(x))
            .collect::<StdResult<Vec<CanonicalAddr>>>()?;
        bots.retain(|m| !no_raw.contains(m));
        let new_len = bots.len();
        if new_len > 0 {
            if old_len != new_len {
                save(&mut deps.storage, BOTS_KEY, &bots)?;
            }
        } else {
            remove(&mut deps.storage, BOTS_KEY);
        }
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::RemoveBots {
            removed_bots: no_bots,
        })?),
    })
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
                let bots: Vec<CanonicalAddr> =
                    may_load(&deps.storage, BOTS_KEY)?.unwrap_or_else(Vec::new);
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
                    let mut ignore = vec![false; 3];
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
                        ignore[i] = bots.contains(&hero.owner);
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
                        &mut deps.storage,
                        config.heroes.drain(..).collect(),
                        env.block.time,
                        &upgrade_rand,
                        &winners,
                        &totals,
                        &versions,
                        &mut messages,
                        &ignore,
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
                        timestamp: env.block.time,
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
        QueryMsg::Config {} => query_config(deps),
        QueryMsg::Bots {} => query_bots(deps),
        QueryMsg::Leaderboards {} => query_leaderboards(deps),
        QueryMsg::PlayerStats {
            address,
            viewing_key,
        } => query_player_stats(deps, address, viewing_key),
    };
    pad_query_result(response, BLOCK_SIZE)
}

/// Returns QueryResult displaying the list of auto-send addresses
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
pub fn query_bots<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let bots: Vec<CanonicalAddr> = may_load(&deps.storage, BOTS_KEY)?.unwrap_or_else(Vec::new);

    to_binary(&QueryAnswer::Bots {
        bots: bots
            .iter()
            .map(|m| deps.api.human_address(m))
            .collect::<StdResult<Vec<HumanAddr>>>()?,
    })
}

/// Returns QueryResult displaying a player's tournament stats and all-time stats
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
/// * `address` - querier's address
/// * `viewing_key` - querier's viewing key
pub fn query_player_stats<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
    viewing_key: String,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(&address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let address_slice = address_raw.as_slice();
    let trn_store = ReadonlyPrefixedStorage::new(PREFIX_TOURN_STATS, &deps.storage);
    let tourn_stats: TourneyStats =
        may_load(&trn_store, address_slice)?.unwrap_or_else(|| TourneyStats {
            last_seen: 0,
            stats: StorePlayerStats::default(),
        });
    let tournament = PlayerStats {
        score: tourn_stats.stats.score,
        address: address.clone(),
        battles: tourn_stats.stats.battles,
        wins: tourn_stats.stats.wins,
        ties: tourn_stats.stats.ties,
        third_in_two_way_ties: tourn_stats.stats.third_in_two_way_ties,
        losses: tourn_stats.stats.losses,
    };
    let all_store = ReadonlyPrefixedStorage::new(PREFIX_ALL_STATS, &deps.storage);
    let all_stats: StorePlayerStats =
        may_load(&all_store, address_slice)?.unwrap_or_else(StorePlayerStats::default);
    let all_time = PlayerStats {
        score: all_stats.score,
        address,
        battles: all_stats.battles,
        wins: all_stats.wins,
        ties: all_stats.ties,
        third_in_two_way_ties: all_stats.third_in_two_way_ties,
        losses: all_stats.losses,
    };

    to_binary(&QueryAnswer::PlayerStats {
        tournament,
        all_time,
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
        battles_have_halted: config.fight_halt,
    })
}

/// Returns QueryResult displaying the arena leaderboards
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
pub fn query_leaderboards<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> QueryResult {
    let mut leaderboards: Leaderboards = load(&deps.storage, LEADERBOARDS_KEY)?;
    leaderboards.all_time.truncate(10);
    leaderboards.tourney.leaderboard.truncate(10);
    let trn_store = ReadonlyPrefixedStorage::new(PREFIX_TOURN_STATS, &deps.storage);
    let tournament = leaderboards
        .tourney
        .leaderboard
        .iter()
        .map(|r| {
            load(&trn_store, r.address.as_slice())
                .and_then(|t: TourneyStats| t.stats.into_humanized(&deps.api, &r.address))
        })
        .collect::<StdResult<Vec<PlayerStats>>>()?;
    let all_store = ReadonlyPrefixedStorage::new(PREFIX_ALL_STATS, &deps.storage);
    let all_time = leaderboards
        .all_time
        .iter()
        .map(|r| {
            load(&all_store, r.address.as_slice())
                .and_then(|s: StorePlayerStats| s.into_humanized(&deps.api, &r.address))
        })
        .collect::<StdResult<Vec<PlayerStats>>>()?;

    to_binary(&QueryAnswer::Leaderboards {
        tournament,
        all_time,
    })
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

#[allow(clippy::too_many_arguments)]
fn update_skills<S: Storage>(
    storage: &mut S,
    fighters: Vec<StoreWaitingHero>,
    time: u64,
    rand: &[u8],
    winners: &[usize],
    totals: &[i16],
    versions: &[ContractInfo],
    messages: &mut Vec<CosmosMsg>,
    ignore: &[bool],
) -> StdResult<Vec<StoreHero>> {
    let adjust: [i8; 23] = [
        -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2,
    ];
    let mut heroes: Vec<StoreHero> = Vec::new();
    let is_tie = winners.len() != 1;
    let mut leaderboards: Leaderboards = load(storage, LEADERBOARDS_KEY)?;
    let mut save_boards = false;
    for (i, hero) in fighters.into_iter().enumerate() {
        let pre_battle_skills = hero.stats.current;
        let base = hero.stats.base;
        let post_battle_skills: Vec<u8>;
        let mut wins = 0u8;
        let mut ties = 0u8;
        let mut lose_ties = 0u8;
        let mut losses = 0u8;
        let delta: i8;
        if is_tie {
            // no skill changes on ties
            post_battle_skills = pre_battle_skills.clone();
            // tying fighter gets 1
            if winners.contains(&i) {
                ties = 1;
                delta = 1;
            // loser gets 0
            } else {
                lose_ties = 1;
                delta = 0;
            }
        // if this is the winner, give him an upgrade
        } else if i == winners[0] {
            // winners get 3 points
            wins = 1;
            delta = 3;
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
            // losers lose a point
            losses = 1;
            delta = -1;
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
        if !ignore[i] {
            let owner_slice = hero.owner.as_slice();
            let mut all_store = PrefixedStorage::new(PREFIX_ALL_STATS, storage);
            let may_all: Option<StorePlayerStats> = may_load(&all_store, owner_slice)?;
            let mut all_stats = if let Some(all) = may_all {
                all
            } else {
                StorePlayerStats::default()
            };
            all_stats.score += delta as i32;
            all_stats.battles += 1;
            all_stats.wins += wins as u32;
            all_stats.ties += ties as u32;
            all_stats.third_in_two_way_ties += lose_ties as u32;
            all_stats.losses += losses as u32;
            save(&mut all_store, owner_slice, &all_stats)?;
            update_leaderboard(
                &mut leaderboards.all_time,
                &hero.owner,
                all_stats.score,
                delta,
                LBOARD_MAX_LEN,
            );
            let mut trn_store = PrefixedStorage::new(PREFIX_TOURN_STATS, storage);
            let mut tourn_stats: TourneyStats =
                may_load(&trn_store, owner_slice)?.unwrap_or_else(|| TourneyStats {
                    last_seen: 0,
                    stats: StorePlayerStats::default(),
                });
            // check if tourney stats are from an older tournament
            if tourn_stats.last_seen < leaderboards.tourney.start {
                tourn_stats.stats = StorePlayerStats::default();
            }
            tourn_stats.last_seen = time;
            tourn_stats.stats.score += delta as i32;
            tourn_stats.stats.battles += 1;
            tourn_stats.stats.wins += wins as u32;
            tourn_stats.stats.ties += ties as u32;
            tourn_stats.stats.third_in_two_way_ties += lose_ties as u32;
            tourn_stats.stats.losses += losses as u32;
            save(&mut trn_store, owner_slice, &tourn_stats)?;
            update_leaderboard(
                &mut leaderboards.tourney.leaderboard,
                &hero.owner,
                tourn_stats.stats.score,
                delta,
                LBOARD_MAX_LEN,
            );
            save_boards = true;
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
    // if leaderboards have been updated
    if save_boards {
        save(storage, LEADERBOARDS_KEY, &leaderboards)?;
    }
    Ok(heroes)
}

fn update_leaderboard(
    leaderboard: &mut Vec<Rank>,
    player: &CanonicalAddr,
    score: i32,
    delta: i8,
    max_len: usize,
) {
    let mut insert_pos = 0usize;
    let mut old_pos = None;
    let mut last_of_group = 0usize;
    for (i, rank) in leaderboard.iter().enumerate().rev() {
        // track the player's old position
        if old_pos.is_none() && rank.address == *player {
            old_pos = Some(i);
        // determine where the player should be with his new score
        } else if insert_pos == 0 {
            match rank.score.cmp(&score) {
                Ordering::Greater => {
                    insert_pos = i + 1;
                }
                Ordering::Equal => {
                    // save the end of this score group
                    if last_of_group == 0 {
                        last_of_group = i + 1;
                    }
                    // if player did not get knocked down to this group
                    if delta >= 0 {
                        insert_pos = i + 1;
                    }
                }
                _ => (),
            };
        }
        // if we found both the insertion point and the old position, we're done
        if insert_pos > 0 && old_pos.is_some() {
            break;
        }
    }
    // if the player was already ranked
    if let Some(old) = old_pos {
        // don't do anything if the score didn't change
        if delta != 0 {
            // rank didn't change
            if insert_pos == old {
                leaderboard[insert_pos].score = score;
            } else {
                let mut append = leaderboard.split_off(old + 1);
                let mut leader = leaderboard.pop().unwrap();
                leader.score = score;
                // if rose in rank, insert before append
                if insert_pos < old {
                    leaderboard.insert(insert_pos, leader);
                    leaderboard.append(&mut append);
                // if fell in rank, insert position is one less after removing old spot
                } else {
                    leaderboard.append(&mut append);
                    leaderboard.insert(insert_pos - 1, leader);
                }
            }
        }
    // new arrival to leaderboard; don't grow past max len
    } else if insert_pos < max_len {
        // new arrivals should be at the lowest end of a group
        if delta < 0 && last_of_group != 0 {
            insert_pos = last_of_group;
        }
        // need to check if being last of a group is past the max len
        if insert_pos < max_len {
            leaderboard.insert(
                insert_pos,
                Rank {
                    score,
                    address: player.clone(),
                },
            );
            leaderboard.truncate(max_len);
        }
    }
}
