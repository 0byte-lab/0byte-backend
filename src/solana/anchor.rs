use std::error::Error;
use crate::config::SETTINGS;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    signature::{read_keypair_file, Signer},
    transaction::Transaction
};
use spl_memo::build_memo;
use log::info;
use std::env;
use std::fs;
use std::io::Write;

pub async fn anchor_to_solana(hash: &str, proof: &str) -> Result<String, Box<dyn Error>> {
    // Decode Solana keypair from environment
    let b64 = env::var("SOLANA_KEYPAIR_B64")
        .map_err(|_| "SOLANA_KEYPAIR_B64 environment variable not set")?;
    let decoded = base64::decode(&b64)
        .map_err(|e| format!("Failed to decode base64 keypair: {}", e))?;

    let tmp_path = "/tmp/solana_keypair.json";
    fs::create_dir_all("/tmp")?;
    let mut file = fs::File::create(tmp_path)?;
    file.write_all(&decoded)?;
    let payer = read_keypair_file(tmp_path)
        .map_err(|e| format!("Failed to read Solana keypair: {}", e))?;

    let client = RpcClient::new(SETTINGS.solana_rpc_url.clone());

    let memo_data = format!("{}|{}", hash, proof);
    let memo_ix: Instruction = build_memo(memo_data.as_bytes(), &[]);

    let recent_blockhash = client.get_latest_blockhash().await?;
    let message = Message::new(&[memo_ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, recent_blockhash);

    let sig = client.send_and_confirm_transaction(&tx).await?;
    info!("Anchored on Solana (txn: {})", sig);

    Ok(sig.to_string())
}