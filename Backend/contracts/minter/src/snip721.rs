use crate::contract::BLOCK_SIZE;
use cosmwasm_std::HumanAddr;
use schemars::JsonSchema;
use secret_toolkit::{
    snip721::{Metadata, ViewerInfo},
    utils::Query,
};
use serde::{Deserialize, Serialize};

/// snip721 query msgs
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Snip721QueryMsg {
    /// displays all the public information about a token
    NftDossier {
        token_id: String,
        viewer: ViewerInfo,
    },
}

impl Query for Snip721QueryMsg {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

/// NftDossier answer stripped down to the fields of interest
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct NftDossierForMinter {
    /// optional owner of the token
    pub owner: Option<HumanAddr>,
    /// optional private metadata of the token
    pub private_metadata: Option<Metadata>,
}

/// wrapper to deserialize NftDossier responses
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct NftDossierResponse {
    pub nft_dossier: NftDossierForMinter,
}
