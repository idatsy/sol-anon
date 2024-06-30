use anchor_lang::prelude::*;

declare_id!("73o3wfunMxS44tEuia2D4WJKmJtqRewX3F4tG2fmkSEV");

#[program]
pub mod sol_anon {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = *ctx.accounts.admin.key;

        msg!("Inbox initialized with admin: {:?}", inbox.admin);

        Ok(())
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = new_admin.key().clone();

        msg!("Inbox admin changed to: {:?}", inbox.admin);

        Ok(())
    }

    pub fn add_to_whitelist(_ctx: Context<AddToWhitelist>, address_to_whitelist: Pubkey) -> Result<()> {
        msg!("Adding address to whitelist: {:?}", address_to_whitelist);
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
    #[account(mut, seeds=[b"inbox"], bump, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Whitelist {}

#[derive(Accounts)]
#[instruction(address_to_whitelist: Pubkey)] // We could make this an account but showcasing how to do this for reference ^.^
pub struct AddToWhitelist<'info> {
    // TODO: If time allows, don't use anchor so we can skip the 8 byte discriminant and save another 7 bytes of space!
    // Also would be nice to have a better understanding of PDA initialization, for example seeding a PDA with rent excemption alone is not secure
    // as there *could* be a way to to send money to a non-existing PDA even though it hasn't been created by this admin.
    // Either way due to time contsraints this probably won't be possible to do with Anchor but interesting to explore using a native implementation
    #[account(init, seeds=[address_to_whitelist.key().as_ref()], bump, payer = admin, space = 8)]
    pub whitelist_pda: Account<'info, Whitelist>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
