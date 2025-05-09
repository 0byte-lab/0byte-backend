// use axum::Json;
// use serde::Deserialize;
// use crate::handlers::proof::verify_proof;

// #[derive(Deserialize)]
// pub struct VerifyInput {
//     pub image: Vec<u8>,
//     pub platform: String,
//     pub model: String,
//     pub proof: String,
// }

// pub async fn verify_proof_route(Json(payload): Json<VerifyInput>) -> Json<bool> {
//     let result = verify_proof(&payload.image, &payload.platform, &payload.model, &payload.proof);
//     Json(result)
// }