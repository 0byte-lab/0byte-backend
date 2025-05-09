use crate::models::proof::Proof;
use crate::handlers::p_hash::compute_phash;
use noir_rs::{prove, native_types::{WitnessMap, Witness}, FieldElement};
use sha2::{Sha256, Digest};
use chrono::Utc;
use std::fs;
use base64::{engine::general_purpose, Engine as _};
use log::error;
use serde_json::json;
use std::path::PathBuf;

// use noirc_abi::input_parser::Format;
// use noir_driver::CompiledProgram;

fn compute_output_hash(
    image_hash: &[u8; 32],
    p_hash: &[u8; 8],
    platform_name: &[u8; 32],
    timestamp: u64,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(image_hash);
    hasher.update(p_hash);
    hasher.update(platform_name);
    hasher.update(timestamp.to_be_bytes());
    hasher.finalize().into()
}

pub fn generate_proof(image: &[u8], platform: &str, _model: &str) -> String {
    // Compute perceptual hash
    let phash = match compute_phash(image) {
        Ok(phash) => phash,
        Err(_) => {
            error!("Failed to compute pHash");
            return json!({ "error": "invalid image" }).to_string();
        }
    };

    // Hash the input image
    let image_hash: [u8; 32] = Sha256::digest(image).into();

    // Normalize platform string to 32 bytes
    let mut platform_name = [0u8; 32];
    let bytes = platform.as_bytes();
    for i in 0..bytes.len().min(32) {
        platform_name[i] = bytes[i];
    }

    // Timestamp
    let timestamp = Utc::now().timestamp() as u64;

    // Convert pHash base64 -> [u8; 8]
    let phash_bytes: [u8; 8] = match general_purpose::STANDARD.decode(&phash) {
        Ok(decoded) if decoded.len() >= 8 => {
            match decoded[..8].try_into() {
                Ok(bytes) => bytes,
                Err(_) => {
                    error!("Invalid pHash length after decoding");
                    return json!({ "error": "invalid pHash length" }).to_string();
                }
            }
        }
        Ok(decoded) => {
            error!("Decoded pHash too short: {} bytes", decoded.len());
            return json!({ "error": "invalid pHash length" }).to_string();
        }
        Err(e) => {
            error!("Failed to decode pHash: {}", e);
            return json!({ "error": "invalid pHash decoding" }).to_string();
        }
    };

    // Compute output hash
    let output_hash = compute_output_hash(&image_hash, &phash_bytes, &platform_name, timestamp);

    // Populate WitnessMap
    let mut inputs = WitnessMap::new();
    inputs.insert(Witness(1), FieldElement::from_be_bytes_reduce(&image_hash));
    inputs.insert(Witness(2), FieldElement::from_be_bytes_reduce(&phash_bytes));
    inputs.insert(Witness(3), FieldElement::from_be_bytes_reduce(&platform_name));
    inputs.insert(Witness(4), FieldElement::from_be_bytes_reduce(&timestamp.to_be_bytes()));
    inputs.insert(Witness(5), FieldElement::from_be_bytes_reduce(&output_hash));

    // Load zkp.json
    let program_path: PathBuf = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("zkp/target/zkp.json");

    let program_json_string = match fs::read_to_string(&program_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read zkp.json from {}: {}", program_path.display(), e);
            return json!({ "error": "failed to load Noir program" }).to_string();
        }
    };

    
    // Parse JSON
    let program_json: serde_json::Value = match serde_json::from_str(&program_json_string) {
        Ok(json) => json,
        Err(e) => {
            error!("Invalid JSON in zkp.json: {}", e);
            return json!({ "error": "invalid Noir program JSON" }).to_string();
        }
    };

    // Extract base64 program field
    let circuit_base64 = match program_json["program"].as_str() {
        Some(s) => s,
        None => {
            error!("Missing 'program' field in zkp.json");
            return json!({ "error": "missing program field in Noir JSON" }).to_string();
        }
    };

    // Generate proof
    let (proof_bytes, _vk_bytes) = match prove(circuit_base64.to_string(), inputs) {
        Ok(result) => result,
        Err(e) => {
            error!("Proof generation failed: {}", e);
            return json!({ "error": "proof generation failed" }).to_string();
        }
    };

    // Build final proof object
    let proof = Proof {
        version: "1.0".to_string(),
        image_phash: phash_bytes,
        proof_hash: output_hash,
        timestamp: timestamp.try_into().unwrap_or(0),
        zk_proof: general_purpose::STANDARD.encode(proof_bytes),
    };

    // Serialize proof
    match serde_json::to_string(&proof) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize proof: {}", e);
            json!({ "error": "serialization failed" }).to_string()
        }
    }
}