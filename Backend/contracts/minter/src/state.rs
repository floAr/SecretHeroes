use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use secret_toolkit::serialization::{Bincode2, Serde};

pub const CONFIG_KEY: &[u8] = b"config";
pub const ADMIN_INFO_KEY: &[u8] = b"admin";

/// grouping the data primarily used when minting cards
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub card_contract: ContractInfo,
    pub stopped: bool,
    pub prng_seed: Vec<u8>,
}

/// code hash and address of a contract
#[derive(Serialize, Deserialize)]
pub struct ContractInfo {
    pub code_hash: String,
    pub address: CanonicalAddr,
}

/// info used for admin functions
#[derive(Serialize, Deserialize)]
pub struct AdminInfo {
    pub admin: CanonicalAddr,
    pub expect_reg: bool,
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
