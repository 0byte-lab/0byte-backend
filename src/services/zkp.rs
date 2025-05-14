// src/services/zkp_mock.rs
use chrono::Utc;
use uuid::Uuid;

pub fn mock_proof(_hash: &str) -> String {
    // a unique identifier for the proof
    let proof_id = Uuid::new_v4().to_string();

    // Capture current UTC timestamp in RFC3339 format
    let timestamp = Utc::now().to_rfc3339();

    // Construct and return the mock proof string
    format!("MOCK_PROOF:{}:{}", proof_id, timestamp)
}