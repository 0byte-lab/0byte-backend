use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub image_bytes: String,      
    pub model_name: String,
    pub platform_name: String,
    pub input_token_count: u32,
    pub output_token_count: u32,
}

#[derive(Serialize)]
pub struct GenerateResponse {
    pub image_bytes: String,       
    pub transaction_id: String,
}