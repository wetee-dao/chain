#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{DefensiveTruncateFrom, Get};
use scale_info::prelude::vec::Vec;
use sp_runtime::BoundedSlice;

pub mod traits;
pub mod types;

// pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: &str = Deserialize::deserialize(de)?;
//     Ok(s.as_bytes().to_vec())
// }

pub fn msg2bytes<N: Get<u32>>(x: &str) -> BoundedSlice<u8, N> {
    BoundedSlice::defensive_truncate_from(x.as_bytes())
}

pub fn vec2bytes<N: Get<u32>>(x: &Vec<u8>) -> BoundedSlice<u8, N> {
    BoundedSlice::defensive_truncate_from(&x)
}
