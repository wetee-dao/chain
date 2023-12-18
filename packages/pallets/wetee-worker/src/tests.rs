#![allow(unused_imports)]
#![cfg(test)]

use super::*;
use crate as wetee_worker;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

pub fn create_cluster() {
    assert!(Pallet::<Test>::cluster_register(
        OriginFor::<Test>::signed(ALICE),
        "test".as_bytes().to_vec(),
        vec![vec![127u8, 0u8, 0u8, 1u8]],
        vec![8080],
        1,
    )
    .is_ok());
    DepositPrices::<Test>::insert(
        1,
        DepositPrice {
            cpu_per: 10,
            memory_per: 10,
            disk_per: 10,
        },
    );
}

#[test]
pub fn cluster_register() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(ALICE),
            "test".as_bytes().to_vec(),
            vec![vec![127u8, 0u8, 0u8, 1u8]],
            vec![8080],
            1,
        )
        .is_ok());
    });
}

// 没有ip
#[test]
pub fn cluster_register_should_fail() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![vec![]],
            vec![8080],
            1,
        )
        .is_err());
    });
}

// ip格式错误
#[test]
pub fn cluster_register_should_fail2() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![vec![127, 0, 0]],
            vec![8080],
            1,
        )
        .is_err());
    });
}

// 没有端口
#[test]
pub fn cluster_register_should_fail3() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![vec![127, 0, 0, 1]],
            vec![],
            1,
        )
        .is_err());
    });
}

#[test]
pub fn cluster_mortgage() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
    });
}

// 非集群所有者
#[test]
pub fn cluster_mortgage_should_fail() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(
            Pallet::<Test>::cluster_mortgage(OriginFor::<Test>::signed(BOB), 0, 1, 1, 1, 100)
                .is_err()
        );
    });
}

// 余额不够
#[test]
pub fn cluster_mortgage_should_fail2() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(
            Pallet::<Test>::cluster_mortgage(OriginFor::<Test>::signed(ALICE), 0, 1, 1, 1, 0)
                .is_err()
        );
    });
}

#[test]
pub fn cluster_unmortgage() {
    new_test_run().execute_with(|| {
        create_cluster();
        frame_system::Pallet::<Test>::set_block_number(30);
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
        frame_system::Pallet::<Test>::set_block_number(31);
        assert!(
            Pallet::<Test>::cluster_unmortgage(OriginFor::<Test>::signed(ALICE), 0, 30).is_ok()
        );
    });
}
