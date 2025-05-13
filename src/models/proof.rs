use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub image_bytes: Vec<u8>,
    pub platform_name: String,
    pub model_name: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub proof: String,
    pub image_hash: String,
    pub timestamp: u64,
    pub output_hash: String,
    pub nullifier: String,
}