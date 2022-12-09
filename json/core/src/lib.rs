use risc0_zkp::core::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub hash1: Digest,
    pub hash2: Digest,
}
