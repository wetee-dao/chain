#![allow(unused_imports)]
#![cfg(test)]
use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug};

pub const ALICE: u64 = 1;
