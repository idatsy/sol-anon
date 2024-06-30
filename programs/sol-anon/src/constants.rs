//! # Constants
//!
//! This module defines constant values used throughout the Sol-Anon program.

/// Seed for deriving the inbox PDA.
pub const INBOX_SEED: &[u8] = b"inbox";

/// Space allocated for the inbox account.
/// Breakdown:
/// - 8 bytes for the account discriminator
/// - 32 bytes for the admin public key
/// - 8 bytes for the latest_free_slot
/// - 8 bytes for the latest_whitelisted_slot
pub const INBOX_SPACE: usize = 8 + 32 + 8 + 8;

/// Space allocated for the whitelist account.
/// Only includes the 8-byte discriminator as the account is empty.
pub const WHITELIST_SPACE: usize = 8;

/// Base space allocated for each slot account.
/// Breakdown:
/// - 8 bytes for the account discriminator
/// - 32 bytes for the recipient's public key
/// - 4 bytes for the String prefix (length of the message)
pub const SLOT_BASE_SPACE: usize = 8 + 32 + 4;