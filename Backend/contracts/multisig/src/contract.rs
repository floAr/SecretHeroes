use cosmwasm_std::{
    to_binary, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Env, Extern, HandleResponse,
    HandleResult, HumanAddr, InitResponse, InitResult, Querier, QueryResult, ReadonlyStorage,
    StdError, StdResult, Storage, Uint128,
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::utils::{pad_handle_result, pad_query_result};

use crate::msg::{Action, HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, Status, Vote};
use crate::proposal::{Proposal, StoredAction, StoredProposal};
use crate::rand::sha_256;
use crate::state::{
    get_proposals, json_may_load, json_save, load, may_load, save, Config, CONFIG_KEY,
    PREFIX_PROPOSAL, PREFIX_VIEW_KEY, PRNG_KEY,
};
use crate::viewing_key::{ViewingKey, VIEWING_KEY_SIZE};

pub const BLOCK_SIZE: usize = 256;

////////////////////////////////////// Init ///////////////////////////////////////
/// Returns InitResult
///
/// Initializes the multi-sig contract
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
    save(&mut deps.storage, PRNG_KEY, &prng_seed)?;
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    let config = Config {
        voters: vec![sender_raw],
        count: 0,
        open: Vec::new(),
    };
    save(&mut deps.storage, CONFIG_KEY, &config)?;
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
        HandleMsg::Propose {
            action,
            description,
        } => try_propose(deps, env, action, description),
        HandleMsg::Vote { id, vote } => try_vote(deps, env, id, vote),
        HandleMsg::CreateViewingKey { entropy } => try_create_key(deps, env, &entropy),
        HandleMsg::SetViewingKey { key, .. } => try_set_key(deps, env, key),
    };
    pad_handle_result(response, BLOCK_SIZE)
}

/// Returns HandleResult
///
/// create a proposal
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `action` - the Action the proposal will execute if it passes
/// * `description` - optional String to supply additional info
pub fn try_propose<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    action: Action,
    description: Option<String>,
) -> HandleResult {
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&sender_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters can create a proposal",
        ));
    }
    let action = match action {
        Action::AddVoter { address } => {
            let address_raw = deps.api.canonical_address(&address)?;
            if config.voters.contains(&address_raw) {
                return Err(StdError::generic_err(format!(
                    "{} is already an authorized voter",
                    address
                )));
            }
            StoredAction::AddVoter {
                address: address_raw,
            }
        }
        Action::RemoveVoter { address } => {
            let address_raw = deps.api.canonical_address(&address)?;
            if !config.voters.contains(&address_raw) {
                return Err(StdError::generic_err(format!(
                    "{} is not currently an authorized voter",
                    address
                )));
            }
            StoredAction::RemoveVoter {
                address: address_raw,
            }
        }
        Action::Withdraw { recipient, amount } => {
            let recipient_raw = deps.api.canonical_address(&recipient)?;
            StoredAction::Withdraw {
                recipient: recipient_raw,
                amount,
            }
        }
    };
    let mut proposal = StoredProposal {
        id: config.count,
        action,
        status: Status::Open,
        description,
        votes: [Vec::new(), Vec::new()],
    };
    config.open.push(config.count);
    config.count += 1;
    let mut state = State {
        config,
        failed: Vec::new(),
        passed: Vec::new(),
        messages: Vec::new(),
        reserve: None,
        withdrawn: Uint128(0),
    };
    let (_it_closed, rmv_voter) =
        process_vote(deps, &env, sender_raw, &mut state, &mut proposal, Vote::Yes)?;
    if let Some(remove) = rmv_voter {
        remove_voter(deps, &env, &remove, &mut state)?;
    }
    save(&mut deps.storage, CONFIG_KEY, &state.config)?;
    Ok(HandleResponse {
        messages: state.messages,
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ClosedProposals {
            passed: state.passed,
            failed: state.failed,
        })?),
    })
}

