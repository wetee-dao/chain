#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug};

pub const ALICE: u64 = 1;

pub fn create_dao() {
    Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
    )
    .unwrap();
}

#[test]
pub fn create_dao_should_work() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::create_dao(
            RuntimeOrigin::signed(ALICE),
            vec![1; 4],
            vec![1; 60],
            vec![1; 60],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
        )
        .is_err());
        assert_ok!(Pallet::<Test>::create_dao(
            RuntimeOrigin::signed(ALICE),
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
        ));
        assert!(Daos::<Test>::get(5000u64).is_some());
        assert!(NextDaoId::<Test>::get() == 5001u64);
    });
}

#[test]
pub fn get_creator() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::try_get_creator(5000u64).is_err());
        create_dao();
        assert_ok!(Pallet::<Test>::try_get_creator(5000u64));
    });
}

#[test]
pub fn get_dao() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::try_get_dao(5000u64).is_err());
        create_dao();
        assert_ok!(Pallet::<Test>::try_get_dao(5000u64));
    });
}

#[test]
pub fn roadmap_task() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::try_get_dao(5000u64).is_err());
        create_dao();
        assert_ok!(Pallet::<Test>::try_get_dao(5000u64));
        assert_ok!(Pallet::<Test>::create_roadmap_task(
            RuntimeOrigin::signed(ALICE),
            5000u64,
            202301,
            vec![1; 4],     // name
            1,              // priority
            vec![1].into(), // tags
        ));
        Pallet::<Test>::get_task(5000u64, 202301, 0).unwrap();
    });
}

#[test]
pub fn get_dao_account_id() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::try_get_dao_account_id(5000u64).is_err());
        create_dao();
        assert_ok!(Pallet::<Test>::try_get_dao_account_id(5000u64));
    });
}
