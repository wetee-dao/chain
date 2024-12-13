#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate as wetee_app;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};
use frame_system::pallet_prelude::OriginFor;
use orml_traits::MultiCurrency;
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

    Economics::<Test>::insert(0, 10);
    Economics::<Test>::insert(1, 25);
    Economics::<Test>::insert(ASSET_ID, 65);
    BlockReward::<Test>::set((0u32.into(), 1, WTE.saturated_into::<BalanceOf<Test>>()));

    System::set_block_number(1);
}

#[test]
pub fn block_reward() {
    new_test_run().execute_with(|| {
        init();

        run_to_block(2);

        let balance = System::account(BOB);
        assert!(balance.data.free == (WTE / 10) as u64);
    });
}

#[test]
pub fn v_staking() {
    new_test_run().execute_with(|| {
        init();

        Pallet::<Test>::v_staking(OriginFor::<Test>::signed(ALICE), VASSET_ID, 100).unwrap();

        let to_staking = Pallet::<Test>::to_stakings(ALICE, ASSET_ID).unwrap();
        assert!(to_staking == 120);

        let reward = Pallet::<Test>::next_block_reward(51, ALICE).unwrap();
        assert!(reward);

        let reward = System::account(ALICE);
        assert!(reward.data.free == 10000000000000000);

        run_to_block(101);

        let reward2 = System::account(ALICE);
        assert!(reward2.data.free == 10032483750000000);
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
        assert!(to_staking2 == 108);
    });
}

#[test]
pub fn v_unstaking() {
    new_test_run().execute_with(|| {
        init();

        Pallet::<Test>::v_staking(OriginFor::<Test>::signed(ALICE), VASSET_ID, 100).unwrap();
        let to_staking = Pallet::<Test>::to_stakings(ALICE, ASSET_ID).unwrap();
        assert!(to_staking == 120);

        run_to_block(51);

        let staking = Pallet::<Test>::stakings(ALICE, ASSET_ID).unwrap();
        assert!(staking == 120);

        let balance = Tokens::free_balance(ASSET_ID, &ALICE);
        assert!(balance == 120);

        assert!(Asset::transfer(OriginFor::<Test>::signed(ALICE), BOB, ASSET_ID, 100).is_err());

        let balance_lock = Tokens::locks(ASSET_ID, &ALICE.clone());
        println!("balance_lock: {:?}", balance_lock);

        Pallet::<Test>::v_unstaking(OriginFor::<Test>::signed(ALICE), VASSET_ID, 10).unwrap();

        let staking2 = Pallet::<Test>::stakings(ALICE, ASSET_ID).unwrap();
        assert!(staking2 == 110);
    });
}
