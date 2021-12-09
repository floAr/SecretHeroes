use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{Api, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use secret_toolkit::serialization::{Bincode2, Serde};

use crate::msg::ContractInfo;

pub const CONFIG_KEY: &[u8] = b"config";
pub const ADMIN_KEY: &[u8] = b"admin";

/// minter state
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// card contract versions info
    pub card_versions: Vec<StoreContractInfo>,
    /// true if minting should be halted
    pub minting_halt: bool,
    /// true if tournaments have been halted
    //    pub tourney_halt: bool,
    /// multi sig contract address
    pub multi_sig: CanonicalAddr,
    /// tournament contract address
    //    pub tourney: CanonicalAddr,
    /// prng seed
    pub prng_seed: Vec<u8>,
    /// number of packs minted
    pub mint_cnt: u32,
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
