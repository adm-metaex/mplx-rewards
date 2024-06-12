use crate::state::{InitRewardPoolParams, RewardPool, RewardVault};
use crate::utils::{
    assert_account_key, create_account, find_reward_pool_program_address,
    find_vault_program_address, initialize_account, AccountLoader,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, system_program, sysvar::SysvarId,
};

use spl_token::state::Account as SplTokenAccount;

/// Instruction context
pub struct InitializePoolContext<'a, 'b> {
    reward_pool: &'a AccountInfo<'b>,
    reward_mint: &'a AccountInfo<'b>,
    reward_vault: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    rent: &'a AccountInfo<'b>,
}

impl<'a, 'b> InitializePoolContext<'a, 'b> {
    /// New instruction context
    pub fn new(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<InitializePoolContext<'a, 'b>, ProgramError> {
        let account_info_iter = &mut accounts.iter().enumerate();

        let reward_pool = AccountLoader::next_uninitialized(account_info_iter)?;
        let reward_mint = AccountLoader::next_with_owner(account_info_iter, &spl_token::id())?;
        let reward_vault = AccountLoader::next_uninitialized(account_info_iter)?;
        let payer = AccountLoader::next_with_owner(account_info_iter, &system_program::ID)?;
        let rent = AccountLoader::next_with_key(account_info_iter, &Rent::id())?;
        let _token_program = AccountLoader::next_with_key(account_info_iter, &spl_token::id())?;
        let _system_program =
            AccountLoader::next_with_key(account_info_iter, &system_program::id())?;

        Ok(InitializePoolContext {
            reward_pool,
            reward_mint,
            reward_vault,
            payer,
            rent,
        })
    }

    /// Process instruction
    pub fn process(
        &self,
        program_id: &Pubkey,
        deposit_authority: Pubkey,
        fill_authority: Pubkey,
        distribute_authority: Pubkey,
    ) -> ProgramResult {
        let (reward_pool_pubkey, pool_bump) =
            find_reward_pool_program_address(program_id, &deposit_authority, &fill_authority);
        assert_account_key(self.reward_pool, &reward_pool_pubkey)?;

        let reward_pool_seeds = &[
            "reward_pool".as_bytes(),
            &deposit_authority.to_bytes(),
            &fill_authority.to_bytes(),
            &[pool_bump],
        ];

        let (vault_pubkey, vault_bump) =
            find_vault_program_address(program_id, self.reward_pool.key, self.reward_mint.key);
        assert_account_key(self.reward_vault, &vault_pubkey)?;
        let vault_seeds = &[
            b"vault".as_ref(),
            self.reward_pool.key.as_ref(),
            self.reward_mint.key.as_ref(),
            &[vault_bump],
        ];

        create_account::<RewardPool>(
            program_id,
            self.payer.clone(),
            self.reward_pool.clone(),
            &[reward_pool_seeds],
        )?;

        create_account::<SplTokenAccount>(
            &spl_token::id(),
            self.payer.clone(),
            self.reward_vault.clone(),
            &[vault_seeds],
        )?;
        initialize_account(
            self.reward_vault.clone(),
            self.reward_mint.clone(),
            self.reward_pool.clone(),
            self.rent.clone(),
        )?;

        let reward_pool = RewardPool::init(InitRewardPoolParams {
            bump: pool_bump,
            deposit_authority,
            fill_authority,
            distribute_authority,
            vault: RewardVault {
                bump: vault_bump,
                reward_mint: *self.reward_mint.key,
                ..Default::default()
            },
        });
        RewardPool::pack(reward_pool, *self.reward_pool.data.borrow_mut())?;

        Ok(())
    }
}
