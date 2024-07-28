use crate::airdrop::RPC_URL;
use crate::programs::wba_prereq::{CompleteArgs, UpdateArgs, WbaPrereqProgram};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::system_program;
use solana_sdk::{
    message::Message,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

#[test]
fn enroll() {
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
    let prereq =
        WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

    let args = CompleteArgs {
        github: b"Rohanarora17".to_vec(),
    };
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
