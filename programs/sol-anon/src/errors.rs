use anchor_lang::prelude::*;

#[error_code]
pub enum SolAnonError {
    #[msg("Invalid admin")]
    InvalidAdmin,
    #[msg("Invalid whitelist")]
    InvalidWhitelist,
    #[msg("Invalid slot")]
    InvalidSlot,
}