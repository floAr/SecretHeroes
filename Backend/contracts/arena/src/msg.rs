use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr};

use crate::stats::Stats;

/// Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    /// entropy for prng
    pub entropy: String,
    /// card ContractInfo
    pub card_contract: ContractInfo,
}

/// Handle messages
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// create a viewing key
    CreateViewingKey {
        /// entropy String used in random key generation
        entropy: String,
    },
    /// set viewing key
    SetViewingKey {
        /// desired viewing key
        key: String,
        /// optional message length padding
        padding: Option<String>,
    },
    /// NFT receiver interface
    BatchReceiveNft {
        /// address that sent the NFTs
        sender: HumanAddr,
        /// previous owner of the NFTs
        from: HumanAddr,
        /// list of NFTs sent from the previous owner
        token_ids: Vec<String>,
        /// msg specified when sending
        msg: Option<Binary>,
    },
    /// withdraw hero from the arena waiting room (bullpen)
    ChickenOut {},
    /// change address with administrative power
    ChangeAdmin {
        /// address with admin authority
        address: HumanAddr,
    },
    /// halt/start battles
    SetBattleStatus {
        /// true if battles should be halted
        stop: bool,
    },
    /// add a compatible card contract
    AddCardContract {
        /// new card ContractInfo
        card_contract: ContractInfo,
    },
    /// set export_to contract info
    SetExportToContract {
        /// new arena ContractInfo
        new_arena: ContractInfo,
    },
    /// set the address of an old arena contract that is allowed to export player stats
    SetImportFromAddress {
        /// old arena contract address
        old_arena: HumanAddr,
    },
    /// import player stats and battle count.  This can only be called by the authorized old arena
    Import {
        stats: Vec<PlayerStats>,
        battle_count: Option<u64>,
    },
    /// export player stats to a new arena.  This will continue with the next block of an on-going export
    /// process.
    Export {},
    /// add bot addresses
    AddBots {
        /// list of addresses that auto-send fighters to shorten user wait
        bots: Vec<HumanAddr>,
    },
    /// remove bot addresses
    RemoveBots {
        /// list of addresses that no longer auto-send fighters
        bots: Vec<HumanAddr>,
    },
    /// reset the tournament leaderboard
    ResetLeaderboard {},
}

/// Responses from handle functions
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    /// response from setting or creating a viewing key
    ViewingKey { key: String },
    /// response from withdrawing a hero from the bullpen
    ChickenOut { message: String },
    /// response from changing the admin address
    ChangeAdmin { new_admin: HumanAddr },
    /// response from starting/stopping battles
    SetBattleStatus { battles_have_halted: bool },
    /// response from adding a new card contract
    AddCardContract { card_contract: HumanAddr },
    /// response from adding auto-send addresses
    AddBots { added_bots: Vec<HumanAddr> },
    /// response from removing auto-send addresses
    RemoveBots { removed_bots: Vec<HumanAddr> },
    /// response from resetting the tournament leaderboard
    ResetLeaderboard { timestamp: u64 },
    /// response from setting an old arena contract allowed to export player stats
    SetImportFromAddress { old_arena: HumanAddr },
    /// response from importing player stats
    Import { successful: bool },
    /// response from exporting player stats
    Export { completed: bool },
    /// response from setting a new arena contract to export to
    SetExportToContract { new_arena: HumanAddr },
}

/// Query messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// display the number of heroes waiting to battle, and the stats of
    /// the querier's hero if applicable
    Bullpen {
        /// querier's address
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
    },
    /// display the querier's battle history
    BattleHistory {
        /// querier's address
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
        /// optional page to display
        page: Option<u32>,
        /// optional number of battles to display
        page_size: Option<u32>,
    },
    /// display the arena config
    Config {},
    /// display player stats export status
    ExportStatus {
        /// admin's address
        admin: HumanAddr,
        /// admin's viewing key
        viewing_key: String,
    },
    /// display game usage metrics
    Usage {},
    /// display list of auto-send addresses
    Bots {},
    /// display the leaderboards
    Leaderboards {},
    /// display tournament info
    Tournament {},
    /// display a player's stats
    PlayerStats {
        /// querier's address
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
    },
    /// admin dump of all players' all-time stats
    DumpPlayerStats {
        /// admin's address
        admin: HumanAddr,
        /// admin's viewing key
        viewing_key: String,
        /// optional index of player to start display.  Use this for pagination
        start_from: Option<u32>,
        /// optional number of players' stats to display
        limit: Option<u32>,
    },
    /// admin dump history of all battles
    DumpBattleHistory {
        /// admin's address
        admin: HumanAddr,
        /// admin's viewing key
        viewing_key: String,
        /// optional index of battle to start display.  Use this for pagination
        start_from: Option<u64>,
        /// optional number of battles to display
        limit: Option<u64>,
    },
}

