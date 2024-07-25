use crate::{
    asserts::{assert_account_key, assert_uninitialized},
    state::{Mining, WeightedStakeDiffs, WrappedMining},
    utils::{find_mining_program_address, AccountLoader},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

pub fn process_initialize_mining<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    mining_owner: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter().enumerate();

    let reward_pool = AccountLoader::next_with_owner(account_info_iter, program_id)?;
    let mining = AccountLoader::next_uninitialized(account_info_iter)?;
    let payer = AccountLoader::next_signer(account_info_iter)?;
    let _system_program = AccountLoader::next_with_key(account_info_iter, &system_program::id())?;

    assert_uninitialized(mining)?;

    let bump = {
        let (pubkey, bump) = find_mining_program_address(program_id, mining_owner, reward_pool.key);
        assert_account_key(mining, &pubkey)?;
        bump
    };

    let signers_seeds = &[
        "mining".as_bytes(),
        &mining_owner.to_bytes(),
        &reward_pool.key.to_bytes(),
        &[bump],
    ];

    // TODO: refactor account creation
    let mining_acc_size = Mining::LEN + std::mem::size_of::<WeightedStakeDiffs>();
    let rent = Rent::get()?;
    let ix = system_instruction::create_account(
        &payer.key,
        &mining.key,
        rent.minimum_balance(mining_acc_size),
        (mining_acc_size) as u64,
        program_id,
    );
    invoke_signed(&ix, &[payer.clone(), mining.clone()], &[signers_seeds])?;

    let mining_data = &mut mining.data.borrow_mut();
    let wrapped_mining = WrappedMining::from_bytes_mut(mining_data)?;
    let mining = Mining::initialize(*reward_pool.key, bump, *mining_owner);
    *wrapped_mining.mining = mining;
    wrapped_mining.weighted_stake_diffs.initialize();

    Ok(())
}
