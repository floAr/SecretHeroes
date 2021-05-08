use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::HumanAddr;

/// Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    /// prng entropy
    pub entropy: String,
    /// card contract info
    pub card_contract: ContractInfo,
    /// address of the multisig contract
    pub multi_sig: HumanAddr,
}

/// Handle messages
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// mint a pack of 3 cards
    Mint {
        /// names to give the cards.  Must provide 3 names
        names: Vec<String>,
    },
    /// change address with administrative power
    ChangeAdmin {
        /// address with admin authority
        address: HumanAddr,
    },
    /// change the ContractInfo of the cards
    NewCardContract {
        /// new card ContractInfo
        card_contract: ContractInfo,
    },
    /// change the address of the multi sig contract
    NewMultiSig {
        /// new multi sig contract address
        address: HumanAddr,
    },
    /// halt/start minting
    SetMintStatus {
        /// true if minting should be halted
        stop: bool,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// display the minter config
    Config {},
    /// display the number of packs minted
    PacksMinted {},
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
    Mint { status: ResponseStatus },
    ChangeAdmin { new_admin: HumanAddr },
    NewCardContract { card_contract: HumanAddr },
    NewMultiSig { multi_sig: HumanAddr },
    SetMintStatus { minting_has_halted: bool },
}

/// Responses from queries
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    /// minter config
    Config {
        card_versions: Vec<ContractInfo>,
        multi_sig_contract: HumanAddr,
        minting_has_halted: bool,
    },
    /// number of packs minted
    PacksMinted { packs_minted: u32 },
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: HumanAddr,
}
