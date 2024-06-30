//! # Instructions
//!
//! This module contains all the instruction handlers for the Sol-Anon program.
//!
//! ## Submodules
//!
//! - `inbox`: Handles inbox-related instructions.
//! - `messages`: Handles message-related instructions.
//! - `whitelist`: Handles whitelist-related instructions.

pub mod inbox;
pub mod messages;
pub mod whitelist;

pub use inbox::*;
pub use messages::*;
pub use whitelist::*;