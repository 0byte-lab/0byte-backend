use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};

#[derive(Clone, Debug)]
pub struct CircuitData {
    pub bytecode: Vec<u8>,
    pub abi: Value,
}

pub fn load_circuit() -> Result<CircuitData> {
    let path: PathBuf = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("circuits/target/zkp.json");

    let circuit_json = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&circuit_json)?;

    let bytecode_base64 = parsed["bytecode"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing 'bytecode' field or not a string"))?;

    let bytecode = general_purpose::STANDARD.decode(bytecode_base64)?;

    Ok(CircuitData {
        bytecode,
        abi: parsed["abi"].clone(),
    })
}
