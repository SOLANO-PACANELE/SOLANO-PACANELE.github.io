use anchor_lang::prelude::*;

declare_id!("5zyZQVoK55mDrTr9pWJKhqX9uMp6xD6eDdsdQrqeWKXR");

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
