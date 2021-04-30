use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{Api, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::{
    serialization::{Bincode2, Serde},
    storage::{AppendStore, AppendStoreMut},
};

use crate::msg::{Battle, ContractInfo, Hero, TokenInfo};
use crate::stats::Stats;

pub const CONFIG_KEY: &[u8] = b"config";
pub const PREFIX_VIEW_KEY: &[u8] = b"viewkey";
pub const PREFIX_HISTORY: &[u8] = b"history";
pub const PREFIX_BATTLE_ID: &[u8] = b"battleids";
pub const ADMIN_KEY: &[u8] = b"admin";

/// arena config
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// heroes waiting to fight
    pub heroes: Vec<StoreWaitingHero>,
    /// prng seed
    pub prng_seed: Vec<u8>,
    /// combined entropy strings supplied with the heroes
    pub entropy: String,
    /// current battle count
    pub battle_cnt: u64,
    /// viewing key used with the card contracts
    pub viewing_key: String,
    /// contract info of all the card versions
    pub card_versions: Vec<StoreContractInfo>,
    /// true if battles are halted
    pub fight_halt: bool,
}

/// waiting hero's info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreWaitingHero {
    /// hero's owner
    pub owner: CanonicalAddr,
    /// name of the hero
    pub name: String,
    /// hero's token info
    pub token_info: StoreTokenInfo,
    /// hero's stats
    pub stats: Stats,
}

/// hero info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreHero {
    /// hero's owner
    pub owner: CanonicalAddr,
    /// name of the hero
    pub name: String,
    /// hero's token info
    pub token_info: StoreTokenInfo,
    /// hero's skills before the battle
    pub pre_battle_skills: Vec<u8>,
    /// hero's skills after the battle
    pub post_battle_skills: Vec<u8>,
}

impl StoreHero {
    /// Returns StdResult<Hero> from converting a StoreHero to a displayable Hero
    ///
    /// # Arguments
    ///
    /// * `versions` - a slice of ContractInfo of token contract versions
    pub fn into_humanized(self, versions: &[ContractInfo]) -> StdResult<Hero> {
        let hero = Hero {
            name: self.name,
            token_info: TokenInfo {
                token_id: self.token_info.token_id,
                address: versions[self.token_info.version as usize].address.clone(),
            },
            pre_battle_skills: self.pre_battle_skills,
            post_battle_skills: self.post_battle_skills,
        };

        Ok(hero)
    }
}

/// a hero's token info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreTokenInfo {
    /// hero's token id
    pub token_id: String,
    /// index of the card contract version
    pub version: u8,
}

/// battle info
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreBattle {
    /// battle id number
    pub battle_number: u64,
    /// number of seconds since epoch time 01/01/1970 in which the battle took place
    pub timestamp: u64,
    /// heroes that fought
    pub heroes: Vec<StoreHero>,
    /// skill used to determine the winner
    pub skill_used: u8,
    /// index of winning hero
    pub winner: Option<u8>,
    /// winning skill value
    pub winning_skill_value: u8,
}

impl StoreBattle {
    /// Returns StdResult<Battle> from converting a StoreBattle to a displayable Battle
    ///
    /// # Arguments
    ///
    /// * `address` - a reference to the address querying their battle history
    /// * `versions` - a slice of ContractInfo of token contract versions
    pub fn into_humanized(
        mut self,
        address: &CanonicalAddr,
        versions: &[ContractInfo],
    ) -> StdResult<Battle> {
        if let Some(pos) = self.heroes.iter().position(|h| h.owner == *address) {
            let battle = Battle {
                battle_number: self.battle_number,
                timestamp: self.timestamp,
                my_hero: self.heroes.swap_remove(pos).into_humanized(versions)?,
                skill_used: self.skill_used,
                winning_skill_value: self.winning_skill_value,
                i_won: self.winner.map_or_else(|| false, |w| w as usize == pos),
            };
            Ok(battle)
        } else {
            Err(StdError::generic_err("Battle History corupted"))
        }
    }
}
/// Returns StdResult<()> after saving the battle id
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `battle_num` - the battle id to store
/// * `address` - a reference to the address for which to store this battle id
pub fn append_battle_for_addr<S: Storage>(
    storage: &mut S,
    battle_num: u64,
    address: &CanonicalAddr,
) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_BATTLE_ID, address.as_slice()], storage);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;
    store.push(&battle_num)
}

/// Returns StdResult<Vec<Battle>> of the battles to display
///
/// # Arguments
///
/// * `api` - a reference to the Api used to convert human and canonical addresses
/// * `storage` - a reference to the contract's storage
/// * `address` - a reference to the address whose battles to display
/// * `page` - page to start displaying
/// * `page_size` - number of txs per page
pub fn get_history<A: Api, S: ReadonlyStorage>(
    api: &A,
    storage: &S,
    address: &CanonicalAddr,
    page: u32,
    page_size: u32,
) -> StdResult<Vec<Battle>> {
    let id_store =
        ReadonlyPrefixedStorage::multilevel(&[PREFIX_BATTLE_ID, address.as_slice()], storage);
    // Try to access the storage of battle ids for the account.
    // If it doesn't exist yet, return an empty list of battles.
    let id_store = if let Some(result) = AppendStore::<u64, _>::attach(&id_store) {
        result?
    } else {
        return Ok(vec![]);
    };
    let config: Config = load(storage, CONFIG_KEY)?;
    let versions = config
        .card_versions
        .iter()
        .map(|v| v.to_humanized(api))
        .collect::<StdResult<Vec<ContractInfo>>>()?;
    // access battle storage
    let his_store = ReadonlyPrefixedStorage::new(PREFIX_HISTORY, storage);
    // Take `page_size` battles starting from the latest battle, potentially skipping `page * page_size`
    // battles from the start.
    let battles: StdResult<Vec<Battle>> = id_store
        .iter()
        .rev()
        .skip((page * page_size) as usize)
        .take(page_size as usize)
        .map(|id| {
            id.map(|id| {
                load(&his_store, &id.to_le_bytes())
                    .and_then(|b: StoreBattle| b.into_humanized(address, &versions))
            })
            .and_then(|x| x)
        })
        .collect();

    battles
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, Clone)]
pub struct StoreContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: CanonicalAddr,
}

impl StoreContractInfo {
    /// Returns StdResult<ContractInfo> from converting a StoreContractInfo to a displayable
    /// ContractInfo
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    pub fn to_humanized<A: Api>(&self, api: &A) -> StdResult<ContractInfo> {
        let info = ContractInfo {
            address: api.human_address(&self.address)?,
            code_hash: self.code_hash.clone(),
        };
        Ok(info)
    }
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
