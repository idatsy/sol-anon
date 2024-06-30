use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("73o3wfunMxS44tEuia2D4WJKmJtqRewX3F4tG2fmkSEV");

#[program]
pub mod sol_anon {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
        instructions::inbox::initialize(ctx)
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::inbox::change_admin(ctx, new_admin)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, address_to_whitelist: Pubkey) -> Result<()> {
        instructions::whitelist::add_to_whitelist(ctx, address_to_whitelist)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, address_to_remove: Pubkey) -> Result<()> {
        instructions::whitelist::remove_from_whitelist(ctx, address_to_remove)
    }

    pub fn send_regular_message(ctx: Context<SendRegularMessage>, message: String, to: Pubkey) -> Result<()> {
        instructions::messages::send_regular_message(ctx, message, to)
    }

    pub fn send_whitelisted_message(ctx: Context<SendWhitelistedMessage>, message: String, to: Pubkey) -> Result<()> {
        instructions::messages::send_whitelisted_message(ctx, message, to)
    }

    pub fn reclaim_slot(ctx: Context<ReclaimSlot>) -> Result<()> {
        instructions::messages::reclaim_slot(ctx)
    }

    pub fn withdraw_surplus_inbox_balance(ctx: Context<WithdrawSurplusInboxBalance>) -> Result<()> {
        instructions::inbox::withdraw_surplus_inbox_balance(ctx)
    }
}