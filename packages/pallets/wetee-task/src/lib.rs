#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    sp_runtime::traits::AccountIdConversion, PalletId,
};
use frame_system::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
use sp_std::result;
use wetee_primitives::{
    traits::UHook,
    types::{
        ClusterLevel, Command, Cr, Disk, EditType, Env, EnvInput, Service, TeeAppId, WorkId,
        WorkStatus,
    },
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

/// Task specific information
/// 程序信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct TeeTask<AccountId, BlockNumber> {
    pub id: TeeAppId,
    /// creator of app
    /// 创建者
    pub creator: AccountId,
    /// contract id
    /// 合约账户
    pub contract_id: AccountId,
    /// The block that creates the Task
    /// Task创建的区块
    pub start_block: BlockNumber,
    /// name of the app.
    /// 程序名字
    pub name: Vec<u8>,
    /// img of the Task.
    /// image 目标宗旨
    pub image: Vec<u8>,
    /// meta of the App.
    /// 应用元数据
    pub meta: Vec<u8>,
    /// command of service
    /// 执行命令
    pub command: Command,
    /// port of service
    /// 服务端口号
    pub port: Vec<Service>,
    /// State of the Task
    /// Task状态
    pub status: WorkStatus,
    /// cpu memory disk
    /// cpu memory disk
    pub cr: Cr,
    /// min score of the Task
    /// 矿工最低等级
    pub level: ClusterLevel,
}

/// 价格
/// price of computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Price {
    /// cpu
    pub cpu_per_block: u32,
    /// memory
    pub memory_per_block: u32,
    /// disk
    pub disk_per_block: u32,
}

#[frame_support::pallet]
pub mod pallet {
    use sp_runtime::SaturatedConversion;
    use wetee_primitives::types::WorkType;

