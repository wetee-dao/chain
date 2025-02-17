#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::convert::TryInto;

use orml_traits::MultiCurrency;
use scale_info::prelude::vec::Vec;
use sp_std::result;

use wetee_assets::AssetMeta;
use wetee_dao::{self as dao};
use wetee_primitives::types::{WeAssetId, APP_MINT_ASSET_ID};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;

mod types;
pub use types::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiCurrency as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + dao::Config + wetee_fairlanch::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::type_value]
    pub fn DefaultForm1() -> u128 {
        1
    }

    /// 获取下一个应用 id
    #[pallet::storage]
    #[pallet::getter(fn next_app_id)]
    pub type NextAppId<T: Config> = StorageValue<_, u128, ValueQuery, DefaultForm1>;

    /// 用户对应集群的信息
    #[pallet::storage]
    #[pallet::getter(fn account_0f_app)]
    pub type AccountOfApp<T: Config> = StorageMap<_, Identity, u128, T::AccountId, OptionQuery>;

    /// 获取用户应用列表
    #[pallet::storage]
    #[pallet::getter(fn account_apps)]
    pub type AccountApps<T: Config> =
        StorageDoubleMap<_, Identity, T::AccountId, Identity, u128, (), ValueQuery>;

    /// 应用信息
    #[pallet::storage]
    #[pallet::getter(fn apps)]
    pub type Apps<T: Config> =
        StorageMap<_, Identity, u128, AppTemplate<T::AccountId, BlockNumberFor<T>>, OptionQuery>;

    /// 应用版本
    #[pallet::storage]
    #[pallet::getter(fn app_versions)]
    pub type AppVerions<T: Config> = StorageDoubleMap<
        _,
        Identity,
        u128,
        Identity,
        u16,
        (Vec<Image>, BlockNumberFor<T>),
        OptionQuery,
    >;

    /// 应用点数
    #[pallet::storage]
    #[pallet::getter(fn app_stakings)]
    pub type AppStakings<T: Config> = StorageMap<_, Identity, u128, BalanceOf<T>, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        App404,
        App403,
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
        pub fn register_app(
            origin: OriginFor<T>,
            // name of the K8sCluster.
            // 集群名字
            name: Vec<u8>,
            // app type
            // 程序运行的类型
            ty: AppType,
            // ip of service
            // 服务端口号
            images: Vec<Image>,
            // runtime type
            // TEE 运行类型
            run: RuntimeType,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let n = frame_system::Pallet::<T>::block_number();

            let app_id = NextAppId::<T>::get();
            let app = AppTemplate {
                id: app_id,
                account: who.clone(),
                start_block: n.clone(),
                stop_block: None,
                terminal_block: None,
                name,
                ty,
                run,
                status: 1,
            };

            Apps::<T>::insert(app_id, app);
            AppVerions::<T>::insert(app_id, 1, (images, n));
            AccountOfApp::<T>::insert(app_id, who.clone());
            AccountApps::<T>::insert(who.clone(), app_id, ());

            NextAppId::<T>::put(app_id + 1);

            // 生成质押资产
            Self::add_token(who, app_id)?;

            Ok(().into())
        }

        #[pallet::call_index(002)]
        #[pallet::weight(<weights::SubstrateWeight<T> as WeightInfo>::guild_join())]
        pub fn unregister_app(origin: OriginFor<T>, app_id: u128) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let mut app = Apps::<T>::get(app_id).ok_or(Error::<T>::App404)?;
            ensure!(app.account == who, Error::<T>::App403);

            // 停止应用
            // stop app
            app.status = 3;
            Apps::<T>::insert(app_id, app);

            Ok(().into())
        }

        #[pallet::call_index(003)]
        #[pallet::weight(<weights::SubstrateWeight<T> as WeightInfo>::guild_join())]
        pub fn add_app_version(
            origin: OriginFor<T>,
            app_id: u128,
            version: u16,
            images: Vec<Image>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let app = Apps::<T>::get(app_id).ok_or(Error::<T>::App404)?;
            ensure!(app.account == who, Error::<T>::App403);

            AppVerions::<T>::insert(
                app_id,
                version,
                (images, frame_system::Pallet::<T>::block_number()),
            );

            Ok(().into())
        }

        #[pallet::call_index(050)]
        #[pallet::weight(<weights::SubstrateWeight<T> as WeightInfo>::guild_join())]
        pub fn review_app(
            _origin: OriginFor<T>,
            _app_id: u128,
            _token_id: WeAssetId,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        /// Set boot peers
        /// 设置引导节点
        #[pallet::call_index(051)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::guild_join())]
        pub fn init_mint(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin)?;

            let root = wetee_dao::Pallet::<T>::asset_root();

            // 创建资产
            wetee_assets::Pallet::<T>::try_create(
                root,
                APP_MINT_ASSET_ID,
                AssetMeta {
                    name: "APP mint".as_bytes().to_vec(),
                    symbol: "wAPP".as_bytes().to_vec(),
                    decimals: 12,
                },
                0u32.into(),
            )?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // 添加运行资产到应用
        pub fn add_token_to_app(app_id: u128) -> result::Result<bool, DispatchError> {
            let who = AccountOfApp::<T>::get(app_id).ok_or(Error::<T>::App404)?;
            Self::add_token(who, app_id)
        }

        // 添加运行资产到应用
        pub fn minus_token_to_app(app_id: u128) -> result::Result<bool, DispatchError> {
            let who = AccountOfApp::<T>::get(app_id).ok_or(Error::<T>::App404)?;
            Self::minus_token(who, app_id)
        }

        // 添加运行资产到应用
        pub fn add_token(who: T::AccountId, app_id: u128) -> result::Result<bool, DispatchError> {
            wetee_fairlanch::Pallet::<T>::staking_asset(who, APP_MINT_ASSET_ID, 1u32.into())?;
            <AppStakings<T>>::mutate(&app_id, |t| *t += 1u32.into());
            Ok(true)
        }

        // 添加运行资产到应用
        pub fn minus_token(who: T::AccountId, app_id: u128) -> result::Result<bool, DispatchError> {
            wetee_fairlanch::Pallet::<T>::staking_minus(who, APP_MINT_ASSET_ID, 1u32.into())?;
            <AppStakings<T>>::mutate(&app_id, |t| *t -= 1u32.into());
            Ok(true)
        }
    }
}
