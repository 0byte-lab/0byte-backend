// use std::error::Error;
// use crate::config::SETTINGS;
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{signature::{read_keypair_file, Signer}, transaction::Transaction};
// use solana_sdk::system_instruction::transfer;
// use log::info;

// pub async fn anchor_to_solana(hash: &str, proof: &str) -> Result<String, Box<dyn Error>> {
//     let keypair_path = &SETTINGS.solana_keypair_path;

//     let payer = read_keypair_file(keypair_path)
//         .map_err(|e| format!("Failed to read Solana keypair: {}", e))?;

//     let client = RpcClient::new(SETTINGS.solana_rpc_url.to_string());

//     let recent_blockhash = client.get_latest_blockhash()?;
//     let tx = Transaction::new_signed_with_payer(
//         &[transfer(
//             &payer.pubkey(),
//             &payer.pubkey(),
//             0,
//         )],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhash,
//     );

//     let sig = client.send_and_confirm_transaction(&tx)?;
//     info!("Anchored hash {} with proof {} on Solana (txn: {})", hash, proof, sig);

//     Ok(sig.to_string())
// }


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

pub async fn anchor_to_solana(hash: &str, proof: &str) -> Result<String, Box<dyn Error>> {
    let keypair_path = shellexpand::tilde(&SETTINGS.solana_keypair_path).to_string();
    let payer = read_keypair_file(&keypair_path)
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