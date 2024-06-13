use crate::error::MplxRewardsError;
use crate::state::{reward_vault::RewardVault, AccountType, Mining};
use crate::utils::{get_curr_unix_ts, LockupPeriod};
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
    /// Account type - RewardPool. This discriminator should exist in order to prevent
    /// shenanigans with customly modified accounts and their fields.
    pub account_type: AccountType,
    /// Saved bump for reward pool account
    pub bump: u8,
    /// The total share of the pool for the moment of the last distribution.
    /// It's so-called "weighted_stake" which is the sum of all stakers' weighted staked.
    /// When somebody deposits or withdraws, or thier stake is expired this value changes.
    pub total_share: u64,
    /// Vault which is responsible for storing rewards.
    pub vault: RewardVault,
    /// This address is the authority from the staking contract.
    /// We want to be sure that some changes might only be done through the
    /// staking contract. It's PDA from staking that will sign transactions.
    pub deposit_authority: Pubkey,
    /// This address is responsible for distributing rewards
    pub distribute_authority: Pubkey,
    /// The address is responsible for filling vaults with money.
    pub fill_authority: Pubkey,
}

impl RewardPool {
    /// Init reward pool
    pub fn init(params: InitRewardPoolParams) -> RewardPool {
        RewardPool {
            account_type: AccountType::RewardPool,
            bump: params.bump,
            total_share: 0,
            vault: params.vault,
            deposit_authority: params.deposit_authority,
            distribute_authority: params.distribute_authority,
            fill_authority: params.fill_authority,
        }
    }

    /// Process fill
    pub fn fill(&mut self, rewards: u64) -> ProgramResult {
        if self.total_share == 0 {
            return Err(MplxRewardsError::RewardsNoDeposits.into());
        }

        let curr_ts = Clock::get().unwrap().unix_timestamp as u64;
        let beginning_of_the_day = curr_ts - (curr_ts % SECONDS_PER_DAY);

        self.total_share = self
            .vault
            .consume_old_modifiers(beginning_of_the_day, self.total_share)?;
        if self
            .vault
            .cumulative_index
            .contains_key(&beginning_of_the_day)
        {
            return Ok(());
        }

        RewardVault::update_index(
            &mut self.vault.cumulative_index,
            &mut self.vault.index_with_precision,
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
    ) -> ProgramResult {
        mining.refresh_rewards(&self.vault)?;

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

        let modifier = self
            .vault
            .weighted_stake_diffs
            .entry(lockup_period.end_timestamp(get_curr_unix_ts())?)
            .or_default();
        *modifier = modifier
            .checked_add(weighted_stake_diff)
            .ok_or(MplxRewardsError::MathOverflow)?;

        let modifier = mining
            .index
            .weighted_stake_diffs
            .entry(lockup_period.end_timestamp(get_curr_unix_ts())?)
            .or_default();
        *modifier = modifier
            .checked_add(weighted_stake_diff)
            .ok_or(MplxRewardsError::MathOverflow)?;

        Ok(())
    }

    /// Process withdraw
    pub fn withdraw(&mut self, mining: &mut Mining, amount: u64) -> ProgramResult {
        mining.refresh_rewards(&self.vault)?;

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

    /// Process restake deposit
    pub fn restake(
        &mut self,
        mining: &mut Mining,
        old_lockup_period: LockupPeriod,
        new_lockup_period: LockupPeriod,
        deposit_start_ts: u64,
        base_amount: u64,
        additional_amount: u64,
    ) -> ProgramResult {
        mining.refresh_rewards(&self.vault)?;

        let curr_ts = get_curr_unix_ts();

        let deposit_old_expiration_ts = if old_lockup_period == LockupPeriod::Flex {
            0 // it's expired, so the date is in the past
        } else {
            old_lockup_period.end_timestamp(deposit_start_ts)?
        };

        // curr_part_of_weighted_stake_for_flex = old_base_amount * flex_multipler
        let curr_part_of_weighted_stake_for_flex = base_amount
            .checked_mul(LockupPeriod::Flex.multiplier())
            .ok_or(MplxRewardsError::MathOverflow)?;

        // if current date is lower than stake expiration date, we need to
        // remove stake modifier from the date of expiration
        if curr_ts < deposit_old_expiration_ts {
            // current_part_of_weighted_stake =
            let curr_part_of_weighted_stake = base_amount
                .checked_mul(old_lockup_period.multiplier())
                .ok_or(MplxRewardsError::MathOverflow)?;

            // weighted_stake_modifier_to_remove = old_base_amount * lockup_period_multiplier - amount_times_flex
            let weighted_stake_diff = curr_part_of_weighted_stake
                .checked_sub(curr_part_of_weighted_stake_for_flex)
                .ok_or(MplxRewardsError::MathOverflow)?;

            self.vault
                .weighted_stake_diffs
                .entry(deposit_old_expiration_ts)
                .and_modify(|modifier| *modifier -= weighted_stake_diff);

            mining
                .index
                .weighted_stake_diffs
                .entry(deposit_old_expiration_ts)
                .and_modify(|modifier| *modifier -= weighted_stake_diff);

            // also, we need to reduce staking power because we want to restake from "scratch"
            mining.share = mining
                .share
                .checked_sub(curr_part_of_weighted_stake)
                .ok_or(MplxRewardsError::MathOverflow)?;

            self.total_share = self
                .total_share
                .checked_sub(curr_part_of_weighted_stake)
                .ok_or(MplxRewardsError::MathOverflow)?;
        } else {
            // otherwise, we want to substract flex multiplier, becase deposit has expired already
            mining.share = mining
                .share
                .checked_sub(curr_part_of_weighted_stake_for_flex)
                .ok_or(MplxRewardsError::MathOverflow)?;

            self.total_share = self
                .total_share
                .checked_sub(curr_part_of_weighted_stake_for_flex)
                .ok_or(MplxRewardsError::MathOverflow)?;
        }

        // do actions like it's a regular deposit
        let amount_to_restake = base_amount
            .checked_add(additional_amount)
            .ok_or(MplxRewardsError::MathOverflow)?;
        self.deposit(mining, amount_to_restake, new_lockup_period)?;

        Ok(())
    }
}

/// Initialize a Reward Pool params
pub struct InitRewardPoolParams {
    /// Saved bump for reward pool account
    pub bump: u8,
    /// This address is the authority of from the staking contract.
    /// We want to be sure that some changes might only be done through the
    /// staking contract. It's PDA from staking that will sign transactions.
    pub deposit_authority: Pubkey,
    /// The address responsible for the filling vaults with rewards.
    /// Those rewards later will be used to distribute rewards.
    pub fill_authority: Pubkey,
    /// This vault will be responsible for storing rewards
    pub vault: RewardVault,
    /// This account is responsible for periodical distribution of rewards
    pub distribute_authority: Pubkey,
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
