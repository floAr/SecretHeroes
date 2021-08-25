use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};

use crate::contract_info::StoreContractInfo;
use secret_toolkit::serialization::{Bincode2, Serde};

pub const CONFIG_KEY: &[u8] = b"config";
pub const ADMIN_KEY: &[u8] = b"admin";
pub const VKEY_KEY: &[u8] = b"vkey";

/// minter state
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// card contract versions info
    pub card_versions: Vec<StoreContractInfo>,
    /// true if minting should be halted
    pub minting_halt: bool,
    /// true if upgrades should be halted
    pub upgrade_halt: bool,
    // true if tournaments have been halted
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
