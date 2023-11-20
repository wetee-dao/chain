#![allow(unused_imports)]
#![cfg(test)]

use super::*;
use frame_support::assert_ok;
use mock::{RuntimeCall, RuntimeOrigin, *};
use sp_runtime::traits::BlakeTwo256;

pub fn create_dao() {
    wetee_org::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
    )
    .unwrap();

    wetee_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        wetee_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        10000,
        200,
    )
    .unwrap();

    let proposal: <Test as wetee_org::Config>::RuntimeCall = Call::set_periods {
        dao_id: DAO_ID,
        periods: vec![Period {
            name: "gov".into(),
            pallet_index: 4,
            decision_deposit: 1,
            prepare_period: 10,
            max_deciding: 100,
            confirm_period: 10,
            decision_period: 10,
            min_enactment_period: 10,
            min_approval: 1,
            min_support: 1,
            max_balance: 0,
        }],
    }
    .into();

    wetee_sudo::Pallet::<Test>::sudo(RuntimeOrigin::signed(ALICE), DAO_ID, Box::new(proposal))
        .unwrap();
}

pub fn proposal() {
    create_dao();
    frame_system::Pallet::<Test>::set_block_number(2);

    let proposal: <Test as wetee_org::Config>::RuntimeCall = Call::set_max_pre_props {
        dao_id: 5000,
        max: 100,
    }
    .into();

    // 提交提案
    Pallet::<Test>::submit_proposal(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        MemberData::GLOBAL,
        Box::new(proposal.clone()),
        0,
    )
    .unwrap();

    frame_system::Pallet::<Test>::set_block_number(15);
}

pub fn deposit_proposal() {
    proposal();

    frame_system::Pallet::<Test>::set_block_number(30);
    assert_ok!(Pallet::<Test>::deposit_proposal(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        P_ID,
        100
    ));
}

pub fn vote() {
    deposit_proposal();
    frame_system::Pallet::<Test>::set_block_number(50);
    Pallet::<Test>::vote_for_prop(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32,
        Vote(100u64),
        Opinion::YES,
    )
    .unwrap();
    frame_system::Pallet::<Test>::set_block_number(70);
}

pub fn run() {
    vote();
    frame_system::Pallet::<Test>::set_block_number(180);
    assert_ok!(Pallet::<Test>::run_proposal(
        RuntimeOrigin::signed(ALICE),
        DAO_ID,
        0u32
    ));
}

#[test]
pub fn proposal_should_work() {
    new_test_run().execute_with(|| {
        proposal();
    });
}

#[test]
pub fn vote_should_work() {
    new_test_run().execute_with(|| {
        vote();
    });
}

#[test]
pub fn cancel_vote_should_work() {
    new_test_run().execute_with(|| {
        vote();
        frame_system::Pallet::<Test>::set_block_number(80);
        assert_ok!(Pallet::<Test>::cancel_vote(
            RuntimeOrigin::signed(ALICE),
            DAO_ID,
            0u32
        ));
    });
}

#[test]
pub fn run_proposal_should_work() {
    new_test_run().execute_with(|| {
        run();
    });
}

#[test]
pub fn unlock_should_work() {
    new_test_run().execute_with(|| {
        run();
        assert_ok!(Pallet::<Test>::unlock(RuntimeOrigin::signed(ALICE), DAO_ID));
    });
}
