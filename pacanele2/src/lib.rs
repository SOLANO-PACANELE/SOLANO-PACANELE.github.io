use solana_program::last_restart_slot::LastRestartSlot;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use rules::rule_set::RuleSet;
use solana_program::account_info::next_account_info;
use solana_program::clock::Clock;
use solana_program::program::set_return_data;
use solana_program::program_error::ProgramError;
use solana_program::sysvar::Sysvar;

entrypoint!(process_instruction);

struct InputParameters<'a, 'b> {
    sysvar_slot_history: &'b AccountInfo<'a>,
    bank_account: &'b AccountInfo<'a>,
    bank_bump: u8,
    player_account: &'b AccountInfo<'a>,
    system_program: &'b AccountInfo<'a>,
    bet_amount: u64,
}

fn extract_input<'a, 'b>(_accounts: &'b [AccountInfo<'a>], _instruction_data: &[u8]) -> Result<InputParameters<'a, 'b>, ProgramError> {
    let accounts_iter = &mut _accounts.iter();
    let sysvar_slot_history = next_account_info(accounts_iter)?;
    let bank_account = next_account_info(accounts_iter)?;
    let bank_bump = _instruction_data[0];
    let player_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let bet_amount = 1_u64 <<( _instruction_data[1] as u64);
    Ok(InputParameters {
        sysvar_slot_history,bank_account,bank_bump,player_account,system_program, bet_amount
    })
}

fn invoke_transfer_player_to_bank(input: &InputParameters, amount: u64) -> Result<(), ProgramError> {
    // msg!("insert coin: {} lamports", amount);
    let insert_coin_instruction = solana_program::system_instruction::transfer(
        input.player_account.key,
        input.bank_account.key,
        amount,
    );
    solana_program::program::invoke(
        &insert_coin_instruction,
        &[
            input.player_account.to_owned(),
            input.bank_account.to_owned(),
            input.system_program.to_owned(),
        ],
    )?;

    Ok(())
}

fn invoke_transfer_bank_to_player(input: &InputParameters, amount: u64) -> Result<(), ProgramError> {
    // msg!("won {} lamports", amount);
    let return_win_instruction = solana_program::system_instruction::transfer(
        input.bank_account.key,
        input.player_account.key,
        amount,
    );
    let bank_signer_seeds: &[&[&[u8]]] = &[&[b"bank", &[input.bank_bump]]];
    solana_program::program::invoke_signed(
        &return_win_instruction,
        &[
            input.bank_account.to_owned(),
            input.player_account.to_owned(),
            input.system_program.to_owned(),
        ],
        bank_signer_seeds,
    )?;
    Ok(())
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // extract accounts
    let input = extract_input(_accounts, _instruction_data)?;

    
    // msg!("after init accounts:");    ::solana_program::log::sol_log_compute_units();

    // send credits to bank account
    invoke_transfer_player_to_bank(&input, input.bet_amount)?;

    // msg!("after insert coin:");    ::solana_program::log::sol_log_compute_units();

    // compute funny random
    let not_random = not_really_random(input.sysvar_slot_history, 0)?;
    // msg!("funny random: {:?}", not_random);

    
    // msg!("after funny random:");    ::solana_program::log::sol_log_compute_units();

    // compute banana seed
    use rand::Rng;
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    let mut chacha = ChaCha8Rng::from_seed(not_random);
    let seed = chacha.gen();

    
    // msg!("after banana seed:");    ::solana_program::log::sol_log_compute_units();

    // compute banana
    // msg!("banana seeds: {:?}", seed);
    let r = rules::rule_set::RuleSet::p96();
    let rv = r.play_random_from_seed(seed);
    let win = rv.1 as u64;
    // msg!("RESULT: {:?}", rv);

    
    // msg!("after banana result:");    ::solana_program::log::sol_log_compute_units();

    // send win back
    if win > 0 {
        let win_lamports = input.bet_amount * win;
        invoke_transfer_bank_to_player(&input, win_lamports)?;
    }

    
    // msg!("after send win back:");    ::solana_program::log::sol_log_compute_units();

    // set return data
    let rv = bincode::serialize(&rv).unwrap();
    set_return_data(&rv);

    
    msg!("after set return data:");    ::solana_program::log::sol_log_compute_units();

    Ok(())
}

fn not_really_random(
    sysvar_slot_history: &AccountInfo,
    nonce: u64,
) -> Result<[u8; 32], ProgramError> {
    let a = get_recent_block_hashes(sysvar_slot_history)?;
    let clock = Clock::get()?;
    let b = [
        clock.slot,
        clock.unix_timestamp as u64,
        clock.epoch_start_timestamp as u64,
        nonce,
    ];
    let e = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| (a ^ b).to_le_bytes())
        .flatten()
        .collect::<Vec<_>>();

    let mut o = [0; 32];
    assert!(e.len() == o.len());
    for i in 0..o.len() {
        o[i] = e[i];
    }
    Ok(o)
}

pub fn get_recent_block_hashes(
    sysvar_slot_history: &AccountInfo,
) -> Result<[u64; 4], ProgramError> {
    use solana_program::sysvar::slot_history::ProgramError;

    /*
        Decoding the SlotHashes sysvar using `from_account_info` is too expensive.
        For example this statement will exceed the current BPF compute unit budget:
            let slot_hashes = SlotHashes::from_account_info(&sysvar_slot_history).unwrap();
        Instead manually decode the sysvar.
    */

    if *sysvar_slot_history.key != solana_program::slot_hashes::sysvar::id() {
        msg!("Invalid SlotHashes sysvar");
        return Err(ProgramError::InvalidArgument);
    }

    let data = sysvar_slot_history.try_borrow_data()?;
    let data_len = data.len();

    let v1 = u64::from_le_bytes(data[16..16 + 8].try_into().unwrap());
    let v2 = u64::from_le_bytes(data[data_len - 16..data_len - 8].try_into().unwrap());
    let v3 = u64::from_le_bytes(data[data_len / 2..data_len / 2 + 8].try_into().unwrap());
    let v4 = u64::from_le_bytes(data[data_len / 3..data_len / 3 + 8].try_into().unwrap());
    Ok([v1, v2, v3, v4])
}

fn print_pacanel(r: &RuleSet, seed: u16) {
    let p = r.play_random_from_seed([seed; 3]);
    msg!("PACANEL/{seed}: {:?}!", p);
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{signature::Signer, transaction::Transaction};

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("pacanele2", program_id, processor!(process_instruction))
                .start()
                .await;

        // Create the instruction to invoke the program
        let instruction =
            solana_program::instruction::Instruction::new_with_borsh(program_id, &(), vec![]);

        // Add the instruction to a new transaction
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);

        // Process the transaction
        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());
    }
}
