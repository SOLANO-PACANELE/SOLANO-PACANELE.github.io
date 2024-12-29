use anchor_lang::prelude::*;

declare_id!("7eKDiZL6sPH4tKUkNjyjQeL7crKPoTXxVG3w5VGC1CJB");

#[program]
pub mod program_pacanele {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
