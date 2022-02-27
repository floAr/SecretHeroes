use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, StdResult};

/// code hash and address of a secret contract
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: HumanAddr,
}

impl ContractInfo {
    /// Returns StdResult<StoreContractInfo> from creating a StoreContractInfo from a
    /// ContractInfo
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    pub fn get_store<A: Api>(&self, api: &A) -> StdResult<StoreContractInfo> {
        Ok(StoreContractInfo {
            code_hash: self.code_hash.clone(),
            address: api.canonical_address(&self.address)?,
        })
    }

    /// Returns StdResult<StoreContractInfo> from converting a ContractInfo to a
    /// StoreContractInfo
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    pub fn into_store<A: Api>(self, api: &A) -> StdResult<StoreContractInfo> {
        Ok(StoreContractInfo {
            code_hash: self.code_hash,
            address: api.canonical_address(&self.address)?,
        })
    }
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StoreContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: CanonicalAddr,
}

impl StoreContractInfo {
    /// Returns StdResult<ContractInfo> from creating a displayable ContractInfo from
    /// a StoreContractInfo
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    pub fn get_humanized<A: Api>(&self, api: &A) -> StdResult<ContractInfo> {
        Ok(ContractInfo {
            code_hash: self.code_hash.clone(),
            address: api.human_address(&self.address)?,
        })
    }

    /// Returns StdResult<ContractInfo> from converting a StoreContractInfo to a
    /// displayable ContractInfo
    ///
    /// # Arguments
    ///
    /// * `api` - a reference to the Api used to convert human and canonical addresses
    pub fn into_humanized<A: Api>(self, api: &A) -> StdResult<ContractInfo> {
        Ok(ContractInfo {
            code_hash: self.code_hash,
            address: api.human_address(&self.address)?,
        })
    }
}
