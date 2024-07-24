use crate::utils::*;
use mplx_rewards::{
    state::{RewardPool, WrappedMining},
    utils::LockupPeriod,
};
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use solana_program_test::*;
use solana_sdk::{clock::SECONDS_PER_DAY, signature::Keypair, signer::Signer};
use std::borrow::{Borrow, BorrowMut};

async fn setup() -> (ProgramTestContext, TestRewards, Pubkey, Pubkey) {
    let test = ProgramTest::new(
        "mplx_rewards",
        mplx_rewards::id(),
        processor!(mplx_rewards::processor::process_instruction),
    );

    let mut context = test.start_with_context().await;
    let owner = &context.payer.pubkey();

    let mint = Keypair::new();
    create_mint(&mut context, &mint, owner).await.unwrap();

    let test_reward_pool = TestRewards::new(mint.pubkey());
    test_reward_pool
        .initialize_pool(&mut context)
        .await
        .unwrap();

    let user = Keypair::new();
    let user_mining = test_reward_pool
        .initialize_mining(&mut context, &user.pubkey())
        .await;

    (context, test_reward_pool, user.pubkey(), user_mining)
}

#[tokio::test]
async fn success() {
    let (mut context, test_rewards, user, mining) = setup().await;

    let lockup_period = LockupPeriod::ThreeMonths;
    test_rewards
        .deposit_mining(&mut context, &mining, 100, lockup_period, &user, &mining)
        .await
        .unwrap();

    test_rewards
        .withdraw_mining(&mut context, &mining, &mining, 30, &user)
        .await
        .unwrap();

    let reward_pool_account = get_account(&mut context, &test_rewards.reward_pool).await;
    let reward_pool = RewardPool::unpack(reward_pool_account.data.borrow()).unwrap();

    assert_eq!(reward_pool.total_share, 170);

    let mut mining_account = get_account(&mut context, &mining).await;
    let mining_data = &mut mining_account.data.borrow_mut();
    let mining = WrappedMining::from_bytes_mut(mining_data).unwrap();
    assert_eq!(mining.mining.share, 170);
}

#[tokio::test]
async fn success_with_5kkk_after_expiring() {
    let (mut context, test_rewards, user, mining) = setup().await;

    let lockup_period = LockupPeriod::ThreeMonths;
    test_rewards
        .deposit_mining(
            &mut context,
            &mining,
            5000000000,
            lockup_period,
            &user,
            &mining,
        )
        .await
        .unwrap();

    advance_clock_by_ts(&mut context, (100 * SECONDS_PER_DAY).try_into().unwrap()).await;

    test_rewards
        .withdraw_mining(&mut context, &mining, &mining, 5000000000, &user)
        .await
        .unwrap();

    let reward_pool_account = get_account(&mut context, &test_rewards.reward_pool).await;
    let reward_pool = RewardPool::unpack(reward_pool_account.data.borrow()).unwrap();

    assert_eq!(reward_pool.total_share, 0);

    let mut mining_account = get_account(&mut context, &mining).await;
    let mining_data = &mut mining_account.data.borrow_mut();
    let wrapped_mining = WrappedMining::from_bytes_mut(mining_data).unwrap();
    assert_eq!(wrapped_mining.mining.share, 0);
}
