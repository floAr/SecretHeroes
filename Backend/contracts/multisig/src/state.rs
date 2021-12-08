use std::any::type_name;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{
    Api, CanonicalAddr, Extern, Querier, ReadonlyStorage, StdError, StdResult, Storage,
};

use cosmwasm_storage::ReadonlyPrefixedStorage;

use crate::proposal::{Proposal, StoredProposal};

use secret_toolkit::serialization::{Bincode2, Json, Serde};

/// key to store the Config
pub const CONFIG_KEY: &[u8] = b"config";
/// key to store the prng seed
pub const PRNG_KEY: &[u8] = b"prngseed";
/// prefix for viewing key storage
pub const PREFIX_VIEW_KEY: &[u8] = b"viewkey";
/// prefix for proposal storage
pub const PREFIX_PROPOSAL: &[u8] = b"proposal";

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// list of authorized voters
    pub voters: Vec<CanonicalAddr>,
    /// number of proposals
    pub count: u32,
    /// list of open proposals
    pub open: Vec<u32>,
}

/// Returns StdResult<Vec<Proposal>> of the proposals to display
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
/// * `start_at` - first proposal to display
/// * `limit` - number of proposals to display
pub fn get_proposals<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    start_at: u32,
    limit: u32,
) -> StdResult<Vec<Proposal>> {
    let prop_store = ReadonlyPrefixedStorage::new(PREFIX_PROPOSAL, &deps.storage);
    let end_range = start_at + 1;
    let start_range = end_range.saturating_sub(limit);
    let mut proposals: Vec<Proposal> = Vec::new();
    for idx in (start_range..end_range).rev() {
        let may_prop: Option<StoredProposal> = json_may_load(&prop_store, &idx.to_le_bytes())?;
        if let Some(prop) = may_prop {
            proposals.push(prop.into_humanized(&deps.api)?);
        }
    }
    Ok(proposals)
}

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

/// Removes an item from storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

/// Returns StdResult<T> from retrieving the item with the specified key.  Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

/// Returns StdResult<Option<T>> from retrieving the item with the specified key.
/// Returns Ok(None) if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}

/// Returns StdResult<()> resulting from saving an item to storage using Json (de)serialization
/// because bincode2 annoyingly uses a float op when deserializing an enum
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn json_save<T: Serialize, S: Storage>(
    storage: &mut S,
    key: &[u8],
    value: &T,
) -> StdResult<()> {
    storage.set(key, &Json::serialize(value)?);
    Ok(())
}

/// Returns StdResult<T> from retrieving the item with the specified key using Json
/// (de)serialization because bincode2 annoyingly uses a float op when deserializing an enum.  
/// Returns a StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn json_load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Json::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

/// Returns StdResult<Option<T>> from retrieving the item with the specified key using Json
/// (de)serialization because bincode2 annoyingly uses a float op when deserializing an enum.
/// Returns Ok(None) if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn json_may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Json::deserialize(&value).map(Some),
        None => Ok(None),
    }
}
