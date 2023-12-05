#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

/// App specific information
/// 程序信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct TeeApp<AccountId, BlockNumber> {
    /// creator of app
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the App
    /// App创建的区块
    pub start_block: BlockNumber,
    /// name of the app.
    /// 程序名字
    pub name: Vec<u8>,
    /// img of the App.
    /// image 目标宗旨
    pub image: Vec<u8>,
    /// port of service
    /// 服务端口号
    pub port: Vec<u32>,
    /// State of the App
    /// App状态
    status: u8,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

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

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CreatedApp { creator: T::AccountId, id: u64 },
        AppRuning { minter: T::AccountId, id: u64 },
        Reported,
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        AppStatusMismatch,
        RootNotExists,
        TooManyApp,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Task register
        #[pallet::call_index(001)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_register(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Worker cluster mortgage
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_mortgage(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Task upload proof of work data
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_proof_upload(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Task withdrawal
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_withdrawal(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Task stop
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_stop(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Worker cluster report
        #[pallet::call_index(006)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_report(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Self::deposit_event(Event::Reported);
            Ok(().into())
        }
    }
}
