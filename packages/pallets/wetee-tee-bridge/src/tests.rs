#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_sudo;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAO_ID: u64 = 5000;

pub fn create_dao() {
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
    crate::Account::<Test>::insert(DAO_ID, ALICE)
}

#[test]
pub fn set_sudo() {
    new_test_run().execute_with(|| {
        create_dao();

        assert_ok!(WETEESudo::set_sudo_account(
            RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(DAO_ID).unwrap().creator),
            DAO_ID,
            BOB,
        ));

        frame_system::Pallet::<Test>::set_block_number(10000);
        assert_eq!(crate::Account::<Test>::get(DAO_ID), Some(BOB));
    });
}

#[test]
pub fn sudo_should_work() {
    new_test_run().execute_with(|| {
        create_dao();
        // let proposal = RuntimeCall::WETEE(wetee_org::Call::create_guild {
        //     dao_id: DAO_ID,
        //     name: "name".as_bytes().to_vec(),
        //     desc: "desc".as_bytes().to_vec(),
        //     meta_data: "{}".as_bytes().to_vec(),
        // });

        // assert_ok!(crate::Pallet::<Test>::sudo(
        //     RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(DAO_ID).unwrap().creator),
        //     DAO_ID,
        //     Box::new(proposal)
        // ));
    });
}

#[test]
pub fn close_sudo_should_work() {
    new_test_run().execute_with(|| {
        create_dao();

        assert_ok!(WETEESudo::close_sudo(
            RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(DAO_ID).unwrap().creator),
            DAO_ID,
        ));

        assert_eq!(crate::CloseDao::<Test>::get(DAO_ID), Some(true));
    });
}
