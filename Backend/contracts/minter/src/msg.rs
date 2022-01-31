use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract_info::ContractInfo;
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
    /// add compatible card contracts without changing the current one used for minting
    AddLegacyCardContracts {
        /// legacy card contracts
        card_contracts: Vec<ContractInfo>,
    },
    /// change the address of the multi sig contract
    NewMultiSig {
        /// new multi sig contract address
        address: HumanAddr,
    },
    /// halt/start minting and/or upgrading
    SetMintAndUpgradeStatus {
        /// true if minting should be halted
        stop_mint: Option<bool>,
        /// true if upgrades should be halted
        stop_upgrade: Option<bool>,
    },
    /// add number of packs minted (admin only)
    AddMintCount {
        /// number of packs minted in previous contracts
        packs_minted: u32,
    },
    /// burn two heroes to upgrade a third
    Upgrade {
        /// heroes to burn
        burn: Vec<HeroInfo>,
        /// hero to upgrade
        upgrade: HeroInfo,
        /// entropy for the rng
        entropy: String,
    },
}

/// the token ID and contract address of a Hero
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct HeroInfo {
    /// the hero's token ID
    pub token_id: String,
    /// the SNIP-721 contract that controls this Hero
    pub contract_address: HumanAddr,
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
    Mint {
        status: ResponseStatus,
    },
    ChangeAdmin {
        new_admin: HumanAddr,
    },
    NewCardContract {
        card_contract: HumanAddr,
    },
    NewMultiSig {
        multi_sig: HumanAddr,
    },
    SetMintAndUpgradeStatus {
        minting_has_halted: bool,
        upgrades_have_halted: bool,
    },
    AddMintCount {
        packs_added: u32,
    },
    AddLegacyCardContracts {
        card_versions: Vec<ContractInfo>,
    },
    Upgrade {
        /// hero's skills before the upgrade
        pre_upgrade_skills: Vec<u8>,
        /// hero's skills after the upgrade
        post_upgrade_skills: Vec<u8>,
    },
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
        upgrades_have_halted: bool,
    },
    /// number of packs minted
    PacksMinted { packs_minted: u32 },
}
