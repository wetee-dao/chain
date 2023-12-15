#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    sp_runtime::traits::AccountIdConversion, PalletId,
};
use frame_system::pallet_prelude::*;
use scale_info::{prelude::vec::Vec, TypeInfo};
use sp_std::result;
use wetee_primitives::{
    traits::AfterCreate,
    types::{AppSetting, AppSettingInput, Cr, TeeAppId, WorkId},
};

use orml_traits::MultiCurrency;

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
pub struct TeeApp<AccountId, BlockNumber, Balance> {
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
    /// deposit of the App
    /// 抵押金额
    pub deposit: Balance,
    /// min score of the App
    /// 矿工最低等级
    pub level: u8,
}

/// 价格
/// price of computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Price {
    /// cpu
    pub cpu_per: u16,
    /// memory
    pub memory_per: u16,
    /// disk
    pub disk_per: u16,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_org::Config + wetee_assets::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Do some things after creating dao, such as setting up a sudo account.
        /// 创建部署任务后回调
        type AfterCreate: AfterCreate<WorkId, Self::AccountId>;

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

    /// App
    /// 应用
    #[pallet::storage]
    #[pallet::getter(fn tee_apps)]
    pub type TEEApps<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Identity,
        TeeAppId,
        TeeApp<T::AccountId, BlockNumberFor<T>, BalanceOf<T>>,
    >;

    /// Price of resource
    /// 价格
    #[pallet::storage]
    #[pallet::getter(fn price)]
    pub type Prices<T: Config> = StorageMap<_, Identity, u8, Price, OptionQuery>;

    /// App 对应账户
    /// user's K8sCluster information
    #[pallet::storage]
    #[pallet::getter(fn k8s_cluster_accounts)]
    pub type AppIdAccounts<T: Config> =
        StorageMap<_, Identity, TeeAppId, T::AccountId, OptionQuery>;

    /// App setting
    /// App设置
    #[pallet::storage]
    #[pallet::getter(fn app_settings)]
    pub type AppSettings<T: Config> =
        StorageDoubleMap<_, Identity, TeeAppId, Identity, u16, AppSetting, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// App created.
        CreatedApp { creator: T::AccountId, id: u64 },
        /// App runing.
        AppRuning { creator: T::AccountId, id: u64 },
        /// App stop.
        AppStop { creator: T::AccountId, id: u64 },
        /// App charge.
        Charge {
            from: T::AccountId,
            to: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// App pay run fee.
        PayRunFee {
            from: T::AccountId,
            to: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// App status mismatch.
        AppStatusMismatch,
        /// Root not exists.
        AppNotExists,
        /// Too many app.
        TooManyApp,
        /// App 403.
        App403,
        /// Not enough balance.
        NotEnoughBalance,
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
            cpu: u16,
            memory: u16,
            disk: u16,
            level: u8,
            #[pallet::compact] deposit: BalanceOf<T>,
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
                cr: Cr { cpu, memory, disk },
                deposit,
                level,
            };

            <NextTeeId<T>>::mutate(|id| *id += 1);
            <TEEApps<T>>::insert(who.clone(), id, app);
            <AppIdAccounts<T>>::insert(id, who.clone());

            // 检查抵押金额是否足够
            let fee_unit = Self::get_fee(id)?;
            ensure!(fee_unit >= deposit, Error::<T>::NotEnoughBalance);

            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(
                0,
                who.clone(),
                Self::app_id_account(id),
                deposit,
            )?;
            Self::deposit_event(Event::<T>::CreatedApp {
                id,
                creator: who.clone(),
            });

            // 执行 App 创建后回调,部署任务添加到消息中间件
            <T as pallet::Config>::AfterCreate::run_hook(WorkId { t: 1, id }, who);

            Ok(().into())
        }

        /// App update
        /// 更新任务
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn update(
            origin: OriginFor<T>,
            // App id
            // 应用id
            app_id: TeeAppId,
            // name of the app.
            // 程序名字
            name: Vec<u8>,
            // img of the App.
            // image 目标宗旨
            image: Vec<u8>,
            // port of service
            // 服务端口号
            port: Vec<u32>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <TEEApps<T>>::try_mutate_exists(
                who.clone(),
                app_id,
                |app_wrap| -> result::Result<(), DispatchError> {
                    let mut app = app_wrap.take().ok_or(Error::<T>::AppNotExists)?;
                    app.name = name;
                    app.image = image;
                    app.port = port;
                    *app_wrap = Some(app);
                    Ok(())
                },
            )?;
            Ok(().into())
        }

        /// App settings
        /// 任务设置
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn set_settings(
            origin: OriginFor<T>,
            app_id: TeeAppId,
            value: Vec<AppSettingInput>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let app_account = <AppIdAccounts<T>>::get(app_id).ok_or(Error::<T>::AppNotExists)?;
            ensure!(who == app_account, Error::<T>::App403);

            let mut iter = AppSettings::<T>::iter_prefix(app_id);
            let mut id = 0;

            // 解除所有的抵押
            while let Some(setting) = iter.next() {
                id = setting.0;
                // 处理更新和删除设置
                value.iter().for_each(|v| {
                    if (v.t == 2 || v.t == 3) && v.index == setting.0 {
                        match v.t {
                            // 更新设置
                            2 => {
                                <AppSettings<T>>::insert(
                                    app_id,
                                    setting.0,
                                    AppSetting {
                                        k: v.k.clone(),
                                        v: v.v.clone(),
                                    },
                                );
                            }
                            // 删除设置
                            3 => {
                                <AppSettings<T>>::remove(app_id, setting.0);
                            }
                            _ => {}
                        };
                    }
                });
            }

            // 新增设置
            value.iter().for_each(|v| {
                if v.t == 1 {
                    id = id + 1;
                    <AppSettings<T>>::insert(
                        app_id,
                        id,
                        AppSetting {
                            k: v.k.clone(),
                            v: v.v.clone(),
                        },
                    );
                }
            });

            Ok(().into())
        }

        /// App charge
        /// 任务充值
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn recharge(
            origin: OriginFor<T>,
            id: TeeAppId,
            deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(
                wetee_assets::NATIVE_ASSET_ID,
                who.clone(),
                Self::app_id_account(id),
                deposit,
            )?;

            Self::deposit_event(Event::<T>::Charge {
                from: who.clone(),
                to: Self::app_id_account(id),
                amount: deposit,
            });

            Ok(().into())
        }

        /// App stop
        /// 停止任务
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn stop(origin: OriginFor<T>, app_id: TeeAppId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::try_stop(who.clone(), app_id)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get app id account
        /// 获取 App 合约账户
        pub fn app_id_account(app_id: TeeAppId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(WorkId { id: app_id, t: 1 })
        }

        /// Get app id from account
        /// 获取账户中合约信息
        pub fn app_id_from_account(x: T::AccountId) -> WorkId {
            let (_, work) = PalletId::try_from_sub_account::<WorkId>(&x).unwrap();
            work
        }

        /// Stop app
        /// 停止任务
        /// 停止任务后,将任务状态设置为 2,并将抵押转移到目标账户
        pub fn try_stop(
            account: T::AccountId,
            app_id: TeeAppId,
        ) -> result::Result<(), DispatchError> {
            // 停止任务后,将任务状态设置为 2
            <TEEApps<T>>::try_mutate_exists(
                account.clone(),
                app_id,
                |app_wrap| -> result::Result<(), DispatchError> {
                    let mut app = app_wrap.take().ok_or(Error::<T>::AppNotExists)?;
                    app.status = 2;
                    *app_wrap = Some(app);
                    Ok(())
                },
            )?;
            Self::deposit_event(Event::<T>::AppStop {
                creator: account.clone(),
                id: app_id,
            });

            // 将抵押转移到目标账户
            if wetee_assets::Pallet::<T>::free_balance(
                wetee_assets::NATIVE_ASSET_ID,
                &Self::app_id_account(app_id),
            ) > 0u32.into()
            {
                wetee_assets::Pallet::<T>::try_transfer(
                    wetee_assets::NATIVE_ASSET_ID,
                    Self::app_id_account(app_id),
                    account,
                    wetee_assets::Pallet::<T>::free_balance(
                        wetee_assets::NATIVE_ASSET_ID,
                        &Self::app_id_account(app_id),
                    ),
                )?;
            }

            Ok(())
        }

        /// Pay run fee
        /// 支付运行费用
        pub fn pay_run_fee(
            wid: WorkId,
            fee: BalanceOf<T>,
            to: T::AccountId,
        ) -> result::Result<(), DispatchError> {
            if wetee_assets::Pallet::<T>::free_balance(0, &Self::app_id_account(wid.id)) < fee + fee
            {
                let app_account =
                    <AppIdAccounts<T>>::get(wid.id).ok_or(Error::<T>::AppNotExists)?;

                // 余额不足支持下一个周期的费用，停止任务
                Self::try_stop(app_account, wid.id)?;

                // 不足以支付当前周期的费用
                if wetee_assets::Pallet::<T>::free_balance(0, &Self::app_id_account(wid.id)) < fee {
                    return Err(Error::<T>::NotEnoughBalance.into());
                }
            }

            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(
                0,
                Self::app_id_account(wid.id),
                to.clone(),
                fee,
            )?;
            Self::deposit_event(Event::<T>::PayRunFee {
                from: Self::app_id_account(wid.id),
                to,
                amount: fee,
            });
            return Ok(());
        }

        /// Get fee
        /// 获取费用
        /// 费用 = cpu_per * cpu + memory_per * memory + disk_per * disk
        pub fn get_fee(id: TeeAppId) -> result::Result<BalanceOf<T>, DispatchError> {
            let app_account = <AppIdAccounts<T>>::get(id).ok_or(Error::<T>::AppNotExists)?;
            let app = <TEEApps<T>>::get(app_account.clone(), id).ok_or(Error::<T>::AppNotExists)?;
            let level = app.level;

            // 获取费用
            let p = <Prices<T>>::get(level).ok_or(Error::<T>::AppNotExists)?;

            return Ok(BalanceOf::<T>::from(
                p.cpu_per * app.cr.cpu + p.memory_per * app.cr.memory + p.disk_per * app.cr.disk,
            ));
        }

        /// Get app
        /// 获取任务信息
        /// 任务信息包括:
        /// 1. app id
        /// 2. app creator
        /// 3. app name
        /// 4. app image
        /// 5. app status
        /// 6. app start block
        /// 7. app stop block
        /// 8. app terminal block
        pub fn get_app(
            id: TeeAppId,
        ) -> result::Result<TeeApp<T::AccountId, BlockNumberFor<T>, BalanceOf<T>>, DispatchError>
        {
            let app_account = <AppIdAccounts<T>>::get(id).ok_or(Error::<T>::AppNotExists)?;
            let app = <TEEApps<T>>::get(app_account.clone(), id).ok_or(Error::<T>::AppNotExists)?;
            Ok(app)
        }
    }
}
