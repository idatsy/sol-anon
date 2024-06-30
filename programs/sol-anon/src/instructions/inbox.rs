use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
    let inbox = &mut ctx.accounts.inbox;
    inbox.admin = *ctx.accounts.admin.key;
    // dev: these seem to not be needed, fields are 0-initialized by default
    // inbox.latest_free_slot = 0;
    // inbox.latest_whitelisted_slot = 0;

    msg!("Inbox initialized with admin: {:?}", inbox.admin);

    Ok(())
}

pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
    let inbox = &mut ctx.accounts.inbox;
    inbox.admin = new_admin;

    msg!("Inbox admin changed to: {:?}", inbox.admin);

    Ok(())
}

pub fn withdraw_surplus_inbox_balance(ctx: Context<WithdrawSurplusInboxBalance>) -> Result<()> {
    let inbox = &ctx.accounts.inbox;
    let admin = &ctx.accounts.admin;

    let rent = Rent::get()?;
    let minimum_balance = rent.minimum_balance(inbox.to_account_info().data_len());
    let surplus = inbox.to_account_info().lamports().saturating_sub(minimum_balance);

    **inbox.to_account_info().try_borrow_mut_lamports()? -= surplus;
    **admin.to_account_info().try_borrow_mut_lamports()? += surplus;

    msg!("Surplus of {:?} lamports withdrawn from inbox", surplus);
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeInbox<'info> {
    #[account(
        init,
        seeds=[INBOX_SEED],
        bump,
        payer = admin,
        space = INBOX_SPACE
    )]
    pub inbox: Account<'info, Inbox>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut, seeds=[INBOX_SEED], bump, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSurplusInboxBalance<'info> {
    #[account(mut, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    #[account(mut)]
    pub admin: Signer<'info>,
}