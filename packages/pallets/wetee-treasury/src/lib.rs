#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use orml_traits::MultiCurrency;
use sp_std::prelude::*;
use wetee_primitives::types::DaoAssetId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + wetee_org::Config + wetee_assets::Config + wetee_gov::Config
    {
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new spend proposal has been approved.
        SpendApproved {
            dao_id: DaoAssetId,
            amount: BalanceOf<T>,
            beneficiary: T::AccountId,
        },

        /// A proposal was rejected;
        SpendRejected {
            dao_id: DaoAssetId,
            amount: BalanceOf<T>,
            beneficiary: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// DAO id mismatch
        MaxBalanceExceeded,
        BadDaoOrigin,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(2)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::spend())]
        pub fn spend(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            beneficiary: T::AccountId,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let (_, account) = wetee_org::Pallet::<T>::ensrue_gov_approve_account(who.clone())?;
            ensure!(account.id == dao_id, Error::<T>::BadDaoOrigin);

            // 确定金额是否超过最大金额
            let period = wetee_gov::Pallet::<T>::get_period(dao_id, account.p)?;
            ensure!(period.max_balance >= amount, Error::<T>::MaxBalanceExceeded);

            // 执行转账
            wetee_assets::Pallet::<T>::try_transfer(
                dao_id,
                wetee_org::Pallet::<T>::dao_treasury(dao_id),
                beneficiary.clone(),
                amount,
            )?;
            Self::deposit_event(Event::<T>::SpendApproved {
                dao_id,
                amount,
                beneficiary,
            });

            Ok(())
        }
    }
}