/// Returns HandleResult
///
/// vote on a proposal
///
/// # Arguments
///
/// * `deps` - mutable reference to Extern containing all the contract's external dependencies
/// * `env` - Env of contract's environment
/// * `id` - ID of the proposal being voted on
/// * `vote` - the Vote being cast
pub fn try_vote<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    id: u32,
    vote: Vote,
) -> HandleResult {
    let sender_raw = deps.api.canonical_address(&env.message.sender)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&sender_raw) {
        return Err(StdError::generic_err("Only authorized voters can vote"));
    }
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &id.to_le_bytes())?;
    if let Some(mut proposal) = may_prop {
        let mut state = State {
            config,
            failed: Vec::new(),
            passed: Vec::new(),
            messages: Vec::new(),
            reserve: None,
            withdrawn: Uint128(0),
        };
        let (it_closed, rmv_voter) =
            process_vote(deps, &env, sender_raw, &mut state, &mut proposal, vote)?;
        if it_closed {
            if let Some(remove) = rmv_voter {
                remove_voter(deps, &env, &remove, &mut state)?;
            }
            save(&mut deps.storage, CONFIG_KEY, &state.config)?;
        }
        return Ok(HandleResponse {
            messages: state.messages,
            log: vec![],
            data: Some(to_binary(&HandleAnswer::ClosedProposals {
                passed: state.passed,
                failed: state.failed,
            })?),
        });
    }
    Err(StdError::generic_err(format!(
        "Proposal ID {} is not valid",
        id
    )))
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
    let sender_raw = &deps.api.canonical_address(&env.message.sender)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&sender_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters need a viewing key",
        ));
    }
    let prng_seed: Vec<u8> = load(&deps.storage, PRNG_KEY)?;
    let key = ViewingKey::new(&env, &prng_seed, entropy.as_ref());
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, sender_raw.as_slice(), &key.to_hashed())?;
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
    let sender_raw = &deps.api.canonical_address(&env.message.sender)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&sender_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters need a viewing key",
        ));
    }
    let vk = ViewingKey(key.clone());
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, sender_raw.as_slice(), &vk.to_hashed())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ViewingKey { key })?),
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
        QueryMsg::ListOpenProposals {
            address,
            viewing_key,
        } => query_open_proposals(deps, &address, viewing_key),
        QueryMsg::ListAllProposals {
            address,
            viewing_key,
            start_at,
            limit,
        } => query_all_proposals(deps, &address, viewing_key, start_at, limit),
        QueryMsg::Proposal {
            id,
            address,
            viewing_key,
        } => query_proposal(deps, id, &address, viewing_key),
        QueryMsg::ListVoters {
            address,
            viewing_key,
        } => query_voters(deps, &address, viewing_key),
        QueryMsg::Vote {
            id,
            voter,
            viewer,
            viewing_key,
        } => query_vote(deps, id, &voter, viewer.as_ref(), viewing_key),
    };
    pad_query_result(response, BLOCK_SIZE)
}

pub fn query_vote<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u32,
    voter: &HumanAddr,
    viewer: Option<&HumanAddr>,
    viewing_key: String,
) -> QueryResult {
    let mut is_viewer = false;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    let voter_raw = deps.api.canonical_address(voter)?;
    let (viewer_raw, vwr_given) = if let Some(vwr) = viewer {
        (deps.api.canonical_address(vwr)?, true)
    } else {
        (CanonicalAddr(Binary::from(b"notused")), false)
    };
    // if a viewer was supplied, they're saying they aren't the voter so check
    // if the key matches the viewer address first
    if vwr_given && check_key(&deps.storage, &viewer_raw, viewing_key.clone()).is_ok() {
        is_viewer = true;
        if !config.voters.contains(&viewer_raw) {
            return Err(StdError::generic_err(
                "Only authorized voters may perform this query",
            ));
        }
    }
    // check if this is the voter's key if we need to
    if !is_viewer {
        check_key(&deps.storage, &voter_raw, viewing_key)?;
        if !config.voters.contains(&voter_raw) {
            return Err(StdError::generic_err(
                "Only authorized voters may perform this query",
            ));
        }
    }
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &id.to_le_bytes())?;
    if let Some(prop) = may_prop {
        let vote = if let Some(_a) = prop.votes[0].iter().find(|&a| *a == voter_raw) {
            Some(Vote::Yes)
        } else if let Some(_a) = prop.votes[1].iter().find(|&a| *a == voter_raw) {
            Some(Vote::No)
        } else {
            None
        };
        return to_binary(&QueryAnswer::Vote { vote });
    }
    Err(StdError::generic_err(format!(
        "Proposal ID {} is not valid",
        id
    )))
}

