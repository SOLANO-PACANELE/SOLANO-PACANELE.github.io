use base64::Engine;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
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

pub async fn simulate_compute_limit(
    client: &RpcClient,
    instructions: &[Instruction],
    payer_pubkey: &Pubkey,
) -> Result<u32, String> {
    let transaction = Transaction::new_with_payer(instructions, Some(payer_pubkey));
    let vt: VersionedTransaction = transaction.into();
    let sim = client
        .simulate_transaction(&vt)
        .await
        .map_err(|e| format!("! sim fail: {:?}", e))?;
    let consumed = sim.value.units_consumed.unwrap_or_default();
    // info!("run transaction simulation: {} units projected.", consumed);
    let consumed = (consumed + 3000 + (consumed / 2)).clamp(5000, 166666);
    Ok(consumed as u32)
}

pub async fn avg_priority_fee(client: &RpcClient) -> Result<u64, String> {
    let fees = client
    .get_recent_prioritization_fees()
    .await
    .map_err(|e| format!("{}", e))?;
if fees.is_empty() {
    Ok(1)
} else {
    let fee_sum: u64 = fees.iter().map(|f| f.prioritization_fee).sum();
    Ok((fee_sum / fees.len() as u64).clamp(1, 50000))
}
}

pub async fn run_transaction(
    client: &RpcClient,
    payer: Keypair,
    instructions: &[Instruction],
) -> Result<UiTransactionStatusMeta, String> {
    // estimate compute units consumed by thing

    let compute_limit = 
        simulate_compute_limit(client, instructions, &payer.pubkey()).await?;

    let avg_priority_fee = avg_priority_fee(client).await?;
    // info!("recent priority fees = {} micro lamport per compute unit", avg_priority_fee);

    // sign real tx with consume limit
    let mut instructions2 = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(compute_limit),
        ComputeBudgetInstruction::set_compute_unit_price(avg_priority_fee),
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

pub async fn _demo() -> Result<UiTransactionStatusMeta, String> {
    let client = get_client().await;
    let k = create_new_keypair();
    request_airdrop(&client, &k.pubkey(), 1).await;
    spin_pcnl(&client, k, 20).await
}

fn spin_pcnl_instruction(player: &Pubkey, bet_amount_exp: u8) -> Result<Instruction, String> {
    assert!(bet_amount_exp > 10);
    assert!(bet_amount_exp < 62);

    let program_id = get_program_address();
    let (bank_address, bank_bump) = get_bank_address();

    let instruction_spin_pcnl = Instruction::new_with_bytes(
        program_id,
        &[
            // instruction data 1 = bank account bump (for seed)
            bank_bump, 
            // instruction data 2 = bet amount (as 2's exponent)
            bet_amount_exp
        ],
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
                pubkey: *player,
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
    Ok(instruction_spin_pcnl)
}

pub async fn spin_pcnl(
    client: &RpcClient,
    payer: Keypair,
    bet_amount_exp: u8,
) -> Result<UiTransactionStatusMeta, String> {
    let bet_interval = pcnl_possible_bet_interval(client, &payer.pubkey()).await?;
    info!("spin_pcnl {bet_amount_exp} {bet_interval:?}");
    if bet_amount_exp > bet_interval.1 || bet_amount_exp < bet_interval.0 {
        return Err(format!("bet amount exp {bet_amount_exp} not in interval {bet_interval:?} inclusive!"))
    }

    let instruction_spin_pcnl = spin_pcnl_instruction(&payer.pubkey(), bet_amount_exp)?;
    run_transaction(&client, payer, &[instruction_spin_pcnl]).await
}

pub async fn pcnl_possible_bet_interval(client: &RpcClient, key: &Pubkey) -> Result<(u8, u8), String> {
    // 1 / MIN_BET_PER_FEE must be smaller than 1-payout
    const MIN_BET_PER_FEE : u64 = 66;
    let acc =client.get_account(key).await.map_err(|e| format!("{}", e))?;
    let balance = acc.lamports;

    let bank_acc =client.get_account(&get_bank_address().0).await.map_err(|e| format!("{}", e))?;
    let bank_balance = bank_acc.lamports;


    let rent =  client.get_minimum_balance_for_rent_exemption(1).await.map_err(|e| format!("{}", e))?;

    let instruction_spin_pcnl = spin_pcnl_instruction(key, 11)?;
    let simulated_compute_unit = simulate_compute_limit(client, &[instruction_spin_pcnl], key).await?;
    let simulated_price = avg_priority_fee(client).await?;
    let exact_tx_price = simulated_compute_unit as u64 * simulated_price / 1000000 + 5000;
    let exact_tx_price = exact_tx_price + exact_tx_price / 10;

    let available_to_play = (balance as i64 - rent as i64 - exact_tx_price as i64 - 1).max(0) as u64;

    let bank_available = (bank_balance as i64 - rent as i64 - exact_tx_price as i64 - 1).max(0) as u64;
    info!("user balance = {balance} ; rent = {rent} ; tx_price = {exact_tx_price} ; available = {available_to_play} ; bank_balance = {bank_balance} ; bank_available = {bank_available}");

    if bank_available <= exact_tx_price * MIN_BET_PER_FEE + 1 {
        let msg = format!("bank account: not enough coin to have {MIN_BET_PER_FEE}x fee. Plz add to bank at least {} SOL", (rent + exact_tx_price * 101 + 1) as f64 / LAMPORTS_PER_SOL as f64);
        info!("{}", msg);
        return Err(msg);
    }
    if available_to_play <= exact_tx_price * MIN_BET_PER_FEE + 1 {
        let msg = format!("account {key}: not enough coin to have bet = {MIN_BET_PER_FEE}x fee. Plz insert at least {} SOL", (rent + exact_tx_price * 101 + 1) as f64 / LAMPORTS_PER_SOL as f64);
        info!("{}", msg);
        return Err(msg);
    }
    let min_bet = f64::log2((exact_tx_price * MIN_BET_PER_FEE - 1) as f64) as u8;
    let max_bet =  f64::log2((available_to_play - 1) as f64) as u8;
    let bank_bet = f64::log2(((bank_available - 1)/MIN_BET_PER_FEE) as f64) as u8;

    assert!(min_bet > 10);
    assert!(max_bet > 10);
    assert!(bank_bet > 10);
    assert!(min_bet <= max_bet);
    assert!(max_bet < 62);
    assert!(bank_bet < 62);
    assert!(min_bet <= bank_bet);
    info!("min/max bet: {min_bet} min bet / {max_bet} max bet / {bank_bet} bank bet");

    Ok((min_bet,max_bet.min(bank_bet)))
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
