#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::ConstU32;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;
use sp_runtime::RuntimeDebug;
use sp_std::result;

use wetee_org::{self};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_org::Config {
        /// pallet event
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

    #[pallet::storage]
    #[pallet::getter(fn next_id)]
    pub type NextId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// root executes external transaction successfully.
        SudoDone { sudo: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        NotSudo,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 注册 dkg 节点
        /// register dkg node
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn test(origin: OriginFor<T>, pubkey: T::AccountId) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn call_app(// origin: T::AccountId,
            // params: Vec<u8>,
        ) -> result::Result<bool, DispatchError> {
            <NextId<T>>::mutate(|id| *id += 1);
            Ok(true)
        }
    }
}