pub fn query_voters<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&address_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters may perform this query",
        ));
    }
    to_binary(&QueryAnswer::VoterList {
        voters: config
            .voters
            .iter()
            .map(|m| deps.api.human_address(m))
            .collect::<StdResult<Vec<HumanAddr>>>()?,
    })
}

pub fn query_proposal<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u32,
    address: &HumanAddr,
    viewing_key: String,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&address_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters may perform this query",
        ));
    }
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &id.to_le_bytes())?;
    if let Some(prop) = may_prop {
        return to_binary(&QueryAnswer::ProposalInfo {
            proposal: prop.into_humanized(&deps.api)?,
        });
    }
    Err(StdError::generic_err(format!(
        "Proposal ID {} is not valid",
        id
    )))
}

pub fn query_all_proposals<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
    start_at: Option<u32>,
    limit: Option<u32>,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&address_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters may perform this query",
        ));
    }
    let proposals = get_proposals(
        &deps,
        start_at.unwrap_or(config.count - 1),
        limit.unwrap_or(30),
    )?;
    to_binary(&QueryAnswer::ProposalList { proposals })
}

pub fn query_open_proposals<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
) -> QueryResult {
    let address_raw = deps.api.canonical_address(address)?;
    check_key(&deps.storage, &address_raw, viewing_key)?;
    let mut config: Config = load(&deps.storage, CONFIG_KEY)?;
    if !config.voters.contains(&address_raw) {
        return Err(StdError::generic_err(
            "Only authorized voters may perform this query",
        ));
    }
    config.open.sort_unstable();
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    let mut proposals: Vec<Proposal> = Vec::new();
    for idx in config.open.iter().rev() {
        let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &idx.to_le_bytes())?;
        if let Some(prop) = may_prop {
            proposals.push(prop.into_humanized(&deps.api)?);
        }
    }
    to_binary(&QueryAnswer::ProposalList { proposals })
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

pub struct State {
    pub config: Config,
    pub passed: Vec<u32>,
    pub failed: Vec<u32>,
    pub messages: Vec<CosmosMsg>,
    pub reserve: Option<Uint128>,
    pub withdrawn: Uint128,
}

fn process_vote<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    voter: CanonicalAddr,
    state: &mut State,
    proposal: &mut StoredProposal,
    vote: Vote,
) -> StdResult<(bool, Option<CanonicalAddr>)> {
    if let Status::Open = proposal.status {
        let idx: usize = if let Vote::Yes = vote { 0 } else { 1 };
        // if they already voted in the opposite, remove the old vote
        if let Some(pos) = proposal.votes[1 - idx].iter().position(|a| *a == voter) {
            proposal.votes[1 - idx].swap_remove(pos);
        }
        // only add vote if the address didn't already vote this way
        if proposal.votes[idx].contains(&voter) {
            return Err(StdError::generic_err(format!(
                "You have already cast the exact same vote for Proposal {}",
                proposal.id
            )));
        }
        proposal.votes[idx].push(voter);
        return check_close(deps, env, proposal, true, state);
    }
    Err(StdError::generic_err(
        "You can not vote on a proposal that is no longer open",
    ))
}

