use crate::{
    asserts::{assert_account_key, verify_delegate_mining_requirements},
    state::{Mining, RewardPool},
    utils::{find_mining_program_address, AccountLoader, LockupPeriod},
};
use solana_program::{
    account_info::AccountInfo, clock::SECONDS_PER_DAY, entrypoint::ProgramResult, msg,
    program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};

/// Instruction context
pub struct ExtendStakeContext<'a, 'b> {
    reward_pool: &'a AccountInfo<'b>,
    mining: &'a AccountInfo<'b>,
    deposit_authority: &'a AccountInfo<'b>,
    delegate_mining: &'a AccountInfo<'b>,
}

impl<'a, 'b> ExtendStakeContext<'a, 'b> {
    /// New instruction context
    pub fn new(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<ExtendStakeContext<'a, 'b>, ProgramError> {
        let account_info_iter = &mut accounts.iter().enumerate();

        let reward_pool = AccountLoader::next_with_owner(account_info_iter, program_id)?;
        let mining = AccountLoader::next_with_owner(account_info_iter, program_id)?;
        let deposit_authority = AccountLoader::next_signer(account_info_iter)?;
        let delegate_mining = AccountLoader::next_with_owner(account_info_iter, program_id)?;

        Ok(ExtendStakeContext {
            reward_pool,
            mining,
            deposit_authority,
            delegate_mining,
        })
    }

    /// Process instruction
    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &self,
        program_id: &Pubkey,
        old_lockup_period: LockupPeriod,
        new_lockup_period: LockupPeriod,
        deposit_start_ts: u64,
        base_amount: u64,
        additional_amount: u64,
        mining_owner: &Pubkey,
    ) -> ProgramResult {
        let mut reward_pool = RewardPool::unpack(&self.reward_pool.data.borrow())?;
        let mut mining = Mining::unpack(&self.mining.data.borrow())?;
        let deposit_start_ts = deposit_start_ts - (deposit_start_ts % SECONDS_PER_DAY);

        {
            let (mining_pubkey, _) =
                find_mining_program_address(program_id, mining_owner, self.reward_pool.key);
            assert_account_key(self.mining, &mining_pubkey)?;
            assert_account_key(self.deposit_authority, &reward_pool.deposit_authority)?;
            assert_account_key(self.reward_pool, &mining.reward_pool)?;
            if mining_owner != &mining.owner {
                msg!(
                    "Assert account error. Got {} Expected {}",
                    *mining_owner,
                    mining.owner
                );
                return Err(ProgramError::InvalidArgument);
            }
        }

        let mut delegate_mining =
            verify_delegate_mining_requirements(self.delegate_mining, self.mining)?;

        reward_pool.extend(
            &mut mining,
            old_lockup_period,
            new_lockup_period,
            deposit_start_ts,
            base_amount,
            additional_amount,
            delegate_mining.as_mut(),
        )?;

        RewardPool::pack(reward_pool, *self.reward_pool.data.borrow_mut())?;
        Mining::pack(mining, *self.mining.data.borrow_mut())?;

        if let Some(delegate_mining) = delegate_mining {
            Mining::pack(delegate_mining, *self.delegate_mining.data.borrow_mut())?;
        }

        Ok(())
    }
}
