#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_worker;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

// pub const ALICE: u64 = 1;
// pub const BOB: u64 = 2;
// pub const DAO_ID: u64 = 1;

#[test]
pub fn set_sudo() {
    new_test_run().execute_with(|| {});
}
