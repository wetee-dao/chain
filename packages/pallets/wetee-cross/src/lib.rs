#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::convert::TryInto;
use wetee_dao::{self as dao};
use wetee_primitives::types::WeAssetId;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + dao::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        BadDaoOrigin,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        Created(WeAssetId, T::AccountId),
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(<weights::SubstrateWeight<T> as WeightInfo>::guild_join())]
        pub fn cross_transfer_from(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::deposit_event(Event::<T>::Created(1, who.clone()));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {}
}
