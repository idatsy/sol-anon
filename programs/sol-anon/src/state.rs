//! # State
//!
//! This module defines the account structures used in the Sol-Anon program.

use anchor_lang::prelude::*;

/// Represents the inbox account structure.
#[account]
pub struct Inbox {
    /// The public key of the admin who controls this inbox.
    pub admin: Pubkey,
    /// The latest free slot index.
    pub latest_free_slot: u64,
    /// The latest whitelisted slot index.
    pub latest_whitelisted_slot: u64,
}

/// Represents a message slot account structure.
#[account]
pub struct Slot {
    /// The public key of the recipient of this message.
    pub to: Pubkey,
    /// The content of the message.
    pub message: String,
}

/// Represents a whitelist account structure.
/// This is currently empty as the existence of the account
/// itself indicates that the address is whitelisted.
#[account]
pub struct Whitelist {}