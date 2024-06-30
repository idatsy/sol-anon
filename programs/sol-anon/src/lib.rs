//! # Sol-Anon
//!
//! Sol-Anon is a Solana program that implements an anonymous messaging system with whitelisting capabilities.
//!
//! ## Overview
//!
//! This program allows users to send messages anonymously, with special privileges for whitelisted users.
//! It manages an inbox, message slots, and a whitelist of users.
//!
//! ## Modules
//!
//! - `constants`: Defines constant values used throughout the program.
//! - `errors`: Defines custom error types for the program.
//! - `instructions`: Contains the main logic for each instruction the program can execute.
//! - `state`: Defines the account structures used in the program.
//! - `utils`: Contains utility functions used across the program.

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("73o3wfunMxS44tEuia2D4WJKmJtqRewX3F4tG2fmkSEV");

/// The main program module containing all instruction handlers.
#[program]
pub mod sol_anon {
    use super::*;

    /// Initializes a new inbox.
    pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
        instructions::inbox::initialize(ctx)
    }

    /// Changes the admin of the inbox.
    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::inbox::change_admin(ctx, new_admin)
    }

    /// Adds an address to the whitelist.
    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, address_to_whitelist: Pubkey) -> Result<()> {
        instructions::whitelist::add_to_whitelist(ctx, address_to_whitelist)
    }

    /// Removes an address from the whitelist.
    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, address_to_remove: Pubkey) -> Result<()> {
        instructions::whitelist::remove_from_whitelist(ctx, address_to_remove)
    }

    /// Sends a regular message.
    pub fn send_regular_message(ctx: Context<SendRegularMessage>, message: String, to: Pubkey) -> Result<()> {
        instructions::messages::send_regular_message(ctx, message, to)
    }

    /// Sends a whitelisted message.
    pub fn send_whitelisted_message(ctx: Context<SendWhitelistedMessage>, message: String, to: Pubkey) -> Result<()> {
        instructions::messages::send_whitelisted_message(ctx, message, to)
    }

    /// Reclaims a slot.
    pub fn reclaim_slot(ctx: Context<ReclaimSlot>) -> Result<()> {
        instructions::messages::reclaim_slot(ctx)
    }

    /// Withdraws surplus balance from the inbox.
    pub fn withdraw_surplus_inbox_balance(ctx: Context<WithdrawSurplusInboxBalance>) -> Result<()> {
        instructions::inbox::withdraw_surplus_inbox_balance(ctx)
    }
}