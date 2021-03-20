use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub entropy: String,
    pub card_contract_label: String,
    pub card_contract: ContractInitInfo,
}

/// Handle messages
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Mint {
        names: Vec<String>,
    },
    RegisterCardContract {},
    NewCardContract {
        label: String,
        entropy: String,
        card_contract: ContractInitInfo,
    },
    SetStatus {
        stop: bool,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    NotUsed {},
}

/// success or failure response
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ResponseStatus {
    Success,
    Failure,
}

/// Responses from handle functions
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    Status { status: ResponseStatus },
}

/// Info needed to instantiate the token contract
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ContractInitInfo {
    pub name: String,
    pub symbol: String,
    pub code_id: u64,
    pub code_hash: String,
}
