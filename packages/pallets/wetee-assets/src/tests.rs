#![allow(unused_imports)]
#![cfg(test)]

use crate::mock::*;
use crate::{self as wetee_assets, AssetMeta};
use frame_support::{assert_noop, assert_ok, debug};
use wetee_primitives::types::WeAssetId;

pub fn create_asset() -> WeAssetId {
    let dao_id = wetee_dao::Pallet::<Test>::next_dao_id();

    wetee_dao::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
    )
    .unwrap();

    wetee_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        wetee_assets::AssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        10000,
    )
    .unwrap();

    dao_id
}

#[test]
pub fn test_create_asset() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        let alice_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();

        println!("alice_dao token {:?} \n", alice_dao,);

        assert_eq!(alice_dao, 99);
    })
}

#[test]
pub fn test_asset_trans() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        wetee_assets::Pallet::<Test>::transfer(RuntimeOrigin::signed(ALICE), BOB, dao_id, 1)
            .unwrap();

        let alice_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
        let bob_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
        println!(
            "\nalice_dao token {:?} ||| bob_dao token {:?}\n",
            alice_dao, bob_dao
        );

        assert_eq!(alice_dao, 98);
        assert_eq!(bob_dao, 1);
    })
}

#[test]
pub fn test_asset_burn() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        wetee_assets::Pallet::<Test>::burn(RuntimeOrigin::signed(ALICE), dao_id, 1).unwrap();

        let alice_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
        println!("\nalice_dao token {:?}", alice_dao,);

        assert_eq!(alice_dao, 98);
    })
}

#[test]
pub fn test_set_parachain_asset() {
    new_test_run().execute_with(|| {
        let proposal = RuntimeCall::WeteeAsset(wetee_assets::Call::set_parachain_asset {
            para_id: 0,
            metadata: AssetMeta {
                name: "TestA".as_bytes().to_vec(),
                symbol: "TA".as_bytes().to_vec(),
                decimals: 10,
            },
        });

        Sudo::sudo(RuntimeOrigin::signed(ALICE), Box::new(proposal)).unwrap();
    })
}
