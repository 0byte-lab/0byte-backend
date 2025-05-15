use actix_web::{post, web, HttpResponse, Responder, http::header};
use base64::engine::general_purpose;
use base64::Engine as _;
use image::{load_from_memory, guess_format, ImageFormat};

use crate::services::{embedd::embed_metadata, phash::compute_phash, zkp::mock_proof};
use crate::solana::anchor::anchor_to_solana;
use crate::models::proof::GenerateRequest;

#[post("/generate-proof")]
pub async fn generate_proof(req: web::Json<GenerateRequest>) -> impl Responder {

    // Step 1: Decode base64
    let img_data = match general_purpose::STANDARD.decode(&req.image_bytes) {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid base64 image data: {}", e)),
    };

    // Step 2: Load image
    let dyn_img = match load_from_memory(&img_data) {
        Ok(img) => img,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid image format: {}", e)),
    };

    // Step 3: Compute phash
    let phash = compute_phash(&dyn_img);
    let phash_hex = hex::encode(phash);

    // Step 4: Build metadata hash
    let metadata_hash = format!("{}:{}", req.input_token_count, req.output_token_count);
    let combined_hash = format!("{}:{}:{}:{}", phash_hex, req.model_name, req.platform_name, metadata_hash);

    // Step 5: Mock proof + Solana anchor
    let proof = mock_proof(&combined_hash);
    let txn_id = match anchor_to_solana(&combined_hash, &proof).await {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Solana anchoring failed: {}", e)),
    };

    // Step 6: Embed metadata
    let embedded_bytes = match embed_metadata(&img_data, &txn_id, &req.platform_name) {
        Ok(bytes) => bytes,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to embed metadata: {}", e)),
    };

    // Step 7: Guess content type
    let content_type = match guess_format(&embedded_bytes) {
        Ok(ImageFormat::Png) => "image/png",
        Ok(ImageFormat::Jpeg) => "image/jpeg",
        _ => "application/octet-stream",
    };

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, content_type))
        .insert_header(("X-Transaction-Id", txn_id))
        .body(embedded_bytes)
}