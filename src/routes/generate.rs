use axum::{Json, response::Json as JsonResponse};
use serde::Deserialize;
use crate::handlers::proof::generate_proof;
use base64::{engine::general_purpose, Engine as _};

#[derive(Deserialize)]
pub struct GenerateInput {
    pub image: String,      // Image as raw bytes
    pub platform: String,
    pub model: String,
}

pub async fn generate_proof_route(
    Json(payload): Json<GenerateInput>
) -> JsonResponse<String> {
    // Convert base64 image to bytes
    let image_bytes = general_purpose::STANDARD.decode(&payload.image)
        .map_err(|e| format!("Failed to decode base64 image: {}", e))
        .unwrap();
    
    let proof_json = generate_proof(
        &image_bytes,
        &payload.platform,
        &payload.model
    );
    JsonResponse(proof_json)
}
