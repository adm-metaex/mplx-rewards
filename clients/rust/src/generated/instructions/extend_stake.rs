//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::LockupPeriod;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct ExtendStake {
    /// The address of the reward pool
    pub reward_pool: solana_program::pubkey::Pubkey,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: solana_program::pubkey::Pubkey,
    /// The address of the reward mint
    pub reward_mint: solana_program::pubkey::Pubkey,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: solana_program::pubkey::Pubkey,
}

impl ExtendStake {
    pub fn instruction(
        &self,
        args: ExtendStakeInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ExtendStakeInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reward_pool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mining,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.deposit_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = ExtendStakeInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPLX_REWARDS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ExtendStakeInstructionData {
    discriminator: u8,
}

impl ExtendStakeInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 6 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendStakeInstructionArgs {
    pub old_lockup_period: LockupPeriod,
    pub new_lockup_period: LockupPeriod,
    pub deposit_start_ts: u64,
    pub base_amount: u64,
    pub additional_amount: u64,
    pub mining_owner: Pubkey,
}

/// Instruction builder for `ExtendStake`.
///
/// ### Accounts:
///
///   0. `[writable]` reward_pool
///   1. `[writable]` mining
///   2. `[]` reward_mint
///   3. `[signer]` deposit_authority
#[derive(Default)]
pub struct ExtendStakeBuilder {
    reward_pool: Option<solana_program::pubkey::Pubkey>,
    mining: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    deposit_authority: Option<solana_program::pubkey::Pubkey>,
    old_lockup_period: Option<LockupPeriod>,
    new_lockup_period: Option<LockupPeriod>,
    deposit_start_ts: Option<u64>,
    base_amount: Option<u64>,
    additional_amount: Option<u64>,
    mining_owner: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ExtendStakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The address of the reward pool
    #[inline(always)]
    pub fn reward_pool(&mut self, reward_pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_pool = Some(reward_pool);
        self
    }
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    #[inline(always)]
    pub fn mining(&mut self, mining: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mining = Some(mining);
        self
    }
    /// The address of the reward mint
    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    #[inline(always)]
    pub fn deposit_authority(
        &mut self,
        deposit_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.deposit_authority = Some(deposit_authority);
        self
    }
    #[inline(always)]
    pub fn old_lockup_period(&mut self, old_lockup_period: LockupPeriod) -> &mut Self {
        self.old_lockup_period = Some(old_lockup_period);
        self
    }
    #[inline(always)]
    pub fn new_lockup_period(&mut self, new_lockup_period: LockupPeriod) -> &mut Self {
        self.new_lockup_period = Some(new_lockup_period);
        self
    }
    #[inline(always)]
    pub fn deposit_start_ts(&mut self, deposit_start_ts: u64) -> &mut Self {
        self.deposit_start_ts = Some(deposit_start_ts);
        self
    }
    #[inline(always)]
    pub fn base_amount(&mut self, base_amount: u64) -> &mut Self {
        self.base_amount = Some(base_amount);
        self
    }
    #[inline(always)]
    pub fn additional_amount(&mut self, additional_amount: u64) -> &mut Self {
        self.additional_amount = Some(additional_amount);
        self
    }
    #[inline(always)]
    pub fn mining_owner(&mut self, mining_owner: Pubkey) -> &mut Self {
        self.mining_owner = Some(mining_owner);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ExtendStake {
            reward_pool: self.reward_pool.expect("reward_pool is not set"),
            mining: self.mining.expect("mining is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            deposit_authority: self
                .deposit_authority
                .expect("deposit_authority is not set"),
        };
        let args = ExtendStakeInstructionArgs {
            old_lockup_period: self
                .old_lockup_period
                .clone()
                .expect("old_lockup_period is not set"),
            new_lockup_period: self
                .new_lockup_period
                .clone()
                .expect("new_lockup_period is not set"),
            deposit_start_ts: self
                .deposit_start_ts
                .clone()
                .expect("deposit_start_ts is not set"),
            base_amount: self.base_amount.clone().expect("base_amount is not set"),
            additional_amount: self
                .additional_amount
                .clone()
                .expect("additional_amount is not set"),
            mining_owner: self.mining_owner.clone().expect("mining_owner is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `extend_stake` CPI accounts.
pub struct ExtendStakeCpiAccounts<'a, 'b> {
    /// The address of the reward pool
    pub reward_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `extend_stake` CPI instruction.
pub struct ExtendStakeCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward pool
    pub reward_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ExtendStakeInstructionArgs,
}

impl<'a, 'b> ExtendStakeCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ExtendStakeCpiAccounts<'a, 'b>,
        args: ExtendStakeInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            reward_pool: accounts.reward_pool,
            mining: accounts.mining,
            reward_mint: accounts.reward_mint,
            deposit_authority: accounts.deposit_authority,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reward_pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mining.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.deposit_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = ExtendStakeInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPLX_REWARDS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(4 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.reward_pool.clone());
        account_infos.push(self.mining.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.deposit_authority.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ExtendStake` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` reward_pool
///   1. `[writable]` mining
///   2. `[]` reward_mint
///   3. `[signer]` deposit_authority
pub struct ExtendStakeCpiBuilder<'a, 'b> {
    instruction: Box<ExtendStakeCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ExtendStakeCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ExtendStakeCpiBuilderInstruction {
            __program: program,
            reward_pool: None,
            mining: None,
            reward_mint: None,
            deposit_authority: None,
            old_lockup_period: None,
            new_lockup_period: None,
            deposit_start_ts: None,
            base_amount: None,
            additional_amount: None,
            mining_owner: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The address of the reward pool
    #[inline(always)]
    pub fn reward_pool(
        &mut self,
        reward_pool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_pool = Some(reward_pool);
        self
    }
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    #[inline(always)]
    pub fn mining(
        &mut self,
        mining: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mining = Some(mining);
        self
    }
    /// The address of the reward mint
    #[inline(always)]
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    #[inline(always)]
    pub fn deposit_authority(
        &mut self,
        deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.deposit_authority = Some(deposit_authority);
        self
    }
    #[inline(always)]
    pub fn old_lockup_period(&mut self, old_lockup_period: LockupPeriod) -> &mut Self {
        self.instruction.old_lockup_period = Some(old_lockup_period);
        self
    }
    #[inline(always)]
    pub fn new_lockup_period(&mut self, new_lockup_period: LockupPeriod) -> &mut Self {
        self.instruction.new_lockup_period = Some(new_lockup_period);
        self
    }
    #[inline(always)]
    pub fn deposit_start_ts(&mut self, deposit_start_ts: u64) -> &mut Self {
        self.instruction.deposit_start_ts = Some(deposit_start_ts);
        self
    }
    #[inline(always)]
    pub fn base_amount(&mut self, base_amount: u64) -> &mut Self {
        self.instruction.base_amount = Some(base_amount);
        self
    }
    #[inline(always)]
    pub fn additional_amount(&mut self, additional_amount: u64) -> &mut Self {
        self.instruction.additional_amount = Some(additional_amount);
        self
    }
    #[inline(always)]
    pub fn mining_owner(&mut self, mining_owner: Pubkey) -> &mut Self {
        self.instruction.mining_owner = Some(mining_owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = ExtendStakeInstructionArgs {
            old_lockup_period: self
                .instruction
                .old_lockup_period
                .clone()
                .expect("old_lockup_period is not set"),
            new_lockup_period: self
                .instruction
                .new_lockup_period
                .clone()
                .expect("new_lockup_period is not set"),
            deposit_start_ts: self
                .instruction
                .deposit_start_ts
                .clone()
                .expect("deposit_start_ts is not set"),
            base_amount: self
                .instruction
                .base_amount
                .clone()
                .expect("base_amount is not set"),
            additional_amount: self
                .instruction
                .additional_amount
                .clone()
                .expect("additional_amount is not set"),
            mining_owner: self
                .instruction
                .mining_owner
                .clone()
                .expect("mining_owner is not set"),
        };
        let instruction = ExtendStakeCpi {
            __program: self.instruction.__program,

            reward_pool: self
                .instruction
                .reward_pool
                .expect("reward_pool is not set"),

            mining: self.instruction.mining.expect("mining is not set"),

            reward_mint: self
                .instruction
                .reward_mint
                .expect("reward_mint is not set"),

            deposit_authority: self
                .instruction
                .deposit_authority
                .expect("deposit_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ExtendStakeCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    reward_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mining: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    deposit_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    old_lockup_period: Option<LockupPeriod>,
    new_lockup_period: Option<LockupPeriod>,
    deposit_start_ts: Option<u64>,
    base_amount: Option<u64>,
    additional_amount: Option<u64>,
    mining_owner: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
