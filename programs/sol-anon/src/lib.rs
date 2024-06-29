use anchor_lang::prelude::*;

declare_id!("F8STtLnjyUZ4KMykWkvVZx33CPqi3dA6JXJdbuwEqvf9");

#[program]
pub mod sol_anon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
