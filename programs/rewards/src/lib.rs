// #![deny(missing_docs)]

//! Rewards contract

pub mod asserts;
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod instructions;
pub mod state;
pub mod utils;

pub use solana_program;

solana_program::declare_id!("BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR");
