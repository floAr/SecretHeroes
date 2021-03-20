use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr};

use crate::state::Battle;

/// Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub entropy: String,
    pub card_contract: ContractInfo,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ContractInfo {
    pub code_hash: String,
    pub address: HumanAddr,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateViewingKey {
        entropy: String,
    },
    SetViewingKey {
        key: String,
        padding: Option<String>,
    },
    ReceiveNft {
        sender: HumanAddr,
        from: HumanAddr,
        token_id: String,
        msg: Option<Binary>,
    },
    ChickenOut {},
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    ViewingKey { key: String },
    ChickenOut { message: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Bullpen {
        address: HumanAddr,
        viewing_key: String,
    },
    BattleHistory {
        address: HumanAddr,
        viewing_key: String,
        page: Option<u32>,
        page_size: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct WaitingHero {
    pub token_id: String,
    pub name: String,
    pub skills: Vec<u8>,
}

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
