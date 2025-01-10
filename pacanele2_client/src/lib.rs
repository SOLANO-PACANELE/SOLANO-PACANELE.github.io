use base64::Engine;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::transaction_context::TransactionReturnData;
use wasm_client_solana::solana_transaction_status::UiTransactionReturnData;
use wasm_client_solana::solana_transaction_status::UiTransactionStatusMeta;

pub use solana_sdk::account::Account;

pub use wasm_client_solana::ClientResult;
use wasm_client_solana::RpcSimulateTransactionConfig;
pub use wasm_client_solana::RpcTransactionConfig;
pub use wasm_client_solana::SolanaRpcClient as RpcClient;
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
pub use std::str::FromStr;
pub use wasm_client_solana::solana_transaction_status::UiTransactionEncoding;

use tracing::info;

pub const CREDIT_IN_LAMPORTS: u64 = 405694;

pub fn get_program_address() -> Pubkey {
    let program_address: &str = include_str!("../../pacanele2/program_address.txt")
        .trim_start()
        .trim_end();
    let program_id = Pubkey::from_str(program_address).unwrap();
    program_id
}

pub fn get_bank_address() -> (Pubkey, u8) {
    let program_id = get_program_address();
    let seed = b"bank";
    Pubkey::find_program_address(&[seed], &program_id)
}

pub fn get_solana_rpc_url() -> String {
    String::from("http://127.0.0.1:8899")
}

pub async fn get_client() -> RpcClient {
    let rpc_url = get_solana_rpc_url();
    let client = RpcClient::new_with_commitment(&rpc_url, CommitmentConfig::confirmed());
    client
}

pub fn create_new_keypair() -> Keypair {
    Keypair::new()
}

pub async fn request_airdrop(client: &RpcClient, target: &Pubkey, sol: u8) {
    // Request airdrop
    let airdrop_amount = 1_000_000_000 * sol as u64; // 1 SOL
    let signature = client
        .request_airdrop(target, airdrop_amount)
        .await
        .expect("Failed to request airdrop");

    // Wait for airdrop confirmation
    loop {
        let confirmed = client.confirm_transaction(&signature).await.unwrap();
        if confirmed {
            info!("AIRDROP OK {} SOL ---> ADDR={}", sol, target);
            break;
        }
        async_std::task::sleep(std::time::Duration::from_secs_f64(0.5)).await;
    }
    // print_tx_logs(client, &signature).await
}

pub async fn get_tx_meta(client: &RpcClient, signature: &Signature) -> UiTransactionStatusMeta {
    let transaction = client
        .get_transaction_with_config(
            &signature,
            RpcTransactionConfig {
                max_supported_transaction_version: Some(0),
                encoding: Some(UiTransactionEncoding::JsonParsed),
                commitment: Some(CommitmentConfig::confirmed()),
            },
        )
        .await
        .unwrap();
    transaction.transaction.meta.unwrap()
}

pub async fn run_transaction(
    client: &RpcClient,
    payer: Keypair,
    instructions: &[Instruction],
) -> Result<UiTransactionStatusMeta, String> {
    // estimate compute units consumed by thing
    let transaction = Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
    let compute_limit = {
        let vt: VersionedTransaction = transaction.into();
        let sim = client
            .simulate_transaction(&vt)
            .await
            .map_err(|e| format!("! sim fail: {:?}", e))?;
        let consumed = sim.value.units_consumed.unwrap_or_default();
        info!("run transaction simulation: {} units projected.", consumed);
        let consumed = (consumed + 3000 + (consumed / 4)).clamp(6666, 166666);
        consumed as u32
    };

    let priority_fee = {
        let fees = client
            .get_recent_prioritization_fees()
            .await
            .map_err(|e| format!("{}", e))?;
        if fees.is_empty() {
            0
        } else {
            let fee_sum: u64 = fees.iter().map(|f| f.prioritization_fee).sum();
            (fee_sum / fees.len() as u64).clamp(0, 5000)
        }
    };
    info!("recent priority fees = {}.", priority_fee);

    // sign real tx with consume limit
    let mut instructions2 = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_limit),
        ComputeBudgetInstruction::set_compute_unit_price(priority_fee),
    ];
    for i in instructions {
        instructions2.push(i.clone());
    }
    let mut transaction = Transaction::new_with_payer(&instructions2, Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().await.unwrap());
    let vt: VersionedTransaction = transaction.into();

    // Send and confirm the transaction
    let signature = match client.send_and_confirm_transaction(&vt).await {
        Ok(signature) => signature,
        Err(err) => {
            return Err(format!("! Error sending transaction:\n {}", err));
        }
    };

    Ok(get_tx_meta(&client, &signature).await)
}

pub async fn spin_pcnl(
    client: &RpcClient,
    payer: Keypair,
) -> Result<UiTransactionStatusMeta, String> {
    let program_id = get_program_address();
    let (bank_address, bank_bump) = get_bank_address();

    let instruction_spin_pcnl = Instruction::new_with_bytes(
        program_id,
        // instruction data = bank account bump (for seed)
        &[bank_bump],
        // account data
        vec![
            // 1st account = slot_hashes metavar for some bytes
            AccountMeta {
                pubkey: solana_sdk::slot_hashes::sysvar::id(),
                is_signer: false,
                is_writable: false,
            },
            // 2nd account = bank
            AccountMeta {
                pubkey: bank_address,
                is_signer: false,
                is_writable: true,
            },
            // 3rd account = player
            AccountMeta {
                pubkey: payer.pubkey(),
                is_signer: true,
                is_writable: true,
            },
            // 4th account = system program
            AccountMeta {
                pubkey: solana_sdk::system_program::id(),
                is_signer: false,
                is_writable: false,
            },
        ],
    );

    run_transaction(&client, payer, &[instruction_spin_pcnl]).await
}

pub async fn send_money(
    client: &RpcClient,
    payer: Keypair,
    target: Pubkey,
    lamports: u64,
) -> Result<UiTransactionStatusMeta, String> {
    let send_solana = solana_sdk::system_instruction::transfer(&payer.pubkey(), &target, lamports);

    run_transaction(client, payer, &[send_solana]).await
}

pub fn base64_decode_return(r: &UiTransactionStatusMeta) -> Result<Vec<u8>, String> {
    let s = r.return_data.clone().ok_or("no return data!")?.data.0;
    base64::prelude::BASE64_STANDARD
        .decode(&s)
        .map_err(|e| format!("base64 decode error: {}", e))
}
