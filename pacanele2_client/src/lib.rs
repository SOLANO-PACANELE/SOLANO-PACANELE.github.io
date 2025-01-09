use base64::Engine;
use solana_sdk::transaction::VersionedTransaction;
use wasm_client_solana::solana_transaction_status::UiTransactionReturnData;
use solana_sdk::transaction_context::TransactionReturnData;
use wasm_client_solana::solana_transaction_status::UiTransactionStatusMeta;

pub use wasm_client_solana::SolanaRpcClient as RpcClient;
pub use wasm_client_solana::ClientResult;
pub use wasm_client_solana::RpcTransactionConfig;
// pub use solana_client::nonblocking::rpc_client::RpcClient;
// pub use solana_client::rpc_config::RpcTransactionConfig;
pub use solana_sdk::signature::Signature;
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::{self, Transaction},
};
// pub use solana_extra_wasm::transaction_status::UiTransactionEncoding;
pub use wasm_client_solana::solana_transaction_status::UiTransactionEncoding;
pub use std::str::FromStr;


pub fn get_program_address() -> Pubkey {
    let program_address: &str = include_str!("../../pacanele2/program_address.txt")
        .trim_start()
        .trim_end();
    let program_id = Pubkey::from_str(program_address).unwrap();
    program_id
}

pub async fn get_client() -> RpcClient {
        let rpc_url = String::from("http://127.0.0.1:8899");
        let client = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());
        ;
        client
}

pub fn create_new_keypair() -> Keypair {
    Keypair::new()
}

pub async fn request_airdrop(client: &RpcClient,target: &Pubkey, sol: u8, ) {
        // Request airdrop
        let airdrop_amount = 1_000_000_000 * sol as u64; // 1 SOL
        let signature = client
            .request_airdrop(target, airdrop_amount).await
            .expect("Failed to request airdrop");
    
        // Wait for airdrop confirmation
        loop {
            let confirmed = client.confirm_transaction(&signature).await.unwrap();
            if confirmed {
                eprintln!("AIRDROP OK {} SOL ---> ADDR={}", sol, target);
                break;
            }
            async_std::task::sleep(std::time::Duration::from_secs_f64(0.5)).await;
        }
        // print_tx_logs(client, &signature).await
}

pub async fn get_tx_meta( client: &RpcClient,signature: &Signature,) -> UiTransactionStatusMeta {
    let transaction = client
        .get_transaction_with_config(&signature, RpcTransactionConfig {
            max_supported_transaction_version: Some(0),
            encoding: Some(UiTransactionEncoding::JsonParsed),
            commitment: Some(CommitmentConfig::confirmed()),
        })
        .await
        .unwrap();
    transaction.transaction.meta.unwrap()
}


pub async fn demo() -> Result<UiTransactionStatusMeta, String> {
    let program_id = get_program_address();
    let client = get_client().await;

    // Generate a new keypair for the payer
    let payer = create_new_keypair();

    request_airdrop( &client, &payer.pubkey(), 1).await;

    // Create the instruction
    let instruction = Instruction::new_with_bytes(
        program_id,
        &[], // Empty instruction data
        // account data
        vec![
            // 1st account = slot_hashes metavar for some bytes
            AccountMeta {
                pubkey: solana_sdk::slot_hashes::sysvar::id(),
                is_signer: false,
                is_writable: false,
            },
        ],
    );

    // Add the instruction to new transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().await.unwrap());
    let vt : VersionedTransaction = transaction.into();

    // Send and confirm the transaction
    let signature = match client.send_and_confirm_transaction(&vt).await {
        Ok(signature) => signature,
        Err(err) => {
            return Err(format!("! Error sending transaction:\n {}", err));
        }
    };

    Ok(get_tx_meta(&client, &signature).await)
}

pub fn base64_decode_return(r: &UiTransactionStatusMeta) -> Vec<u8> {
    let s = r.return_data.clone().unwrap().data.0;
    base64::prelude::BASE64_STANDARD.decode(&s).unwrap()
}