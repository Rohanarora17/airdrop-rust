use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

use crate::airdrop::RPC_URL;

#[test]
fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let to_pubkey = Pubkey::from_str("bGmtRVwFdYQPeTCM5Tjdxr3iqZoyupaDPBJuNY9tic4").unwrap();

    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print our transaction out
    println!(
        "Success! Check out your TX here:
    https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );

    // Get balance of dev wallet
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print our transaction out
    println!(
        "Success! Check out your TX here:
    https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
