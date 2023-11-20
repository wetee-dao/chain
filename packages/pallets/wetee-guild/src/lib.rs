#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::*;
use frame_system::{ensure_signed, pallet_prelude::*};
use scale_info::prelude::vec::Vec;
use sp_std::convert::TryInto;
use wetee_org::{self as dao};
use wetee_primitives::types::DaoAssetId;

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
        GuildCreated(DaoAssetId, u64, T::AccountId),
        GuildJoined(DaoAssetId, u64, T::AccountId),
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
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(me.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            wetee_org::Pallet::<T>::try_add_guild_member(dao_id, guild_id, who.clone())?;

            Self::deposit_event(Event::GuildJoined(dao_id, guild_id, who));
            Ok(().into())
        }

        /// 创建公会
        #[pallet::call_index(002)]
        #[pallet::weight(<weights::SubstrateWeight<T> as WeightInfo>::create_guild())]
        pub fn create_guild(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            name: Vec<u8>,
            desc: Vec<u8>,
            meta_data: Vec<u8>,
            creator: T::AccountId,
        ) -> DispatchResult {
            let me = ensure_signed(origin.clone())?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(me.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            ensure!(desc.len() <= 50, dao::Error::<T>::PurposeTooLong);
            ensure!(meta_data.len() <= 1024, dao::Error::<T>::MetaDataTooLong);

            let now = <frame_system::Pallet:: <T>  as sp_runtime::traits::BlockNumberProvider>::current_block_number();

            // 创建核心团队-coreTeam
            let mut guilds = <dao::Guilds<T>>::get(dao_id);
            let id: u64 = guilds.len().try_into().unwrap();
            guilds
                .try_insert(
                    guilds.len(),
                    dao::GuildInfo {
                        id,
                        dao_account_id: wetee_org::Pallet::<T>::dao_guild(dao_id, id),
                        creator: creator.clone(),
                        start_block: now,
                        name,
                        desc,
                        status: dao::Status::Active,
                        meta_data,
                    },
                )
                .map_err(|_| dao::Error::<T>::GuildCreateError)?;
            <dao::Guilds<T>>::insert(dao_id, &guilds);

            // 更新团队成员
            let mut members = <dao::GuildMembers<T>>::get(dao_id, id);
            members
                .try_insert(0, creator.clone())
                .map_err(|_| dao::Error::<T>::GuildCreateError)?;

            // 更新组织
            <dao::GuildMembers<T>>::insert(dao_id, id, members);

            Self::deposit_event(Event::GuildCreated(
                dao_id,
                guilds.len().try_into().unwrap(),
                creator,
            ));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {}
}
