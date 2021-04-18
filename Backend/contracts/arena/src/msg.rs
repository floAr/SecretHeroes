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
}

/// battle info
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Battle {
    /// batlle id number
    pub battle_number: u64,
    /// querier's hero in the battle
    pub my_hero: Hero,
    /// skill used to determine battle results
    pub skill_used: u8,
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
