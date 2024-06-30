//! # Errors
//!
//! This module defines custom error types for the Sol-Anon program.

use anchor_lang::prelude::*;

/// Custom error types for the Sol-Anon program.
#[error_code]
pub enum SolAnonError {
    /// Thrown when an invalid admin tries to perform an admin-only action.
    #[msg("Invalid admin")]
    InvalidAdmin,
    /// Thrown when an operation is attempted with an invalid whitelist account.
    #[msg("Invalid whitelist")]
    InvalidWhitelist,
    /// Thrown when an operation is attempted with an invalid slot account.
    #[msg("Invalid slot")]
    InvalidSlot,
}