use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// card stats
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Stats {
    /// the card's skills at time of minting
    pub base: Vec<u8>,
    /// the card's current skills
    pub current: Vec<u8>,
}
