use actix_web::{post, web, HttpResponse, Responder, http::header};
use base64::engine::general_purpose;
use base64::Engine as _;
use image::{load_from_memory, guess_format, ImageFormat};

use crate::services::{embedd::embed_metadata, phash::compute_phash, zkp::mock_proof};
use crate::solana::anchor::anchor_to_solana;
use crate::models::proof::GenerateRequest;

#[post("/generate-proof")]
pub async fn generate_proof(req: web::Json<GenerateRequest>) -> impl Responder {
    // 1. Decode base64-encoded image input
    let img_data = match general_purpose::STANDARD.decode(&req.image_bytes) {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid base64 image data: {}", e)),
    };

    // 2. Load image into memory
    let dyn_img = match load_from_memory(&img_data) {
        Ok(img) => img,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid image format: {}", e)),
    };

    // 3. Compute perceptual hash
    let phash = compute_phash(&dyn_img);
    let phash_hex = hex::encode(phash);

    // 4. Combine with token metadata
    let metadata_hash = format!("{}:{}", req.input_token_count, req.output_token_count);
    let combined_hash = format!("{}:{}:{}:{}", phash_hex, req.model_name, req.platform_name, metadata_hash);

    // 5. Embed proof hash into image metadata
    let embedded_bytes = match embed_metadata(&img_data, &combined_hash) {
        Ok(bytes) => bytes,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to embed metadata: {}", e)),
    };

    // 6. Generate mock proof and anchor to Solana
    let proof = mock_proof(&combined_hash);
    let txn_id = match anchor_to_solana(&combined_hash, &proof).await {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Solana anchoring failed: {}", e)),
    };

    // 7. Determine content type for the response
    let content_type = match guess_format(&embedded_bytes) {
        Ok(ImageFormat::Png) => "image/png",
        Ok(ImageFormat::Jpeg) => "image/jpeg",
        _ => "application/octet-stream",
    };

    // 8. Return the raw image bytes with headers
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, content_type))
        .insert_header(("X-Transaction-Id", txn_id))
        .body(embedded_bytes)
}
