#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

pub fn do_create() {
    Prices::<Test>::insert(
        1,
        Price {
            cpu_per_block: 100,
            memory_per_block: 100,
            disk_per_block: 100,
        },
    );
    let res = Pallet::<Test>::create(
        OriginFor::<Test>::signed(ALICE),
        "test".as_bytes().to_vec(),
        "test".as_bytes().to_vec(),
        vec![1, 2, 3],
        1,
        1,
        1,
        1,
        1000,
    );
    println!("{:?}", res);
}

#[test]
pub fn create() {
    new_test_run().execute_with(|| {
        Prices::<Test>::insert(
            1,
            Price {
                cpu_per_block: 100,
                memory_per_block: 100,
                disk_per_block: 100,
            },
        );
        Pallet::<Test>::create(
            OriginFor::<Test>::signed(ALICE),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec![1, 2, 3],
            1,
            1,
            1,
            1,
            300,
        )
        .unwrap();
    });
}

#[test]
pub fn update() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::update(
            OriginFor::<Test>::signed(ALICE),
            0,
            "test".as_bytes().to_vec(),
            vec![1, 2, 3],
            vec![1],
        )
        .is_ok());
    });
}

// 应用不属于用户
#[test]
pub fn update_should_fail() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::update(
            OriginFor::<Test>::signed(ALICE),
            0,
            "test".as_bytes().to_vec(),
            vec![1, 2, 3],
            vec![1, 2]
        )
        .is_ok(),);
    });
}

#[test]
pub fn stop() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::stop(OriginFor::<Test>::signed(ALICE), 0).is_ok());
    });
}

#[test]
pub fn stop_should_fail() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::stop(OriginFor::<Test>::signed(BOB), 0).is_err());
    });
}

#[test]
pub fn get_fee() {
    new_test_run().execute_with(|| {
        do_create();
        assert_ok!(Pallet::<Test>::get_fee(0));
    });
}

#[test]
pub fn get_fee_should_fail() {
    new_test_run().execute_with(|| {
        assert_noop!(Pallet::<Test>::get_fee(0), Error::<Test>::TaskNotExists);
    });
}

#[test]
pub fn recharge() {
    new_test_run().execute_with(|| {
        do_create();
        assert_ok!(Pallet::<Test>::recharge(
            OriginFor::<Test>::signed(ALICE),
            0,
            1
        ));
    });
}

#[test]
pub fn charge2() {
    new_test_run().execute_with(|| {
        assert_ok!(Pallet::<Test>::recharge(
            OriginFor::<Test>::signed(BOB),
            0,
            1
        ));
    });
}

#[test]
pub fn set_settings() {
    new_test_run().execute_with(|| {
        do_create();
        assert_ok!(Pallet::<Test>::set_settings(
            OriginFor::<Test>::signed(ALICE),
            0,
            vec![
                AppSettingInput {
                    t: 1,
                    index: 0,
                    k: "test".as_bytes().to_vec(),
                    v: "test".as_bytes().to_vec(),
                },
                AppSettingInput {
                    t: 1,
                    index: 1,
                    k: "test".as_bytes().to_vec(),
                    v: "test".as_bytes().to_vec(),
                }
            ]
        ));
    });
}

#[test]
pub fn set_settings_should_fail() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::set_settings(
            OriginFor::<Test>::signed(BOB),
            0,
            vec![
                AppSettingInput {
                    t: 1,
                    index: 0,
                    k: "test".as_bytes().to_vec(),
                    v: "test".as_bytes().to_vec(),
                },
                AppSettingInput {
                    t: 1,
                    index: 1,
                    k: "test".as_bytes().to_vec(),
                    v: "test".as_bytes().to_vec(),
                }
            ]
        )
        .is_err());
    });
}
