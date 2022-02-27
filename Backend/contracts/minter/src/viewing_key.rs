use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Env;

use crate::rand::{extend_entropy, sha_256, Prng};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ViewingKey(pub String);

impl ViewingKey {
    pub fn new(env: &Env, seed: &[u8], entropy: &[u8]) -> Self {
        let rng_entropy = extend_entropy(env, entropy);

        let mut rng = Prng::new(seed, &rng_entropy);

        let rand_slice = rng.rand_bytes();

        let key = sha_256(&rand_slice);

        Self(base64::encode(key))
    }
}

impl fmt::Display for ViewingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
