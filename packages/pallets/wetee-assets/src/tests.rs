#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_assets;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug};
use wetee_primitives::types::DaoAssetId;

pub fn create_asset() -> DaoAssetId {
    let dao_id = wetee_org::Pallet::<Test>::next_dao_id();

    wetee_org::Pallet::<Test>::create_dao(
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

    wetee_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        dao_id,
        wetee_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        10000,
        99,
    )
    .unwrap();

    let proposal = RuntimeCall::WeteeAsset(wetee_assets::Call::set_existenial_deposit {
        dao_id,
        existenial_deposit: 1,
    });

    assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(dao_id).unwrap().creator),
        dao_id,
        Box::new(proposal)
    ));

    dao_id
}

#[test]
pub fn test_create_asset() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        let alice_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();

        println!("alice_dao token {:?} \n", alice_dao,);

        assert_eq!(alice_dao, 10000);
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

        assert_eq!(alice_dao, 9999);
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

        assert_eq!(alice_dao, 9999);
    })
}

// #[test]
// pub fn test_asset_join() {
//     new_test_run().execute_with(|| {
//         let dao_id = create_asset();

//         wetee_assets::Pallet::<Test>::join(RuntimeOrigin::signed(BOB), dao_id, 100, 100)
//             .unwrap();

//         let bob_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
//         let bob = wetee_assets::Pallet::<Test>::get_balance(0, BOB).unwrap();

//         let dao = wetee_org::Pallet::<Test>::dao_account(dao_id);
//         let dao_b = wetee_assets::Pallet::<Test>::get_balance(0, dao).unwrap();

//         println!(
//             "join_request >>>>>> bob_dao token {:?} ||| bob token {:?} ||| dao_b {:?}",
//             bob_dao, bob, dao_b
//         );
//         assert_eq!(bob_dao, 100);
//         assert_eq!(bob, 9900);
//         assert_eq!(dao_b, 10100);
//     })
// }
