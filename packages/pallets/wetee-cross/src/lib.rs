#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::*;
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_std::convert::TryInto;
use wetee_dao::{self as dao};
use wetee_primitives::types::WeAssetId;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
        GuildCreated(WeAssetId, u64, T::AccountId),
        GuildJoined(WeAssetId, u64, T::AccountId),
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
        pub fn guild_join(
            origin: OriginFor<T>,
            dao_id: WeAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let daogov = wetee_dao::Pallet::<T>::ensrue_gov_approve_account(me.clone())?;

            log::info!("call by {:?}", daogov.1.id);
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            wetee_dao::Pallet::<T>::try_add_guild_member(dao_id, guild_id, who.clone())?;

            Self::deposit_event(Event::GuildJoined(dao_id, guild_id, who));
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {}
}
