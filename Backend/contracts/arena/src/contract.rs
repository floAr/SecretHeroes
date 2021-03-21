use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

use cosmwasm_std::{
    to_binary, Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HandleResult, HumanAddr,
    InitResponse, InitResult, Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use serde_json_wasm as serde_json;

use secret_toolkit::utils::{pad_handle_result, pad_query_result, HandleCallback, Query};

use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, WaitingHero};
use crate::rand::{sha_256, Prng};
use crate::state::{
    append_battle, append_battle_for_addr, get_history, load, may_load, save, CardContract, Config,
    Hero, StoredBattle, CONFIG_KEY, CONTRACT_KEY, PREFIX_VIEW_KEY,
};
use crate::viewing_key::{ViewingKey, VIEWING_KEY_SIZE};

pub const BLOCK_SIZE: usize = 256;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Expiration {
    AtHeight(u64),
    AtTime(u64),
    Never,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    ApproveToken,
    All,
    RevokeToken,
    None,
}

#[derive(Serialize)]
pub struct Transfer {
    pub recipient: HumanAddr,
    pub token_id: String,
    pub memo: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CardHandleMsg {
    RegisterReceiveNft {
        code_hash: String,
        padding: Option<String>,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    SetWhitelistedApproval {
        address: HumanAddr,
        token_id: Option<String>,
        view_owner: Option<AccessLevel>,
        view_private_metadata: Option<AccessLevel>,
        transfer: Option<AccessLevel>,
        expires: Option<Expiration>,
        padding: Option<String>,
    },
    BatchTransferNft {
        transfers: Vec<Transfer>,
        padding: Option<String>,
    },
    TransferNft {
        recipient: HumanAddr,
        token_id: String,
        memo: Option<String>,
        padding: Option<String>,
    },
}

impl HandleCallback for CardHandleMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize)]
pub struct ViewerInfo {
    pub address: HumanAddr,
    pub viewing_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CardQueryMsg {
    PrivateMetadata {
        token_id: String,
        viewer: Option<ViewerInfo>,
    },
}

impl Query for CardQueryMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateMetadata {
    pub private_metadata: Metadata,
}

////////////////////////////////////// Init ///////////////////////////////////////
/// Returns InitResult
///
/// Initializes the auction state and registers Receive function with sell and bid
/// token contracts
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
    let config = Config {
        heroes: Vec::new(),
        prng_seed,
        entropy: String::default(),
        battle_cnt: 0,
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;
    let key = base64::encode(config.prng_seed);
    let contract_raw = deps.api.canonical_address(&msg.card_contract.address)?;
    let card_contract = CardContract {
        address: contract_raw,
        code_hash: msg.card_contract.code_hash.clone(),
        viewing_key: key.clone(),
    };
    save(&mut deps.storage, CONTRACT_KEY, &card_contract)?;

    let reg_msg = CardHandleMsg::RegisterReceiveNft {
        code_hash: env.contract_code_hash,
        padding: None,
    };
    let reg_cosmos = reg_msg.to_cosmos_msg(
        msg.card_contract.code_hash,
        msg.card_contract.address.clone(),
        None,
    )?;
    let key_msg = CardHandleMsg::SetViewingKey { key, padding: None };
    let key_cosmos =
        key_msg.to_cosmos_msg(card_contract.code_hash, msg.card_contract.address, None)?;
    Ok(InitResponse {
        messages: vec![reg_cosmos, key_cosmos],
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
        HandleMsg::ReceiveNft {
            from,
            token_id,
            msg,
            ..
        } => try_receive(deps, env, from, token_id, msg),
        HandleMsg::ChickenOut {} => try_chicken(deps, env),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

pub fn try_chicken<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> HandleResult {
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    let owner_raw = deps.api.canonical_address(&env.message.sender)?;
    if let Some(pos) = config.heroes.iter().position(|h| h.owner == owner_raw) {
        let hero = config.heroes.swap_remove(pos);
        let card_contract: CardContract = load(&deps.storage, CONTRACT_KEY)?;
        let xfer_msg = CardHandleMsg::TransferNft {
            recipient: env.message.sender,
            token_id: hero.token_id.clone(),
            memo: None,
            padding: None,
        };
        let xfer_cosmos = xfer_msg.to_cosmos_msg(
            card_contract.code_hash,
            deps.api.human_address(&card_contract.address)?,
            None,
        )?;
        save(&mut deps.storage, CONFIG_KEY, &config)?;
        return Ok(HandleResponse {
            messages: vec![xfer_cosmos],
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

pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: HumanAddr,
    token_id: String,
    msg: Option<Binary>,
) -> HandleResult {
    let card_contract: CardContract = load(&deps.storage, CONTRACT_KEY)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    if card_contract.address != sender_raw {
        return Err(StdError::generic_err(format!(
            "This arena does not accept fighters from that guild (nft contract {})",
            env.message.sender
        )));
    }
    let owner_raw = deps.api.canonical_address(&from)?;
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if config.heroes.iter().any(|h| h.owner == owner_raw) {
        return Err(StdError::generic_err(
            "You already have a dog in this fight!",
        ));
    }
    if let Some(bin) = msg {
        let mut messages = Vec::new();
        let entropy: String = bin.to_base64();
        config.entropy.push_str(&entropy);
        let card_human = deps.api.human_address(&card_contract.address)?;
        let get_meta_msg = CardQueryMsg::PrivateMetadata {
            token_id: token_id.clone(),
            viewer: Some(ViewerInfo {
                address: env.contract.address.clone(),
                viewing_key: card_contract.viewing_key,
            }),
        };
        let priv_meta: PrivateMetadata = get_meta_msg.query(
            &deps.querier,
            card_contract.code_hash.clone(),
            card_human.clone(),
        )?;
        let skills: Vec<u8> = serde_json::from_str(
            &priv_meta
                .private_metadata
                .image
                .unwrap_or_else(|| "[0,0,0,0]".to_string()),
        )
        .map_err(|e| StdError::generic_err(format!("Error parsing private metadata: {}", e)))?;
        let new_hero = Hero {
            owner: owner_raw,
            token_id: token_id.clone(),
            name: priv_meta.private_metadata.name.unwrap_or_else(String::new),
            skills,
        };
        config.heroes.push(new_hero);
        if config.heroes.len() == 3 {
            let rand_slice = get_rand_slice(&env, &config.prng_seed, config.entropy.as_ref());
            let fight_idx = (rand_slice[0] % 4) as usize;
            config.entropy.clear();
            config.prng_seed = rand_slice.to_vec();
            let mut win_score = 0u8;
            let mut winners = Vec::new();
            let mut ties = Vec::new();
            let mut transfers = Vec::new();
            let mut opt_winner = None;
            for (i, hero) in config.heroes.iter().enumerate() {
                transfers.push(Transfer {
                    recipient: deps.api.human_address(&hero.owner)?,
                    token_id: hero.token_id.clone(),
                    memo: None,
                });
                let cur_score = hero.skills[fight_idx];
                match cur_score.cmp(&win_score) {
                    Ordering::Greater => {
                        win_score = cur_score;
                        winners = vec![i];
                    }
                    Ordering::Equal => winners.push(i),
                    _ => (),
                };
            }
            if winners.len() > 1 {
                let mut max = 0u16;
                for winner in winners {
                    let sum: u16 = config.heroes[winner].skills.iter().map(|u| *u as u16).sum();
                    match sum.cmp(&max) {
                        Ordering::Greater => {
                            max = sum;
                            ties = vec![winner];
                        }
                        Ordering::Equal => ties.push(winner),
                        _ => (),
                    };
                }
                winners = ties;
            }
            if winners.len() == 1 {
                opt_winner = Some(config.heroes[winners[0]].token_id.clone());
                let win_own = transfers[winners[0]].recipient.clone();
                for (i, xfer) in transfers.iter_mut().enumerate() {
                    if i != winners[0] {
                        xfer.recipient = win_own.clone();
                    }
                }
            }
            let xfer_msg = CardHandleMsg::BatchTransferNft {
                transfers,
                padding: None,
            };
            let xfer_cosmos = xfer_msg.to_cosmos_msg(card_contract.code_hash, card_human, None)?;
            messages.push(xfer_cosmos);

            let battle = StoredBattle {
                battle_number: config.battle_cnt,
                heroes: config.heroes.drain(..).collect(),
                skill_used: fight_idx as u8,
                winner: opt_winner,
                winning_skill_value: win_score,
            };
            append_battle(&mut deps.storage, &battle)?;
            for hero in battle.heroes {
                append_battle_for_addr(&mut deps.storage, config.battle_cnt, &hero.owner)?;
            }
            config.battle_cnt += 1;
        } else {
            let access_msg = CardHandleMsg::SetWhitelistedApproval {
                address: from,
                token_id: Some(token_id),
                view_owner: None,
                view_private_metadata: Some(AccessLevel::ApproveToken),
                transfer: None,
                expires: None,
                padding: None,
            };
            let acc_cosmos = access_msg.to_cosmos_msg(card_contract.code_hash, card_human, None)?;
            messages.push(acc_cosmos);
        }
        save(&mut deps.storage, CONFIG_KEY, &config)?;
        let resp = HandleResponse {
            messages,
            log: vec![],
            data: None,
        };
        return Ok(resp);
    }
    Err(StdError::generic_err(
        "You forgot to provide a password (random entropy string) when entering the arena",
    ))
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
    to_binary(&QueryAnswer::Bullpen {
        heroes_waiting: config.heroes.len() as u8,
        your_hero: config
            .heroes
            .into_iter()
            .find(|h| h.owner == address_raw)
            .map(|h| WaitingHero {
                token_id: h.token_id,
                name: h.name,
                skills: h.skills,
            }),
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
