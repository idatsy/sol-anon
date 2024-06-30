//! # Message Instructions
//!
//! This module contains instructions related to sending and managing messages.

use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::utils::realloc_slot;

/// Sends a regular message.
pub fn send_regular_message(ctx: Context<SendRegularMessage>, message: String, to: Pubkey) -> Result<()> {
    let slot = &mut ctx.accounts.slot;
    slot.to = to;
    slot.message = message;

    let inbox = &mut ctx.accounts.inbox;
    inbox.latest_free_slot += 1;

    msg!("Message {:?} sent to: {:?}. New free slot: {:?}", slot.message, slot.to, inbox.latest_free_slot);
    Ok(())
}

/// Sends a whitelisted message.
pub fn send_whitelisted_message(ctx: Context<SendWhitelistedMessage>, message: String, to: Pubkey) -> Result<()> {
    let inbox = &mut ctx.accounts.inbox;

    // Reallocate the slot if necessary
    realloc_slot(
        &ctx.accounts.slot.to_account_info(),
        &message,
        inbox,
        &ctx.accounts.sender.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    let slot = &mut ctx.accounts.slot;
    slot.to = to;
    slot.message = message;

    inbox.latest_whitelisted_slot += 1;

    msg!("Whitelisted message {:?} sent to: {:?}. New whitelisted slot: {:?}", slot.message, slot.to, inbox.latest_whitelisted_slot);
    Ok(())
}

/// Reclaims a slot.
pub fn reclaim_slot(ctx: Context<ReclaimSlot>) -> Result<()> {
    msg!("Slot reclaimed: {:?}", ctx.accounts.slot.key());
    Ok(())
}

/// Accounts required for sending a regular message.
#[derive(Accounts)]
#[instruction(message: String)]
pub struct SendRegularMessage<'info> {
    #[account(mut)]
    pub inbox: Account<'info, Inbox>,
    #[account(
        init,
        seeds=[&inbox.latest_free_slot.to_le_bytes()],
        bump,
        payer = sender,
        space = SLOT_BASE_SPACE + message.len()
    )]
    pub slot: Account<'info, Slot>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts required for sending a whitelisted message.
#[derive(Accounts)]
pub struct SendWhitelistedMessage<'info> {
    #[account(mut)]
    pub inbox: Account<'info, Inbox>,
    #[account(
        mut,
        seeds=[&inbox.latest_whitelisted_slot.to_le_bytes()],
        bump,
        // ensure whitelisted messages can only be sent to slots that have been allocated by the inbox
        constraint = inbox.latest_whitelisted_slot < inbox.latest_free_slot,
        // DEV: If we realloc here, any message that is smaller than the already allocated space would result in a refund to the payer.
        // This would create some weird incentives where anyone whitelisted could be incentivised to send an empty message to get the refund.
        // Implementing realloc in the method execution allows us to make sure that the refund goes to the inbox admin instead of the sender
        // in cases where the message is smaller than the allocated space.
        //
        // realloc = 8 + 32 + 4 + message.len(),
        // realloc:payer = sender,
        // realloc::zero = false,
    )]
    pub slot: Account<'info, Slot>,
    #[account(seeds=[sender.key().as_ref()], bump)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts required for reclaiming a slot.
#[derive(Accounts)]
pub struct ReclaimSlot<'info> {
    #[account(mut, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    #[account(mut, close = admin)]
    pub slot: Account<'info, Slot>,
    #[account(mut)]
    pub admin: Signer<'info>,
}