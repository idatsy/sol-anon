//! # Whitelist Instructions
//!
//! This module contains instructions related to managing the whitelist.

use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

/// Adds an address to the whitelist.
pub fn add_to_whitelist(_ctx: Context<AddToWhitelist>, address_to_whitelist: Pubkey) -> Result<()> {
    msg!("Adding address to whitelist: {:?}", address_to_whitelist);
    Ok(())
}

/// Removes an address from the whitelist.
pub fn remove_from_whitelist(_ctx: Context<RemoveFromWhitelist>, address_to_remove: Pubkey) -> Result<()> {
    msg!("Removing address from whitelist: {:?}", address_to_remove);
    Ok(())
}

/// Accounts required for adding an address to the whitelist.
#[derive(Accounts)]
#[instruction(address_to_whitelist: Pubkey)]
pub struct AddToWhitelist<'info> {
    // TODO: If time allows, don't use anchor so we can skip the 8 byte discriminant and save another 7 bytes of space!
    // Also would be nice to have a better understanding of PDA initialization, for example seeding a PDA with rent excemption alone is not secure
    // as there *could* be a way to to send money to a non-existing PDA even though it hasn't been created by this admin.
    // Either way due to time contsraints this probably won't be possible to do with Anchor but interesting to explore using a native implementation
    #[account(
        init,
        seeds=[address_to_whitelist.key().as_ref()],
        bump,
        payer = admin,
        space = WHITELIST_SPACE
    )]
    pub whitelist_pda: Account<'info, Whitelist>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts required for removing an address from the whitelist.
#[derive(Accounts)]
#[instruction(address_to_remove: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut, seeds=[address_to_remove.key().as_ref()], bump, close = admin)]
    pub whitelist_pda: Account<'info, Whitelist>,
    #[account(mut)]
    pub admin: Signer<'info>,
}