//! State types

mod mining;
mod reward_pool;
mod reward_vault;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
pub use mining::*;
pub use reward_pool::*;
pub use reward_vault::*;

/// Enum representing the account type managed by the program
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema, Default)]
pub enum AccountType {
    /// If the account has not been initialized, the enum will be 0
    #[default]
    Uninitialized,
    /// Reward pool
    RewardPool,
    /// Mining Account
    Mining,
    /// RewardVault
    RewardVault,
}