/// responses from queries
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Bullpen {
        heroes_waiting: u8,
        your_hero: Option<WaitingHero>,
    },
    BattleHistory {
        history: Vec<Battle>,
    },
    /// arena config
    Config {
        card_versions: Vec<ContractInfo>,
        battles_have_halted: bool,
    },
    /// list of auto-send addresses
    Bots {
        bots: Vec<HumanAddr>,
    },
    /// point leaderboards
    Leaderboards {
        /// seconds after 01/01/1970 in which the tournament started
        tournament_started: u64,
        /// tournament leaderboard
        tournament: Vec<PlayerStats>,
        /// all time leaderboard
        all_time: Vec<PlayerStats>,
    },
    /// player's stats
    PlayerStats {
        /// tournament stats
        tournament: PlayerStats,
        /// all time stats
        all_time: PlayerStats,
    },
    /// display tournament info
    Tournament {
        /// seconds after 01/01/1970 in which the tournament started
        tournament_started: u64,
        /// the tournament leaderboard
        leaderboard: Vec<PlayerStats>,
    },
    /// game usage metrics
    Usage {
        /// number of players
        player_count: u32,
        /// number of battles that occurred in this arena
        arena_battle_count: u64,
        /// number of battles in previous arenas
        previous_arena_battles: u64,
    },
    /// status of player stats export
    ExportStatus {
        next_block: Option<u32>,
        last_block: Option<u32>,
    },
    /// all players' all-time stats
    DumpPlayerStats {
        /// list of players' stats and indexes
        stats: Vec<PlayerDump>,
    },
    /// all battle histories for this arena
    DumpBattleHistory {
        history: Vec<BattleDump>,
    },
}

/// info of hero waiting to fight
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct WaitingHero {
    /// name of the hero
    pub name: String,
    /// hero's token info
    pub token_info: TokenInfo,
    /// hero's stats
    pub stats: Stats,
}

/// battle info
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Battle {
    /// battle id number
    pub battle_number: u64,
    /// number of seconds since epoch time 01/01/1970 in which the battle took place
    pub timestamp: u64,
    /// querier's hero in the battle
    pub my_hero: Hero,
    /// skill used to determine battle results
    pub skill_used: u8,
    /// winning hero's name, if any
    pub winner: Option<String>,
    /// the skill value that won the battle
    pub winning_skill_value: u8,
    /// true if the querier's hero won the battle
    pub i_won: bool,
}

/// token info
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct TokenInfo {
    /// id of the token
    pub token_id: String,
    /// address of the token contract that controls the token
    pub address: HumanAddr,
}

/// hero info
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Hero {
    /// hero's name
    pub name: String,
    /// token info
    pub token_info: TokenInfo,
    /// hero's skills before the battle
    pub pre_battle_skills: Vec<u8>,
    /// hero's skills after the battle
    pub post_battle_skills: Vec<u8>,
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: HumanAddr,
}

/// player stats and point leaderboard entry
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct PlayerStats {
    /// player's score
    pub score: i32,
    /// player's address
    pub address: HumanAddr,
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

/// player stats coupled with the player index for better pagination
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct PlayerDump {
    /// index of this player
    pub index: u32,
    /// player's stats
    pub stats: PlayerStats,
}

/// hero info with owner
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct HeroDump {
    /// hero's owner
    pub owner: HumanAddr,
    /// name of the hero
    pub name: String,
    /// hero's token info
    pub token_info: TokenInfo,
    /// hero's skills before the battle
    pub pre_battle_skills: Vec<u8>,
    /// hero's skills after the battle
    pub post_battle_skills: Vec<u8>,
}

/// battle info with index
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct BattleDump {
    /// battle id number
    pub battle_number: u64,
    /// number of seconds since epoch time 01/01/1970 in which the battle took place
    pub timestamp: u64,
    /// heroes that fought
    pub heroes: Vec<HeroDump>,
    /// skill used to determine the winner
    pub skill_used: u8,
    /// index of winning hero
    pub winner: Option<u8>,
    /// winning skill value
    pub winning_skill_value: u8,
}