fn check_close<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    proposal: &mut StoredProposal,
    save_prop: bool,
    state: &mut State,
) -> StdResult<(bool, Option<CanonicalAddr>)> {
    let mut closed_prop = false;
    let mut rmv_voter: Option<CanonicalAddr> = None;
    if let Status::Open = proposal.status {
        let mut save_it = save_prop;
        // check if both yes or no got enough votes to win
        for idx in 0..2 {
            if proposal.votes[idx].len() > state.config.voters.len() / 2 {
                // if yes won, perform the action
                if idx == 0 {
                    match &proposal.action {
                        StoredAction::AddVoter { address } => {
                            if !state.config.voters.contains(&address) {
                                state.config.voters.push(address.clone());
                            }
                        }
                        StoredAction::RemoveVoter { address } => {
                            if let Some(pos) = state.config.voters.iter().position(|a| a == address)
                            {
                                state.config.voters.swap_remove(pos);
                                rmv_voter = Some(address.clone());
                            }
                        }
                        StoredAction::Withdraw { recipient, amount } => {
                            let reserve = if state.reserve.is_none() {
                                let res = deps
                                    .querier
                                    .query_balance(&env.contract.address, "uscrt")?
                                    .amount;
                                state.reserve = Some(res);
                                res
                            } else {
                                state.reserve.unwrap()
                            };
                            if (state.withdrawn + *amount) > reserve {
                                return Err(StdError::generic_err(
                                    format!("Withdrawal Proposal {} passed, but there is not enough SCRT in the reserve", proposal.id),
                                ));
                            }
                            state.withdrawn += *amount;
                            let withdrawal_coins: Vec<Coin> = vec![Coin {
                                denom: "uscrt".to_string(),
                                amount: *amount,
                            }];
                            state.messages.push(CosmosMsg::Bank(BankMsg::Send {
                                from_address: env.contract.address.clone(),
                                to_address: deps.api.human_address(&recipient)?,
                                amount: withdrawal_coins,
                            }));
                        }
                    }
                    proposal.status = Status::Passed;
                    state.passed.push(proposal.id);
                // no won, so mark it as failed
                } else {
                    proposal.status = Status::Failed;
                    state.failed.push(proposal.id);
                }
                // remove proposal from list of open proposals
                if let Some(pos) = state.config.open.iter().position(|i| *i == proposal.id) {
                    state.config.open.swap_remove(pos);
                }
                closed_prop = true;
                save_it = true;
                break;
            }
        }
        if save_it {
            let mut prop_store = PrefixedStorage::new(PREFIX_PROPOSAL, &mut deps.storage);
            json_save(&mut prop_store, &proposal.id.to_le_bytes(), &proposal)?;
        }
    }
    Ok((closed_prop, rmv_voter))
}

fn remove_voter<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    voter: &CanonicalAddr,
    state: &mut State,
) -> StdResult<()> {
    let mut open_props: Vec<StoredProposal> = Vec::new();
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    // create a list of all the open proposals
    for prop_id in state.config.open.iter() {
        let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &prop_id.to_le_bytes())?;
        if let Some(prop) = may_prop {
            open_props.push(prop);
        }
    }
    // remove the voter's votes from all open proposals
    remove_votes(voter, &mut open_props);
    // check through all open proposals to see if removing a voter made any side have
    // enough votes to win with the new lower threshhold
    let mut idx = 0;
    while idx < open_props.len() {
        let (it_closed, rmv_voter) = check_close(deps, env, &mut open_props[idx], false, state)?;
        if it_closed {
            // remove the closed proposal from the list of open proposals
            open_props.swap_remove(idx);
            // if another voter got removed
            if let Some(rmv) = rmv_voter {
                // remove the voter's votes from all open proposals
                remove_votes(&rmv, &mut open_props);
                // now that number of voters changed again, we need to check from the
                // beginning
                idx = 0;
            }
        } else {
            // check next proposal
            idx += 1;
        }
    }
    // save all the updated proposals
    let mut prop_store = PrefixedStorage::new(PREFIX_PROPOSAL, &mut deps.storage);
    for prop in open_props.iter() {
        json_save(&mut prop_store, &prop.id.to_le_bytes(), prop)?;
    }
    Ok(())
}

fn remove_votes(voter: &CanonicalAddr, open_props: &mut Vec<StoredProposal>) {
    for prop in open_props.iter_mut() {
        if let Some(pos) = prop.votes[0].iter().position(|a| a == voter) {
            prop.votes[0].swap_remove(pos);
        } else if let Some(pos) = prop.votes[1].iter().position(|a| a == voter) {
            prop.votes[1].swap_remove(pos);
        }
    }
}