    use super::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[derive(frame_support::DefaultNoBound)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            Prices::<T>::insert(
                1,
                Price {
                    cpu_per_block: 100,
                    memory_per_block: 100,
                    disk_per_block: 100,
                },
            );
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_org::Config + wetee_assets::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Do some things after creating dao, such as setting up a sudo account.
        /// 创建部署任务后回调
        type UHook: UHook<WorkId, Self::AccountId>;

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

    /// Task
    /// 应用
    #[pallet::storage]
    #[pallet::getter(fn tee_apps)]
    pub type TEETasks<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Identity,
        TeeAppId,
        TeeTask<T::AccountId, BlockNumberFor<T>>,
    >;

    /// Price of resource
    /// 价格
    #[pallet::storage]
    #[pallet::getter(fn price)]
    pub type Prices<T: Config> = StorageMap<_, Identity, u8, Price, OptionQuery>;

    /// Task 对应账户
    /// user's K8sCluster information
    #[pallet::storage]
    #[pallet::getter(fn k8s_cluster_accounts)]
    pub type TaskIdAccounts<T: Config> =
        StorageMap<_, Identity, TeeAppId, T::AccountId, OptionQuery>;

    /// Task setting
    /// Task设置
    #[pallet::storage]
    #[pallet::getter(fn app_settings)]
    pub type Envs<T: Config> =
        StorageDoubleMap<_, Identity, TeeAppId, Identity, u16, Env, OptionQuery>;

    /// Task version
    /// Task 版本
    #[pallet::storage]
    #[pallet::getter(fn task_version)]
    pub type TaskVersion<T: Config> =
        StorageMap<_, Identity, TeeAppId, BlockNumberFor<T>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Task created.
        CreatedTask {
            creator: T::AccountId,
            id: u64,
        },
        /// Task runing.
        TaskRuning {
            creator: T::AccountId,
            id: u64,
        },
        /// Task stop.
        TaskStop {
            creator: T::AccountId,
            id: u64,
        },
        /// Task charge.
        Charge {
            from: T::AccountId,
            to: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Task pay run fee.
        PayRunFee {
            from: T::AccountId,
            to: T::AccountId,
            amount: BalanceOf<T>,
        },
        WorkUpdated {
            user: T::AccountId,
            work_id: WorkId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Task status mismatch.
        TaskStatusMismatch,
        /// Root not exists.
        TaskNotExists,
        /// Too many app.
        TooManyTask,
        /// Task 403.
        Task403,
        /// Not enough balance.
        NotEnoughBalance,
        /// Task is runing.
        TaskIsRuning,
        /// Task is stop.
        TaskIsStoped,
        /// Level not exists.
        LevelNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Task create
        /// 注册任务
        #[pallet::call_index(001)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn create(
            origin: OriginFor<T>,
            name: Vec<u8>,
            image: Vec<u8>,
            meta: Vec<u8>,
            port: Vec<Service>,
            command: Command,
            env: Vec<EnvInput>,
            cpu: u32,
            memory: u32,
            disk: Vec<Disk>,
            level: u8,
            #[pallet::compact] deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let id = Self::next_tee_id();
            let app = TeeTask {
                id,
                name,
                image,
                meta,
                port,
                command,
                creator: who.clone(),
                start_block: <frame_system::Pallet<T>>::block_number(),
                status: 0,
                cr: Cr {
                    cpu,
                    mem: memory,
                    disk: disk.clone(),
                    gpu: 0,
                },
                contract_id: Self::task_id_account(id),
                level,
            };

            <NextTeeId<T>>::mutate(|id| *id += 1);
            <TEETasks<T>>::insert(who.clone(), id, app);
            <TaskIdAccounts<T>>::insert(id, who.clone());
            <TaskVersion<T>>::insert(id, <frame_system::Pallet<T>>::block_number());

            let mut sid = 0;
            env.iter().for_each(|v| {
                if v.etype == EditType::INSERT {
                    sid = sid + 1;
                    <Envs<T>>::insert(
                        id,
                        sid,
                        Env {
                            k: v.k.clone(),
                            v: v.v.clone(),
                        },
                    );
                }
            });

            // Check deposit
            // 检查抵押金额是否足够
            let fee_unit = Self::get_fee(id)?;
            ensure!(deposit >= fee_unit, Error::<T>::NotEnoughBalance);

            // Transfer deposit
            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(
                0,
                who.clone(),
                Self::task_id_account(id),
                deposit,
            )?;
            Self::deposit_event(Event::<T>::CreatedTask {
                id,
                creator: who.clone(),
            });

            // Run UHook hook
            // 执行 Task 创建后回调,部署任务添加到消息中间件
            <T as pallet::Config>::UHook::run_hook(
                WorkId {
                    wtype: WorkType::TASK,
                    id,
                },
                who,
            );

            Ok(().into())
        }

        /// Rerun task
        /// 重启任务
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn rerun(origin: OriginFor<T>, id: TeeAppId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let account = <TaskIdAccounts<T>>::get(id).ok_or(Error::<T>::TaskNotExists)?;
            ensure!(who == account, Error::<T>::Task403);

            let deposit = wetee_assets::Pallet::<T>::free_balance(
                wetee_assets::NATIVE_ASSET_ID,
                &Self::task_id_account(id),
            );

            let task = Self::tee_apps(who.clone(), id).unwrap();
            ensure!(task.status == 2, Error::<T>::TaskStatusMismatch);

            // Check deposit
            // 检查抵押金额是否足够
            let fee_unit = Self::get_fee(id)?;
            ensure!(deposit >= fee_unit, Error::<T>::NotEnoughBalance);

            <TEETasks<T>>::try_mutate_exists(
                who.clone(),
                id,
                |app_wrap| -> result::Result<(), DispatchError> {
                    let mut app = app_wrap.take().ok_or(Error::<T>::TaskNotExists)?;
                    app.status = 4;
                    *app_wrap = Some(app);
                    Ok(())
                },
            )?;

            // Run UHook hook
            // 执行 Task 创建后回调,部署任务添加到消息中间件
            <T as pallet::Config>::UHook::run_hook(
                WorkId {
                    wtype: WorkType::TASK,
                    id,
                },
                who,
            );

            Ok(().into())
        }

        /// Task update
        /// 更新任务
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn update(
            origin: OriginFor<T>,
            // App id
            // 应用id
            app_id: TeeAppId,
            // name of the app.
            // 程序名字
            new_name: Option<Vec<u8>>,
            // img of the App.
            // image 目标宗旨
            new_image: Option<Vec<u8>>,
            // port of service
            // 服务端口号
            new_port: Option<Vec<Service>>,
            // run command
            new_command: Option<Command>,
            // setting
            // 设置
            new_env: Vec<EnvInput>,
            // with restart
            // 是否重启
            with_restart: bool,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let account = <TaskIdAccounts<T>>::get(app_id).ok_or(Error::<T>::TaskNotExists)?;
            ensure!(who == account, Error::<T>::Task403);

            <TEETasks<T>>::try_mutate_exists(
                who.clone(),
                app_id,
                |app_wrap| -> result::Result<(), DispatchError> {
                    let mut app = app_wrap.take().ok_or(Error::<T>::TaskNotExists)?;
                    if new_name.is_some() {
                        app.name = new_name.unwrap();
                    }
                    if new_image.is_some() {
                        app.image = new_image.unwrap();
                    }
                    if new_port.is_some() {
                        app.port = new_port.unwrap();
                    }
                    if new_command.is_some() {
                        app.command = new_command.unwrap();
                    }
                    *app_wrap = Some(app);
                    Ok(())
                },
            )?;

            let mut iter = Envs::<T>::iter_prefix(app_id);
            let mut id = 0;

            // 遍历设置
            while let Some(setting) = iter.next() {
                id = setting.0;
                // 处理更新和删除设置
                new_env.iter().for_each(|v| {
                    match v.etype {
                        // 更新设置
                        EditType::UPDATE(index) => {
                            if index == setting.0 {
                                <Envs<T>>::insert(
                                    app_id,
                                    setting.0,
                                    Env {
                                        k: v.k.clone(),
                                        v: v.v.clone(),
                                    },
                                );
                            }
                        }
                        // 删除设置
                        EditType::REMOVE(index) => {
                            if index == setting.0 {
                                <Envs<T>>::remove(app_id, setting.0);
                            }
                        }
                        _ => {}
                    };
                });
            }

            // add all deposit
            // 处理新增设置
            new_env.iter().for_each(|v| {
                if v.etype == EditType::INSERT {
                    id = id + 1;
                    <Envs<T>>::insert(
                        app_id,
                        id,
                        Env {
                            k: v.k.clone(),
                            v: v.v.clone(),
                        },
                    );
                }
            });

            if with_restart {
                <TaskVersion<T>>::insert(app_id, <frame_system::Pallet<T>>::block_number());
            }

            Self::deposit_event(Event::WorkUpdated {
                user: account,
                work_id: WorkId {
                    wtype: WorkType::TASK,
                    id: app_id,
                },
            });

            Ok(().into())
        }

        /// Task settings
        /// 任务设置
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn set_settings(
            origin: OriginFor<T>,
            app_id: TeeAppId,
            value: Vec<EnvInput>,
            // with restart
            // 是否重启
            with_restart: bool,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let app_account = <TaskIdAccounts<T>>::get(app_id).ok_or(Error::<T>::TaskNotExists)?;
            ensure!(who == app_account, Error::<T>::Task403);

            let mut iter = Envs::<T>::iter_prefix(app_id);
            let mut id = 0;

            // 遍历设置
            while let Some(setting) = iter.next() {
                id = setting.0;

                // 处理更新和删除设置
                value.iter().for_each(|v| {
                    match v.etype {
                        // update
                        // 更新设置
                        EditType::UPDATE(index) => {
                            if index == setting.0 {
                                <Envs<T>>::insert(
                                    app_id,
                                    setting.0,
                                    Env {
                                        k: v.k.clone(),
                                        v: v.v.clone(),
                                    },
                                );
                            }
                        }
                        // remove
                        // 删除设置
                        EditType::REMOVE(index) => {
                            if index == setting.0 {
                                <Envs<T>>::remove(app_id, setting.0);
                            }
                        }
                        _ => {}
                    };
                });
            }

            // inster
            // 新增设置
            value.iter().for_each(|v| {
                if v.etype == EditType::INSERT {
                    id = id + 1;
                    <Envs<T>>::insert(
                        app_id,
                        id,
                        Env {
                            k: v.k.clone(),
                            v: v.v.clone(),
                        },
                    );
                }
            });

            if with_restart {
                <TaskVersion<T>>::insert(app_id, <frame_system::Pallet<T>>::block_number());
            }

            Self::deposit_event(Event::WorkUpdated {
                user: app_account,
                work_id: WorkId {
                    wtype: WorkType::TASK,
                    id: app_id,
                },
            });

            Ok(().into())
        }

        /// Task charge
        /// 任务充值
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn charge(
            origin: OriginFor<T>,
            id: TeeAppId,
            deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // Transfer fee to task account
            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(
                wetee_assets::NATIVE_ASSET_ID,
                who.clone(),
                Self::task_id_account(id),
                deposit,
            )?;

            Self::deposit_event(Event::<T>::Charge {
                from: who.clone(),
                to: Self::task_id_account(id),
                amount: deposit,
            });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get app id account
        /// 获取 Task 合约账户
        pub fn task_id_account(app_id: TeeAppId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(WorkId {
                id: app_id,
                wtype: WorkType::TASK,
            })
        }

        /// Get app id from account
        /// 获取账户中合约信息
        pub fn task_id_from_account(x: T::AccountId) -> WorkId {
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
            <TEETasks<T>>::try_mutate_exists(
                account.clone(),
                app_id,
                |app_wrap| -> result::Result<(), DispatchError> {
                    let mut app = app_wrap.take().ok_or(Error::<T>::TaskNotExists)?;
                    app.status = 2;
                    *app_wrap = Some(app);
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::<T>::TaskStop {
                creator: account.clone(),
                id: app_id,
            });

            Ok(())
        }

        /// Pay run fee
        /// 支付运行费用
        pub fn pay_run_fee(
            wid: WorkId,
            fee: BalanceOf<T>,
            to: T::AccountId,
        ) -> result::Result<u8, DispatchError> {
            let who = Self::task_id_account(wid.id);
            let app_total = wetee_assets::Pallet::<T>::free_balance(0, &who);
            log::warn!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++app_total {:?}",
                app_total
            );

            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(0, who.clone(), to.clone(), fee)?;

            Self::deposit_event(Event::<T>::PayRunFee {
                from: who.clone(),
                to,
                amount: fee,
            });

            // 任务只执行一次，执行后停止
            Ok(2)
        }

        /// Get fee
        /// 获取费用
        pub fn get_fee(id: TeeAppId) -> result::Result<BalanceOf<T>, DispatchError> {
            let app_account = <TaskIdAccounts<T>>::get(id).ok_or(Error::<T>::TaskNotExists)?;
            let app =
                <TEETasks<T>>::get(app_account.clone(), id).ok_or(Error::<T>::TaskNotExists)?;
            let level = app.level;

            let number = <frame_system::Pallet<T>>::block_number();

            // 获取费用
            let p = <Prices<T>>::get(level).ok_or(Error::<T>::LevelNotExists)?;
            let cos: u32 = (number - app.start_block).saturated_into::<u32>();
            let disk_all = app
                .cr
                .disk
                .iter()
                .map(|d| d.size)
                .fold(0, |acc, size| acc + size);

            return Ok(BalanceOf::<T>::from(
                (p.cpu_per_block * app.cr.cpu
                    + p.memory_per_block * app.cr.mem
                    + p.disk_per_block * disk_all) as u32
                    * cos,
            ));
        }
    }
}
