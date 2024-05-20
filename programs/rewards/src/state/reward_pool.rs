use std::collections::BTreeMap;

use crate::error::MplxRewardsError;
use crate::state::{AccountType, Mining};
use crate::utils::LockupPeriod;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    clock::{Clock, SECONDS_PER_DAY},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    sysvar::Sysvar,
};

/// Precision for index calculation
pub const PRECISION: u128 = 10_000_000_000_000_000;
/// Max reward vaults
pub const MAX_REWARDS: usize = 5;

/// Ring buffer capacity
pub const RINGBUF_CAP: usize = 365;

/// Reward pool
#[derive(Debug, BorshDeserialize, BorshSerialize, BorshSchema, Default)]
pub struct RewardPool {
    /// Account type - RewardPool
    pub account_type: AccountType,
    /// Rewards root account (ex-Config program account)
    pub rewards_root: Pubkey,
    /// Saved bump for reward pool account
    pub bump: u8,
    /// Liquidity mint
    pub liquidity_mint: Pubkey,
    /// Reward total share
    pub total_share: u64,
    /// A set of all possible rewards that we can get for this pool
    pub vaults: Vec<RewardVault>,
    /// The address responsible for the charge of rewards for users.
    /// It executes deposits on the rewards pools.
    pub deposit_authority: Pubkey,
}

impl RewardPool {
    /// Init reward pool
    pub fn init(params: InitRewardPoolParams) -> RewardPool {
        RewardPool {
            account_type: AccountType::RewardPool,
            rewards_root: params.rewards_root,
            bump: params.bump,
            liquidity_mint: params.liquidity_mint,
            total_share: 0,
            vaults: vec![],
            deposit_authority: params.deposit_authority,
        }
    }

    /// Process add vault
    pub fn add_vault(&mut self, reward: RewardVault) -> ProgramResult {
        if self
            .vaults
            .iter()
            .any(|v| v.reward_mint == reward.reward_mint)
        {
            return Err(ProgramError::InvalidArgument);
        }

        self.vaults.push(reward);

        Ok(())
    }

    /// Process fill
    pub fn fill(&mut self, reward_mint: Pubkey, rewards: u64) -> ProgramResult {
        if self.total_share == 0 {
            return Err(MplxRewardsError::RewardsNoDeposits.into());
        }

        let curr_ts = Clock::get().unwrap().unix_timestamp as u64;
        let beginning_of_the_day = curr_ts - (curr_ts % SECONDS_PER_DAY);

        let vault = self
            .vaults
            .iter_mut()
            .find(|v| v.reward_mint == reward_mint)
            .ok_or(MplxRewardsError::RewardsInvalidVault)?;

        self.total_share =
            vault.consume_old_modifiers(beginning_of_the_day, self.total_share, rewards)?;
        if vault.cumulative_index.contains_key(&beginning_of_the_day) {
            return Ok(());
        }

        RewardVault::update_index(
            &mut vault.cumulative_index,
            &mut vault.index_with_precision,
            rewards,
            self.total_share,
            beginning_of_the_day,
        )?;

        Ok(())
    }

    /// Process deposit
    pub fn deposit(
        &mut self,
        mining: &mut Mining,
        amount: u64,
        lockup_period: LockupPeriod,
        reward_mint: &Pubkey,
    ) -> ProgramResult {
        // regular weighted stake which will be used in rewards distribution
        let weighted_stake = amount
            .checked_mul(lockup_period.multiplier())
            .ok_or(MplxRewardsError::MathOverflow)?;

        // shows how weighted stake will change at the end of the staking period
        // weighted_stake_diff = weighted_stake - (amount * flex_multiplier)
        let weighted_stake_diff = weighted_stake
            .checked_sub(
                amount
                    .checked_mul(LockupPeriod::Flex.multiplier())
                    .ok_or(MplxRewardsError::MathOverflow)?,
            )
            .ok_or(MplxRewardsError::MathOverflow)?;

        self.total_share = self
            .total_share
            .checked_add(weighted_stake)
            .ok_or(MplxRewardsError::MathOverflow)?;

        mining.share = mining
            .share
            .checked_add(weighted_stake)
            .ok_or(MplxRewardsError::MathOverflow)?;

        let vault = self
            .vaults
            .iter_mut()
            .find(|v| v.reward_mint == *reward_mint)
            .ok_or(MplxRewardsError::RewardsInvalidVault)?;
        let modifier = vault
            .weighted_stake_diffs
            .entry(lockup_period.end_timestamp()?)
            .or_default();
        *modifier = modifier
            .checked_add(weighted_stake_diff)
            .ok_or(MplxRewardsError::MathOverflow)?;

        let reward_index = mining.reward_index_mut(*reward_mint);

        // Guard for the case when fill had been made before the deposit
        // in that case end-user shouldn't receive rewards for that day
        // TODO: check if it's safe to do, because in case user has valid unclaimed
        // rewards and user's diffs are empty for some reason (e.g. consumed on the previous refresh rewards calculations),
        //the reward will be lost
        if reward_index.weighted_stake_diffs.is_empty() {
            reward_index.index_with_precision = vault.index_with_precision;
        };

        let modifier = reward_index
            .weighted_stake_diffs
            .entry(lockup_period.end_timestamp()?)
            .or_default();
        *modifier = modifier
            .checked_add(weighted_stake_diff)
            .ok_or(MplxRewardsError::MathOverflow)?;

        mining.refresh_rewards(self.vaults.iter())?;

        Ok(())
    }

