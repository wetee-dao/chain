#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate as wetee_app;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};
use frame_system::pallet_prelude::OriginFor;
use wetee_primitives::types::{DiskClass, EnvKey};

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const VASSET_ID: WeAssetId = 5000;
pub const ASSET_ID: WeAssetId = 5001;

pub fn init() {
    wetee_assets::Pallet::<Test>::create_asset(
        OriginFor::<Test>::signed(ALICE),
        wetee_assets::AssetMeta {
            name: "VToken".as_bytes().to_vec(),
            symbol: "VToken".as_bytes().to_vec(),
            decimals: 12,
        },
        1000000000,
    )
    .unwrap();

    wetee_assets::Pallet::<Test>::create_asset(
        OriginFor::<Test>::signed(ALICE),
        wetee_assets::AssetMeta {
            name: "Token".as_bytes().to_vec(),
            symbol: "Token".as_bytes().to_vec(),
            decimals: 12,
        },
        0,
    )
    .unwrap();

    Pallet::<Test>::register_vtoken(
        OriginFor::<Test>::signed(ALICE),
        VASSET_ID,
        ASSET_ID,
        10,
        12,
    )
    .unwrap();

    Pallet::<Test>::set_economics(OriginFor::<Test>::signed(ALICE), ASSET_ID, 65).unwrap();
}

#[test]
pub fn v_staking() {
    new_test_run().execute_with(|| {
        init();

        Pallet::<Test>::v_staking(OriginFor::<Test>::signed(ALICE), VASSET_ID, 100).unwrap();

        let to_staking = Pallet::<Test>::to_stakings(ALICE, ASSET_ID).unwrap();
        assert!(to_staking == 120);

        let reward = Pallet::<Test>::next_block_reward(50, ALICE).unwrap();
        assert!(reward);
    });
}

#[test]
pub fn v_staking_cancel() {
    new_test_run().execute_with(|| {
        init();

        Pallet::<Test>::v_staking(OriginFor::<Test>::signed(ALICE), VASSET_ID, 100).unwrap();

        let to_staking = Pallet::<Test>::to_stakings(ALICE, ASSET_ID).unwrap();
        assert!(to_staking == 120);

        Pallet::<Test>::v_staking_cancel(OriginFor::<Test>::signed(ALICE), VASSET_ID, 10).unwrap();

        let to_staking2 = Pallet::<Test>::to_stakings(ALICE, ASSET_ID).unwrap();
        // assert!(to_staking == 120);
        println!("{}", to_staking2)
    });
}
