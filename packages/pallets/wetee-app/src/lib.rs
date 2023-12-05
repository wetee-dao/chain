#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::{prelude::vec::Vec, TypeInfo};
use wetee_primitives::{
    traits::AfterCreate,
    types::{Cr, TeeAppId, WorkerId},
};

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
    pub id: TeeAppId,
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
    pub status: u8,
    /// cpu memory disk
    /// cpu memory disk
    pub cr: Cr,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_org::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Do some things after creating dao, such as setting up a sudo account.
        /// 创建部署任务后回调
        type AfterCreate: AfterCreate<WorkerId, Self::AccountId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// The id of the next app to be created.
    /// 获取下一个app id
    #[pallet::storage]
    #[pallet::getter(fn next_tee_id)]
    pub type NextTeeId<T: Config> = StorageValue<_, TeeAppId, ValueQuery>;

    /// 任务
    /// Those who have locked a deposit.
    /// TWOX-NOTE: Safe, as increasing integer keys are safe.
    #[pallet::storage]
    #[pallet::getter(fn tee_apps)]
    pub type TEEApps<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Identity,
        TeeAppId,
        TeeApp<T::AccountId, BlockNumberFor<T>>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CreatedApp { creator: T::AccountId, id: u64 },
        AppRuning { minter: T::AccountId, id: u64 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// App status mismatch.
        AppStatusMismatch,
        /// Root not exists.
        RootNotExists,
        /// Too many app.
        TooManyApp,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// App create
        /// 注册任务
        #[pallet::call_index(001)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn create(
            origin: OriginFor<T>,
            name: Vec<u8>,
            image: Vec<u8>,
            port: Vec<u32>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let id = Self::next_tee_id();
            let app = TeeApp {
                id,
                name,
                image,
                port,
                creator: who.clone(),
                start_block: <frame_system::Pallet<T>>::block_number(),
                status: 1,
                cr: Cr {
                    cpu: 0,
                    memory: 0,
                    disk: 0,
                },
            };

            <TEEApps<T>>::insert(who.clone(), id, app);
            Self::deposit_event(Event::<T>::CreatedApp {
                id,
                creator: who.clone(),
            });

            // 执行 App 创建后回调,部署任务添加到消息中间件
            <T as pallet::Config>::AfterCreate::run_hook(WorkerId { t: 1, id }, who);

            Ok(().into())
        }

        /// App update
        /// 更新任务
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn update(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let who = ensure_signed(origin)?;
            Ok(().into())
        }

        /// App settings
        /// 任务设置
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn set_settings(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let who = ensure_signed(origin)?;
            Ok(().into())
        }

        /// App charge
        /// 任务充值
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn recharge(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let who = ensure_signed(origin)?;
            Ok(().into())
        }

        /// App stop
        /// 停止任务
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn stop(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let who = ensure_signed(origin)?;
            Ok(().into())
        }
    }
}
