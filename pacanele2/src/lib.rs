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

const CREDIT_IN_LAMPORTS: u64 = 405694;

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // extract accounts
    let accounts_iter = &mut _accounts.iter();
    let sysvar_slot_history = next_account_info(accounts_iter)?;
    let bank_account = next_account_info(accounts_iter)?;
    let bank_bump = _instruction_data[0];
    let bank_signer_seeds: &[&[&[u8]]] = &[&[b"bank", &[bank_bump]]];
    let player_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // send credits to bank account
    msg!("insert coin: {} lamports", CREDIT_IN_LAMPORTS);
    let insert_coin_instruction = solana_program::system_instruction::transfer(
        player_account.key,
        bank_account.key,
        CREDIT_IN_LAMPORTS,
    );
    solana_program::program::invoke(
        &insert_coin_instruction,
        &[
            player_account.to_owned(),
            bank_account.to_owned(),
            system_program.to_owned(),
        ],
    )?;

    // compute funny random
    let not_random = not_really_random(sysvar_slot_history, 0)?;
    msg!("funny random: {:?}", not_random);

    // compute banana seed
    use rand::Rng;
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    let mut chacha = ChaCha8Rng::from_seed(not_random);
    let seed = chacha.gen();

    // compute banana
    msg!("banana seeds: {:?}", seed);
    let r = rules::rule_set::RuleSet::p96();
    let rv = r.play_random_from_seed(seed);
    let win = rv.1 as u64;
    msg!("RESULT: {:?}", rv);

    // send win back
    if win > 0 {
        let win_lamports = CREDIT_IN_LAMPORTS * win;
        msg!("won {} lamports", win_lamports);
        let return_win_instruction = solana_program::system_instruction::transfer(
            bank_account.key,
            player_account.key,
            win_lamports,
        );
        solana_program::program::invoke_signed(
            &return_win_instruction,
            &[
                bank_account.to_owned(),
                player_account.to_owned(),
                system_program.to_owned(),
            ],
            bank_signer_seeds,
        )?;
    }

    // set return data
    let rv = bincode::serialize(&rv).unwrap();
    set_return_data(&rv);

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
