use anchor_lang::prelude::*;

declare_id!("A3NxPbGv9f9y6j7mEmMeptZp7n8CYYyJxZeuMde68Gi6");

#[program]
pub mod pacanele {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
