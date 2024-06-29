use anchor_lang::prelude::*;

declare_id!("73o3wfunMxS44tEuia2D4WJKmJtqRewX3F4tG2fmkSEV");

#[program]
pub mod sol_anon {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = *ctx.accounts.admin.key;
        // msg!("Inbox initialized with admin: {:?}", inbox.admin);
        Ok(())
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = new_admin;
        // msg!("Inbox admin changed to: {:?}", inbox.admin);
        Ok(())
    }
}

#[account]
pub struct Inbox {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeInbox<'info> {
    #[account(init, seeds=[b"inbox"], bump, payer = admin, space = 8 + 32)]
    pub inbox: Account<'info, Inbox>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(seeds=[b"inbox"], bump, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    pub admin: Signer<'info>,
}