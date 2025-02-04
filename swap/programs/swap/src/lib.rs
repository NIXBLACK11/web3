use anchor_lang::prelude::*;

declare_id!("DWnqsRihDuB2aNZoWq3fPFXsdQqYk1mXAqCAiXHPbu3x");

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
