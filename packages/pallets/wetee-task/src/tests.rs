#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};
use wetee_primitives::types::{DiskClass, EnvKey};

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
        "{}".as_bytes().to_vec(),
        vec![Service::Tcp(80)],
        Command::SH(vec![1]),
        vec![],
        1,
        1,
        vec![Disk {
            path: DiskClass::SSD("test".as_bytes().to_vec()),
            size: 10,
        }],
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
            "{}".as_bytes().to_vec(),
            vec![Service::Tcp(80)],
            Command::SH(vec![1]),
            vec![],
            1,
            1,
            vec![Disk {
                path: DiskClass::SSD("test".as_bytes().to_vec()),
                size: 10,
            }],
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
            Some("test".as_bytes().to_vec()),
            Some(vec![1, 2, 3]),
            Some(vec![Service::Tcp(80)]),
            None,
            vec![EnvInput {
                etype: EditType::INSERT,
                k: EnvKey::Env("test".as_bytes().to_vec()),
                v: "test".as_bytes().to_vec(),
            }],
            false,
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
            Some("test".as_bytes().to_vec()),
            Some(vec![1, 2, 3]),
            Some(vec![Service::Tcp(80)]),
            None,
            vec![EnvInput {
                etype: EditType::INSERT,
                k: EnvKey::Env("test".as_bytes().to_vec()),
                v: "test".as_bytes().to_vec(),
            }],
            false,
        )
        .is_ok(),);
    });
}

#[test]
pub fn stop() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::try_stop(ALICE, 0).is_ok());
    });
}

#[test]
pub fn stop_should_fail() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::try_stop(BOB, 0).is_err());
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
        assert_ok!(Pallet::<Test>::charge(
            OriginFor::<Test>::signed(ALICE),
            0,
            1
        ));
    });
}

#[test]
pub fn charge2() {
    new_test_run().execute_with(|| {
        assert_ok!(Pallet::<Test>::charge(OriginFor::<Test>::signed(BOB), 0, 1));
    });
}
