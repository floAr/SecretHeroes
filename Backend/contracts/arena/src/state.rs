use std::any::type_name;

use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::{
    serialization::{Bincode2, Serde},
    storage::{AppendStore, AppendStoreMut},
};

pub const CONFIG_KEY: &[u8] = b"config";
pub const CONTRACT_KEY: &[u8] = b"contract";
pub const PREFIX_VIEW_KEY: &[u8] = b"viewkey";
pub const PREFIX_HISTORY: &[u8] = b"history";
pub const PREFIX_BATTLE_ID: &[u8] = b"battleids";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub heroes: Vec<Hero>,
    pub prng_seed: Vec<u8>,
    pub entropy: String,
    pub battle_cnt: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hero {
    pub owner: CanonicalAddr,
    pub token_id: String,
    pub name: String,
    pub skills: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct CardContract {
    pub address: CanonicalAddr,
    pub code_hash: String,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Battle {
    pub battle_number: u64,
    pub my_hero: String,
    pub my_token_id: String,
    pub my_skills: Vec<u8>,
    pub skill_used: u8,
    pub winning_skill_value: u8,
    pub i_won: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoredBattle {
    pub battle_number: u64,
    pub heroes: Vec<Hero>,
    pub skill_used: u8,
    pub winner: Option<String>,
    pub winning_skill_value: u8,
}

impl StoredBattle {
    pub fn into_humanized(self, address: &CanonicalAddr) -> StdResult<Battle> {
        if let Some(mine) = self.heroes.iter().find(|h| h.owner == *address) {
            let battle = Battle {
                battle_number: self.battle_number,
                my_hero: mine.name.clone(),
                my_token_id: mine.token_id.clone(),
                my_skills: mine.skills.clone(),
                skill_used: self.skill_used,
                winning_skill_value: self.winning_skill_value,
                i_won: matches!(self.winner, Some(h) if h == mine.token_id),
            };
            Ok(battle)
        } else {
            Err(StdError::generic_err("Battle History corupted"))
        }
    }
}

pub fn append_battle<S: Storage>(storage: &mut S, battle: &StoredBattle) -> StdResult<()> {
    let mut store = PrefixedStorage::new(PREFIX_HISTORY, storage);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;
    store.push(battle)
}

pub fn append_battle_for_addr<S: Storage>(
    storage: &mut S,
    battle_num: u64,
    address: &CanonicalAddr,
) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_BATTLE_ID, address.as_slice()], storage);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;
    store.push(&battle_num)
}

pub fn get_history<S: ReadonlyStorage>(
    storage: &S,
    address: &CanonicalAddr,
    page: u32,
    page_size: u32,
) -> StdResult<Vec<Battle>> {
    let id_store =
        ReadonlyPrefixedStorage::multilevel(&[PREFIX_BATTLE_ID, address.as_slice()], storage);

    let id_store = if let Some(result) = AppendStore::<u32, _>::attach(&id_store) {
        result?
    } else {
        return Ok(vec![]);
    };
    let his_store = ReadonlyPrefixedStorage::new(PREFIX_HISTORY, storage);
    let his_store = AppendStore::<StoredBattle, _, _>::attach(&his_store)
        .ok_or_else(|| StdError::generic_err("Unable to retrieve battle history"))??;

    let battles: StdResult<Vec<Battle>> = id_store
        .iter()
        .rev()
        .skip((page * page_size) as usize)
        .take(page_size as usize)
        .map(|id| {
            id.map(|id| his_store.get_at(id).and_then(|b| b.into_humanized(address)))
                .and_then(|x| x)
        })
        .collect();

    battles
}

pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
