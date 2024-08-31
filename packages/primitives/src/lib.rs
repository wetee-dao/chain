#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{DefensiveTruncateFrom, Get};
use scale_info::prelude::format;
use scale_info::prelude::vec::Vec;
use sp_runtime::BoundedSlice;
use sp_runtime::DispatchError;

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

pub fn handle_dispatch_error(error: DispatchError) -> Vec<u8> {
    match error {
        DispatchError::Other(str) => {
            return str.bytes().collect();
        }
        DispatchError::CannotLookup => {
            return "CannotLookup".bytes().collect();
        }
        DispatchError::BadOrigin => {
            return "BadOrigin".bytes().collect();
        }
        DispatchError::Module(e) => {
            let msg = format!("ModuleError index {}, error {:?}", e.index, e.error);
            return msg.into_bytes();
        }
        DispatchError::ConsumerRemaining => {
            return "ConsumerRemaining".bytes().collect();
        }
        DispatchError::NoProviders => {
            return "NoProviders".bytes().collect();
        }
        DispatchError::TooManyConsumers => {
            return "TooManyConsumers".bytes().collect();
        }
        DispatchError::Token(_) => {
            return "TokenError".bytes().collect();
        }
        DispatchError::Arithmetic(_) => {
            return "ArithmeticError".bytes().collect();
        }
        DispatchError::Transactional(_) => {
            return "TransactionalError".bytes().collect();
        }
        DispatchError::Exhausted => {
            return "Exhausted".bytes().collect();
        }
        DispatchError::Corruption => {
            return "Corruption".bytes().collect();
        }
        DispatchError::Unavailable => {
            return "Unavailable".bytes().collect();
        }
        DispatchError::RootNotAllowed => {
            return "RootNotAllowed".bytes().collect();
        }
    }
}
