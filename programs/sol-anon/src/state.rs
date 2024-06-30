use anchor_lang::prelude::*;

#[account]
pub struct Inbox {
    pub admin: Pubkey,
    pub latest_free_slot: u64,
    pub latest_whitelisted_slot: u64,
}

#[account]
pub struct Slot {
    pub to: Pubkey,
    pub message: String,
}

#[account]
pub struct Whitelist {}