pub const INBOX_SEED: &[u8] = b"inbox";
pub const INBOX_SPACE: usize = 8 + 32 + 8 + 8; // discriminator + pubkey + 2 * u64
pub const WHITELIST_SPACE: usize = 8; // discriminator
pub const SLOT_BASE_SPACE: usize = 8 + 32 + 4; // discriminator + pubkey + String prefix