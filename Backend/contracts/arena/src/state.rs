use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{Api, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::{
    serialization::{Bincode2, Serde},
    storage::{AppendStore, AppendStoreMut},
};

use crate::msg::{Battle, BattleDump, ContractInfo, Hero, HeroDump, PlayerStats, TokenInfo};
use crate::stats::Stats;

pub const CONFIG_KEY: &[u8] = b"config";
pub const PREFIX_VIEW_KEY: &[u8] = b"viewkey";
pub const PREFIX_HISTORY: &[u8] = b"history";
pub const PREFIX_BATTLE_ID: &[u8] = b"battleids";
pub const PREFIX_TOURN_STATS: &[u8] = b"trnstat";
pub const PREFIX_ALL_STATS: &[u8] = b"allstat";
pub const PREFIX_PLAYERS: &[u8] = b"players";
pub const PREFIX_SEEN: &[u8] = b"seen";
pub const ADMIN_KEY: &[u8] = b"admin";
pub const BOTS_KEY: &[u8] = b"bots";
pub const LEADERBOARDS_KEY: &[u8] = b"ldrbds";
pub const IMPORT_FROM_KEY: &[u8] = b"import";
pub const EXPORT_CONFIG_KEY: &[u8] = b"export";

/// arena config
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// heroes waiting to fight
    pub heroes: Vec<StoreWaitingHero>,
    /// prng seed
    pub prng_seed: Vec<u8>,
    /// combined entropy strings supplied with the heroes
    pub entropy: String,
    /// current battle count in this arena
    pub battle_cnt: u64,
    /// battle count from previous arenas
    pub previous_battles: u64,
    /// viewing key used with the card contracts
    pub viewing_key: String,
    /// contract info of all the card versions
    pub card_versions: Vec<StoreContractInfo>,
    /// true if battles are halted
    pub fight_halt: bool,
    /// total number of players
    pub player_cnt: u32,
    /// list of new players that need to be added
    pub new_players: Vec<CanonicalAddr>,
}

/// export config
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportConfig {
    /// new arena contract info
    pub new_arena: StoreContractInfo,
    /// next block to export
    pub next: u32,
}

/// stored leaderboard entry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rank {
    /// player's score
    pub score: i32,
    /// player's address
    pub address: CanonicalAddr,
}

/// tournament data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tourney {
    /// tournament start time
    pub start: u64,
    /// tournament leaderboard
    pub leaderboard: Vec<Rank>,
}

/// leaderboards
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Leaderboards {
    /// tournament leaderboard
    pub tourney: Tourney,
    /// all time leaderboard
    pub all_time: Vec<Rank>,
}

/// tournament stats
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TourneyStats {
    /// time of last update
    pub last_seen: u64,
    /// player's stats for this tournament
    pub stats: StorePlayerStats,
}

/// stored player stats
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StorePlayerStats {
    /// player's score
    pub score: i32,
    /// number of battles
    pub battles: u32,
    /// number of wins
    pub wins: u32,
    /// number of ties
    pub ties: u32,
    /// number of times took 3rd place in a 2-way tie
    pub third_in_two_way_ties: u32,
    /// number of losses
    pub losses: u32,
}

impl Default for StorePlayerStats {
    fn default() -> Self {
        Self {
            score: 0,
            battles: 0,
            wins: 0,
            ties: 0,
            third_in_two_way_ties: 0,
            losses: 0,
        }
    }
}

impl StorePlayerStats {
    /// Returns StdResult<PlayerStats> from converting a StorePlayerStats to a displayable
    /// PlayerStats
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    /// * `address` - a reference to the address corresponding to these stats
    pub fn into_humanized<A: Api>(
        self,
        api: &A,
        address: &CanonicalAddr,
    ) -> StdResult<PlayerStats> {
        let stats = PlayerStats {
            score: self.score,
            address: api.human_address(address)?,
            battles: self.battles,
            wins: self.wins,
            ties: self.ties,
            third_in_two_way_ties: self.third_in_two_way_ties,
            losses: self.losses,
        };
        Ok(stats)
    }
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

    /// Returns StdResult<HeroDump> from converting a StoreHero to a displayable HeroDump
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    /// * `versions` - a slice of ContractInfo of token contract versions
    pub fn into_dump<A: Api>(self, api: &A, versions: &[ContractInfo]) -> StdResult<HeroDump> {
        let hero = HeroDump {
            owner: api.human_address(&self.owner)?,
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
            let winner = self.winner.map(|u| self.heroes[u as usize].name.clone());
            let battle = Battle {
                battle_number: self.battle_number,
                timestamp: self.timestamp,
                my_hero: self.heroes.swap_remove(pos).into_humanized(versions)?,
                skill_used: self.skill_used,
                winner,
                winning_skill_value: self.winning_skill_value,
                i_won: self.winner.map_or_else(|| false, |w| w as usize == pos),
            };
            Ok(battle)
        } else {
            Err(StdError::generic_err("Battle History corupted"))
        }
    }

    /// Returns StdResult<BattleDump> from converting a StoreBattle to a displayable BattleDump
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    /// * `versions` - a slice of ContractInfo of token contract versions
    pub fn into_dump<A: Api>(
        mut self,
        api: &A,
        versions: &[ContractInfo],
    ) -> StdResult<BattleDump> {
        let battle = BattleDump {
            battle_number: self.battle_number,
            timestamp: self.timestamp,
            heroes: self
                .heroes
                .drain(..)
                .map(|h| h.into_dump(api, versions))
                .collect::<StdResult<Vec<HeroDump>>>()?,
            skill_used: self.skill_used,
            winner: self.winner,
            winning_skill_value: self.winning_skill_value,
        };
        Ok(battle)
    }
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, Clone, Debug)]
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
