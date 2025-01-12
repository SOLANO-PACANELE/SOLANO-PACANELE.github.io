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
use solana_program::serialize_utils::read_u16;
use solana_program::serialize_utils::read_pubkey;

entrypoint!(process_instruction);

struct InputParameters<'a, 'b> {
    sysvar_slot_hashes: &'b AccountInfo<'a>,
    bank_account: &'b AccountInfo<'a>,
    bank_bump: u8,
    player_account: &'b AccountInfo<'a>,
    system_program: &'b AccountInfo<'a>,
    bet_amount: u64,
    program_id: Pubkey,
    program_account: &'b AccountInfo<'a>,
}


pub fn get_program_id<'info>(info: AccountInfo<'info>) -> Result<Pubkey, ProgramError> {
    let instruction_sysvar = info.data.borrow();
    let mut idx = 0;
    let num_instructions = read_u16(&mut idx, &instruction_sysvar).map_err(|_e| ProgramError::Custom(1))?;

    for index in 0..num_instructions {
        let mut current = 2 + (index * 2) as usize;
        let start = read_u16(&mut current, &instruction_sysvar).unwrap();

        current = start as usize;
        let num_accounts = read_u16(&mut current, &instruction_sysvar).unwrap();
        current += (num_accounts as usize) * (1 + 32);
        let program_id = read_pubkey(&mut current, &instruction_sysvar).unwrap();

        if program_id == Pubkey::from_str_const("ComputeBudget111111111111111111111111111111") {
            continue;
        }
        if program_id == Pubkey::from_str_const("11111111111111111111111111111111") {
            continue;
        }
        return Ok(program_id);

    }

    Err(ProgramError::Custom(2))
}

fn extract_input<'a, 'b>(_accounts: &'b [AccountInfo<'a>], _instruction_data: &[u8]) -> Result<InputParameters<'a, 'b>, ProgramError> {
    let accounts_iter = &mut _accounts.iter();
    let sysvar_instructions = next_account_info(accounts_iter)?;
    solana_program::sysvar::instructions::check_id(&sysvar_instructions.key);
    let program_id = get_program_id(sysvar_instructions.clone())?;
    
    let sysvar_slot_hashes = next_account_info(accounts_iter)?;
    solana_program::sysvar::slot_hashes::check_id(&sysvar_slot_hashes.key);
    
    let system_program = next_account_info(accounts_iter)?;
    solana_program::system_program::check_id(&system_program.key);

    let bank_account = next_account_info(accounts_iter)?;
    let bank_bump = _instruction_data[0];
    let x = solana_program::pubkey::Pubkey::find_program_address(&[b"bank"], &program_id);
    assert!(x.0 == *bank_account.key);
    assert!(x.1 == bank_bump);

    let player_account = next_account_info(accounts_iter)?;
    assert!(_instruction_data[1] > 10 && _instruction_data[1] < 63);
    let bet_amount = 1_u64 <<( _instruction_data[1] as u64);
    assert!(bet_amount > 33 * 5000);
    assert!(bet_amount < player_account.lamports());
    assert!(bet_amount < bank_account.lamports());
    
    let program_account = next_account_info(accounts_iter)?;
    assert!(*program_account.key == program_id);

    Ok(InputParameters {
        sysvar_slot_hashes,bank_account,bank_bump,player_account,system_program, bet_amount, program_id, program_account
    })
}

fn invoke_transfer_player_to_bank(input: &InputParameters, bet_amount: u64) -> Result<(), ProgramError> {
    // msg!("insert coin: {} lamports", amount);
    // keep 3.33% in program account
    // let program_fee_amount = bet_amount / 33;
    // let bank_amount = bet_amount - program_fee_amount;
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            input.player_account.key,
            input.bank_account.key,
            bet_amount,
        ),
        &[
            input.player_account.to_owned(),
            input.bank_account.to_owned(),
            input.system_program.to_owned(),
        ],
    )?;
    
    // solana_program::program::invoke(
    //     &solana_program::system_instruction::transfer(
    //         input.player_account.key,
    //         input.program_account.key,
    //         program_fee_amount,
    //     ),
    //     &[
    //         input.player_account.to_owned(),
    //         input.program_account.to_owned(),
    //         input.system_program.to_owned(),
    //     ],
    // )?;
    

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
    let not_random = not_really_random(input.sysvar_slot_hashes, 0)?;
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
        let max_payable = input.bank_account.lamports()/2-890880*2;
        let win_lamports = (input.bet_amount * win).min(max_payable);
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
