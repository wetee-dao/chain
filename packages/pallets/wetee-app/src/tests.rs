#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate as wetee_app;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};
use wetee_primitives::types::{DiskClass, EnvKey};

pub fn do_create() {
    Prices::<Test>::insert(
        1,
        Price {
            cpu_per: 100,
            memory_per: 100,
            disk_per: 100,
        },
    );
    Pallet::<Test>::create(
        OriginFor::<Test>::signed(ALICE),
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        vec![Service::Tcp(80)],
        Command::SH("".as_bytes().to_vec()),
        vec![EnvInput::default()],
        Some(EnvHash::default()),
        500,
        100,
        vec![Disk::default()],
        vec![Container::default(), Container::default()],
        1,
        TEEVersion::SGX,
    )
    .unwrap();
}

#[test]
pub fn create() {
    new_test_run().execute_with(|| {
        Prices::<Test>::insert(
            1,
            Price {
                cpu_per: 100,
                memory_per: 100,
                disk_per: 100,
            },
        );
        assert!(Pallet::<Test>::create(
            OriginFor::<Test>::signed(ALICE),
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            vec![Service::Tcp(80)],
            Command::SH("".as_bytes().to_vec()),
            vec![EnvInput::default()],
            Some(EnvHash::default()),
            500,
            100,
            vec![Disk::default()],
            vec![Container::default(), Container::default()],
            1,
            TEEVersion::SGX,
        )
        .is_ok());
    });
}

#[test]
pub fn update() {
    new_test_run().execute_with(|| {
        do_create();
        assert!(Pallet::<Test>::update(
            OriginFor::<Test>::signed(ALICE),
            0,
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],),
            Some(vec![Service::Tcp(80)]),
            Some(Command::SH("".as_bytes().to_vec())),
            vec![EnvInput::default()],
            Some(EnvHash::default()),
            true,
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
            1,
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],),
            Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],),
            Some(vec![Service::Tcp(80)]),
            Some(Command::SH("".as_bytes().to_vec())),
            vec![EnvInput::default()],
            Some(EnvHash::default()),
            true,
        )
        .is_err());
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
        assert_noop!(Pallet::<Test>::get_fee(0), Error::<Test>::AppNotExist);
    });
}
