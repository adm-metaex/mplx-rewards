//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Claim {
    /// The address of the reward pool
    pub reward_pool: solana_program::pubkey::Pubkey,
    /// The address of the reward mint
    pub reward_mint: solana_program::pubkey::Pubkey,
    /// The address of the reward vault
    pub vault: solana_program::pubkey::Pubkey,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: solana_program::pubkey::Pubkey,
    /// The end user the mining accounts belongs to
    pub mining_owner: solana_program::pubkey::Pubkey,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: solana_program::pubkey::Pubkey,
    /// ATA where tokens will be claimed to
    pub mining_owner_reward_token_account: solana_program::pubkey::Pubkey,
    /// The address of the Token program where rewards are minted
    pub token_program: solana_program::pubkey::Pubkey,
}

impl Claim {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_pool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mining,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mining_owner,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.deposit_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mining_owner_reward_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ClaimInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MPLX_REWARDS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ClaimInstructionData {
    discriminator: u8,
}

impl ClaimInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 5 }
    }
}

/// Instruction builder for `Claim`.
///
/// ### Accounts:
///
///   0. `[]` reward_pool
///   1. `[]` reward_mint
///   2. `[writable]` vault
///   3. `[writable]` mining
///   4. `[signer]` mining_owner
///   5. `[signer]` deposit_authority
///   6. `[writable]` mining_owner_reward_token_account
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Default)]
pub struct ClaimBuilder {
    reward_pool: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    mining: Option<solana_program::pubkey::Pubkey>,
    mining_owner: Option<solana_program::pubkey::Pubkey>,
    deposit_authority: Option<solana_program::pubkey::Pubkey>,
    mining_owner_reward_token_account: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ClaimBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The address of the reward pool
    #[inline(always)]
    pub fn reward_pool(&mut self, reward_pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_pool = Some(reward_pool);
        self
    }
    /// The address of the reward mint
    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }
    /// The address of the reward vault
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    #[inline(always)]
    pub fn mining(&mut self, mining: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mining = Some(mining);
        self
    }
    /// The end user the mining accounts belongs to
    #[inline(always)]
    pub fn mining_owner(&mut self, mining_owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mining_owner = Some(mining_owner);
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
    /// ATA where tokens will be claimed to
    #[inline(always)]
    pub fn mining_owner_reward_token_account(
        &mut self,
        mining_owner_reward_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.mining_owner_reward_token_account = Some(mining_owner_reward_token_account);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// The address of the Token program where rewards are minted
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
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
        let accounts = Claim {
            reward_pool: self.reward_pool.expect("reward_pool is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            vault: self.vault.expect("vault is not set"),
            mining: self.mining.expect("mining is not set"),
            mining_owner: self.mining_owner.expect("mining_owner is not set"),
            deposit_authority: self
                .deposit_authority
                .expect("deposit_authority is not set"),
            mining_owner_reward_token_account: self
                .mining_owner_reward_token_account
                .expect("mining_owner_reward_token_account is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `claim` CPI accounts.
pub struct ClaimCpiAccounts<'a, 'b> {
    /// The address of the reward pool
    pub reward_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward vault
    pub vault: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: &'b solana_program::account_info::AccountInfo<'a>,
    /// The end user the mining accounts belongs to
    pub mining_owner: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// ATA where tokens will be claimed to
    pub mining_owner_reward_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Token program where rewards are minted
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `claim` CPI instruction.
pub struct ClaimCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward pool
    pub reward_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the reward vault
    pub vault: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the mining account which belongs to the user and stores info about user's rewards
    pub mining: &'b solana_program::account_info::AccountInfo<'a>,
    /// The end user the mining accounts belongs to
    pub mining_owner: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Staking program's Registrar, which is PDA and is responsible for signing CPIs
    pub deposit_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// ATA where tokens will be claimed to
    pub mining_owner_reward_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the Token program where rewards are minted
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ClaimCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ClaimCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            reward_pool: accounts.reward_pool,
            reward_mint: accounts.reward_mint,
            vault: accounts.vault,
            mining: accounts.mining,
            mining_owner: accounts.mining_owner,
            deposit_authority: accounts.deposit_authority,
            mining_owner_reward_token_account: accounts.mining_owner_reward_token_account,
            token_program: accounts.token_program,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mining.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mining_owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.deposit_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mining_owner_reward_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ClaimInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPLX_REWARDS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.reward_pool.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.mining.clone());
        account_infos.push(self.mining_owner.clone());
        account_infos.push(self.deposit_authority.clone());
        account_infos.push(self.mining_owner_reward_token_account.clone());
        account_infos.push(self.token_program.clone());
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

/// Instruction builder for `Claim` via CPI.
///
/// ### Accounts:
///
///   0. `[]` reward_pool
///   1. `[]` reward_mint
///   2. `[writable]` vault
///   3. `[writable]` mining
///   4. `[signer]` mining_owner
///   5. `[signer]` deposit_authority
///   6. `[writable]` mining_owner_reward_token_account
///   7. `[]` token_program
pub struct ClaimCpiBuilder<'a, 'b> {
    instruction: Box<ClaimCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ClaimCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ClaimCpiBuilderInstruction {
            __program: program,
            reward_pool: None,
            reward_mint: None,
            vault: None,
            mining: None,
            mining_owner: None,
            deposit_authority: None,
            mining_owner_reward_token_account: None,
            token_program: None,
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
    /// The address of the reward mint
    #[inline(always)]
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }
    /// The address of the reward vault
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
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
    /// The end user the mining accounts belongs to
    #[inline(always)]
    pub fn mining_owner(
        &mut self,
        mining_owner: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mining_owner = Some(mining_owner);
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
    /// ATA where tokens will be claimed to
    #[inline(always)]
    pub fn mining_owner_reward_token_account(
        &mut self,
        mining_owner_reward_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mining_owner_reward_token_account =
            Some(mining_owner_reward_token_account);
        self
    }
    /// The address of the Token program where rewards are minted
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
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
        let instruction = ClaimCpi {
            __program: self.instruction.__program,

            reward_pool: self
                .instruction
                .reward_pool
                .expect("reward_pool is not set"),

            reward_mint: self
                .instruction
                .reward_mint
                .expect("reward_mint is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            mining: self.instruction.mining.expect("mining is not set"),

            mining_owner: self
                .instruction
                .mining_owner
                .expect("mining_owner is not set"),

            deposit_authority: self
                .instruction
                .deposit_authority
                .expect("deposit_authority is not set"),

            mining_owner_reward_token_account: self
                .instruction
                .mining_owner_reward_token_account
                .expect("mining_owner_reward_token_account is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ClaimCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    reward_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mining: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mining_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    deposit_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mining_owner_reward_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}