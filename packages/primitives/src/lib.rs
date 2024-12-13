#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{DefensiveTruncateFrom, Get};
use scale_info::prelude::format;
use scale_info::prelude::string::String;
use scale_info::prelude::vec::Vec;
use sp_runtime::BoundedSlice;
use sp_runtime::DispatchError;

pub mod converter;
pub mod traits;
pub mod types;

// pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: &str = Deserialize::deserialize(de)?;
//     Ok(s.as_bytes().to_vec())
// }

// 1 WTE = 1_000_000_000_000
pub const WTE: u128 = 1_000_000_000_000;

pub fn msg2bytes<N: Get<u32>>(x: &str) -> BoundedSlice<u8, N> {
    BoundedSlice::defensive_truncate_from(x.as_bytes())
}

pub fn vec2bytes<N: Get<u32>>(x: &Vec<u8>) -> BoundedSlice<u8, N> {
    BoundedSlice::defensive_truncate_from(&x)
}

pub fn handle_dispatch_error(error: DispatchError) -> String {
    match error {
        DispatchError::Other(str) => {
            return String::from(str);
        }
        DispatchError::CannotLookup => {
            return String::from("CannotLookup");
        }
        DispatchError::BadOrigin => {
            return String::from("BadOrigin");
        }
        DispatchError::Module(e) => {
            let msg = format!("ModuleError index {}, error {:?}", e.index, e.error);
            return msg;
        }
        DispatchError::ConsumerRemaining => {
            return String::from("ConsumerRemaining");
        }
        DispatchError::NoProviders => {
            return String::from("NoProviders");
        }
        DispatchError::TooManyConsumers => {
            return String::from("TooManyConsumers");
        }
        DispatchError::Token(_) => {
            return String::from("TokenError");
        }
        DispatchError::Arithmetic(_) => {
            return String::from("ArithmeticError");
        }
        DispatchError::Transactional(_) => {
            return String::from("TransactionalError");
        }
        DispatchError::Exhausted => {
            return String::from("Exhausted");
        }
        DispatchError::Corruption => {
            return String::from("Corruption");
        }
        DispatchError::Unavailable => {
            return String::from("Unavailable");
        }
        DispatchError::RootNotAllowed => {
            return String::from("RootNotAllowed");
        }
    }
}
