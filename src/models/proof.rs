use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Proof {
    pub version: String,
    pub image_phash: [u8; 8],
    pub proof_hash: [u8; 32],
    pub timestamp: i64,
    pub zk_proof: String,
}
