use solana_program::last_restart_slot::LastRestartSlot;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use rules::rule_set::RuleSet;
use solana_program::account_info::next_account_info;
use solana_program::clock::Clock;
use solana_program::program_error::ProgramError;
use solana_program::sysvar::Sysvar;
use solana_program::program::set_return_data;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    
    let accounts_iter = &mut _accounts.iter();
    let sysvar_slot_history = next_account_info(accounts_iter)?;
    let not_random = not_really_random(sysvar_slot_history)?.to_le_bytes();
    msg!("NOT_RANDOM {:?}", not_random);
    let var_a = u16::from_le_bytes([not_random[0], not_random[1]]);
    let var_b = u16::from_le_bytes([not_random[2], not_random[3]]);
    let var_c = u16::from_le_bytes([not_random[4], not_random[5]]);
    let var_d = u16::from_le_bytes([not_random[6], not_random[7]]);

    // let r = RuleSet::default_rule_set();
    // use rules::generated_rules::*;
    let r = rules::rule_set::RuleSet::p96();
    
    let rv = r.play_random_from_seed([var_a, var_b, var_c^var_d]);
    msg!("RESULT: {:?}", rv);
    let rv = bincode::serialize(&rv).unwrap();
    set_return_data(&rv);

    Ok(())
}

fn not_really_random(sysvar_slot_history: &AccountInfo) -> Result<u64, ProgramError> {
    let clock = {
        let clock = Clock::get()?;
        let a = clock.slot;
        let b = clock.unix_timestamp;
        let c = clock.epoch_start_timestamp;
        a ^ b as u64 ^ c as u64
    };

    let recent = get_recent_block_hashes(sysvar_slot_history)?;

    Ok(clock^recent)
}

pub fn get_recent_block_hashes(sysvar_slot_history: &AccountInfo) -> Result<u64, ProgramError> {
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

    let v1 = u64::from_le_bytes(data[16..16+8].try_into().unwrap());
    let v2 = u64::from_le_bytes(data[data_len-16..data_len-8].try_into().unwrap());
    let v3 = u64::from_le_bytes(data[data_len/2..data_len/2+8].try_into().unwrap());
    let x = v1 ^ v2 ^ v3;
    msg!("recent block hashes: {}", x);
    Ok(x)
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
