use actix_web::{web, HttpResponse};
use std::sync::Arc;
use base64::Engine;
use hex;
use base64::{engine::general_purpose, Engine as _};

use noir_rs::{
    barretenberg::{srs::setup_srs_from_bytecode, prove::prove_ultra_honk},
    witness::from_vec_str_to_witness_map,
};

use crate::{
    models::proof::{GenerateRequest, GenerateResponse},
    services::witness::generate_witness,
};

pub async fn generate_proof(
    request: web::Json<GenerateRequest>,
    circuit: web::Data<Arc<String>>,
) -> Result<HttpResponse, actix_web::Error> {

    // generate witness
    let witness = generate_witness(&request.image_bytes, &request.platform_name);

    // convert image_hash + p_hash + platform_name + timestamp into decimal strings
    let mut vals = Vec::new();
    for fe in &witness.image_hash {
        vals.push(fe.to_string());
    }
    vals.push(witness.p_hash.to_string());
    for fe in &witness.platform_name {
        vals.push(fe.to_string());
    }
    vals.push(witness.timestamp.to_string());

    // 3) Add placeholders for public outputs (4 hash elems + nullifier)
    //    (noir_rs will fill them in during proof)
    for _ in 0..5 {
        vals.push("0".to_string());
    }

    // 4) Build the witness map
    let witness_map = from_vec_str_to_witness_map(&vals)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("witness map error: {}", e)))?;

    // 5) Ensure SRS is set up (you can call this once at startup instead)
    let json_acir = circuit.get_ref();
    setup_srs_from_bytecode(json_acir, None, false)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("SRS setup error: {}", e)))?;

    // 6) Generate the proof
    let proof_bytes = prove_ultra_honk(json_acir, witness_map,0)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("proof error: {}", e)))?;

    // recompute the raw image‐hash bytes & timestamp for response
    let image_hash_bytes: Vec<u8> = witness
        .image_hash
        .iter()
        .flat_map(|f| f.to_be_bytes())
        .collect();
    let ts_u64 = witness.timestamp.to_u128() as u64;

    // encode and return
    Ok(HttpResponse::Ok().json(GenerateResponse {
        proof: general_purpose::STANDARD.encode(&proof_bytes),
        image_hash: hex::encode(image_hash_bytes),
        timestamp: ts_u64,
        output_hash: String::new(),
        nullifier: String::new(),
    }))
}