    /// Process withdraw
    pub fn withdraw(&mut self, mining: &mut Mining, amount: u64) -> ProgramResult {
        mining.refresh_rewards(self.vaults.iter())?;

        self.total_share = self
            .total_share
            .checked_sub(amount)
            .ok_or(MplxRewardsError::MathOverflow)?;
        mining.share = mining
            .share
            .checked_sub(amount)
            .ok_or(MplxRewardsError::MathOverflow)?;

        Ok(())
    }
}

/// Initialize a Reward Pool params
pub struct InitRewardPoolParams {
    /// Rewards Root (ex-Config program account)
    pub rewards_root: Pubkey,
    /// Saved bump for reward pool account
    pub bump: u8,
    /// Liquidity mint
    pub liquidity_mint: Pubkey,
    /// The address responsible for the charge of rewards for users.
    /// It executes deposits on the rewards pools.
    pub deposit_authority: Pubkey,
}

impl Sealed for RewardPool {}
impl Pack for RewardPool {
    // RewardPool size
    const LEN: usize = 1 + (32 + 1 + 32 + 8 + (4 + RewardVault::LEN) + 32);

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap()
    }

    fn unpack_from_slice(src: &[u8]) -> Result<RewardPool, ProgramError> {
        let mut src_mut = src;
        Self::deserialize(&mut src_mut).map_err(|err| {
            msg!("Failed to deserialize");
            msg!("{}", err.to_string());
            ProgramError::InvalidAccountData
        })
    }
}

impl IsInitialized for RewardPool {
    fn is_initialized(&self) -> bool {
        self.account_type == AccountType::RewardPool
    }
}

/// Reward vault
#[derive(Debug, BorshDeserialize, BorshSerialize, BorshSchema, Default)]
pub struct RewardVault {
    /// Bump of vault account
    pub bump: u8,
    /// Reward mint address
    pub reward_mint: Pubkey,
    /// Index with precision
    pub index_with_precision: u128,
    /// Weighted stake diffs is used to store the modifiers which will be applied to the total_share
    pub weighted_stake_diffs: BTreeMap<u64, u64>,
    /// Cumulative index per day. <Date, index>
    pub cumulative_index: BTreeMap<u64, u128>,
}

impl RewardVault {
    /// Reward Vault size
    /// TODO: size isn't large enough
    pub const LEN: usize = 1 + 32 + 16 + 32 + (4 + (8 + 8) * 100) + (4 + (8 + 16) * 100);

    /// Consuming old total share modifiers in order to change the total share for the current date
    pub fn consume_old_modifiers(
        &mut self,
        beginning_of_the_day: u64,
        mut total_share: u64,
        rewards: u64,
    ) -> Result<u64, ProgramError> {
        for (date_to_process, modifier) in self.weighted_stake_diffs.iter() {
            if date_to_process > &beginning_of_the_day {
                break;
            }

            total_share = total_share
                .checked_sub(*modifier)
                .ok_or(MplxRewardsError::MathOverflow)?;

            RewardVault::update_index(
                &mut self.cumulative_index,
                &mut self.index_with_precision,
                rewards,
                total_share,
                *date_to_process,
            )?;
        }
        // drop keys because they have been already consumed and no longer needed
        self.weighted_stake_diffs
            .retain(|date, _modifier| date > &beginning_of_the_day);
        Ok(total_share)
    }

    /// recalculates the index for the given rewards and total share
    pub fn update_index(
        cumulative_index: &mut BTreeMap<u64, u128>,
        index_with_precision: &mut u128,
        rewards: u64,
        total_share: u64,
        date_to_process: u64,
    ) -> ProgramResult {
        let index = PRECISION
            .checked_mul(rewards as u128)
            .ok_or(MplxRewardsError::MathOverflow)?
            .checked_div(total_share as u128)
            .ok_or(MplxRewardsError::MathOverflow)?;

        let cumulative_index_to_insert = {
            if let Some((_, index)) = cumulative_index.last_key_value() {
                index.to_owned()
            } else {
                0
            }
            .checked_add(index)
            .ok_or(MplxRewardsError::MathOverflow)?
        };

        cumulative_index.insert(date_to_process, cumulative_index_to_insert);

        *index_with_precision = index_with_precision
            .checked_add(index)
            .ok_or(MplxRewardsError::MathOverflow)?;

        Ok(())
    }
}
