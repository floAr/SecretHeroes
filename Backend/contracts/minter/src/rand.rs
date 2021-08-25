use cosmwasm_std::Env;
use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use sha2::{Digest, Sha256};

pub fn sha_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(hash.as_slice());
    result
}

pub struct Prng {
    rng: ChaChaRng,
}

impl Prng {
    pub fn new(seed: &[u8], entropy: &[u8]) -> Self {
        let mut hasher = Sha256::new();

        // write input message
        hasher.update(&seed);
        hasher.update(&entropy);
        let hash = hasher.finalize();

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(hash.as_slice());

        let rng: ChaChaRng = ChaChaRng::from_seed(hash_bytes);

        Self { rng }
    }

    pub fn rand_bytes(&mut self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        self.rng.fill_bytes(&mut bytes);

        bytes
    }
}

pub fn extend_entropy(env: &Env, entropy: &[u8]) -> Vec<u8> {
    // 16 here represents the lengths in bytes of the block height and time.
    let entropy_len = 16 + env.message.sender.len() + entropy.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.height.to_be_bytes());
    rng_entropy.extend_from_slice(&env.block.time.to_be_bytes());
    rng_entropy.extend_from_slice(&env.message.sender.0.as_bytes());
    rng_entropy.extend_from_slice(entropy);
    rng_entropy
}
