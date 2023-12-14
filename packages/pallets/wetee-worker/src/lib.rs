#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Randomness};
use frame_system::pallet_prelude::*;
use scale_info::{prelude::vec::Vec, TypeInfo};
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::RuntimeDebug;
use sp_std::result;

use orml_traits::MultiCurrency;

use wetee_primitives::types::{ClusterId, Cr, MintId, TeeAppId, WorkId};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

/// K8sCluster specific information
/// 集群信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct K8sCluster<AccountId, BlockNumber> {
    /// 节点id
    /// 节点id
    pub id: ClusterId,
    /// creator of K8sCluster
    /// 创建者
    pub account: AccountId,
    /// The block that creates the K8sCluster
    /// App创建的区块
    pub start_block: BlockNumber,
    /// Stop time
    /// 停止时间
    pub stop_block: Option<BlockNumber>,
    /// terminal time
    /// 终止时间
    pub terminal_block: Option<BlockNumber>,
    /// name of the K8sCluster.
    /// 集群名字
    pub name: Vec<u8>,
    /// img of the K8sCluster.
    /// image 目标宗旨
    pub image: Vec<u8>,
    /// ip of service
    /// 服务端口号
    pub ip: Vec<Vec<u8>>,
    /// port of service
    /// 服务端口号
    pub port: Vec<u32>,
    /// State of the App
    /// K8sCluster 状态
    pub status: u8,
}

/// 质押数据
/// deposit of computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Deposit<Balance> {
    /// Deposit amount
    /// 质押金额
    pub deposit: Balance,
    /// cpu
    /// cpu
    pub cpu: u16,
    /// memory
    /// memory
    pub memory: u16,
    /// disk
    /// disk
    pub disk: u16,
}

/// 集群证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProofOfCluster {
    /// Cluster  id
    /// 节点id
    pub cid: u64,
}

/// 工作证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProofOfWork {
    /// Cluster id
    /// 节点id
    pub cid: u64,
    /// worker id
    /// 任务 id
    pub wid: WorkId,
    /// Task log address and hash
    /// 任务日志地址及hash
    pub log_hash: Vec<u8>,
    /// task cpu memory usage
    /// 任务cpu 内存 占用
    pub cr: Cr,
    /// task cpu memory usage hash
    /// 任务cpu 内存 占用监控hash
    pub cr_hash: Vec<u8>,
}

/// 合约日志
/// Log of contract
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ContractState<BlockNumber> {
    /// block_number
    /// 区块号
    pub block_number: BlockNumber,
    /// state
    /// 状态
    pub minted: u16,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + wetee_assets::Config
        + wetee_org::Config
        + wetee_app::Config
        + wetee_task::Config
        + pallet_insecure_randomness_collective_flip::Config
    {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        /// extrinsics 权重信息
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// 用户对应集群的信息
    /// user's K8sCluster information
    #[pallet::storage]
    #[pallet::getter(fn k8s_cluster_accounts)]
    pub type K8sClusterAccounts<T: Config> =
        StorageMap<_, Identity, T::AccountId, ClusterId, OptionQuery>;

    /// The id of the next cluster to be created.
    /// 获取下一个集群id
    #[pallet::storage]
    #[pallet::getter(fn next_cluster_id)]
    pub type NextClusterId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// 集群信息
    #[pallet::storage]
    #[pallet::getter(fn k8s_clusters)]
    pub type K8sClusters<T: Config> = StorageMap<
        _,
        Identity,
        ClusterId,
        K8sCluster<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// 集群工作量证明
    /// K8sCluster proof of work
    #[pallet::storage]
    #[pallet::getter(fn proofs_of_cluster)]
    pub type ProofOfClusters<T: Config> =
        StorageMap<_, Identity, ClusterId, ProofOfCluster, OptionQuery>;

    /// 计算资源 抵押/使用
    /// computing resource
    #[pallet::storage]
    #[pallet::getter(fn crs)]
    pub type Crs<T: Config> = StorageMap<_, Identity, ClusterId, (Cr, Cr), OptionQuery>;

    /// 节点(评级,评分)
    /// computing resource
    #[pallet::storage]
    #[pallet::getter(fn scores)]
    pub type Scores<T: Config> = StorageMap<_, Identity, ClusterId, (u8, u8), OptionQuery>;

    /// 抵押信息
    /// deposit of computing resource
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageDoubleMap<
        _,
        Identity,
        ClusterId,
        Identity,
        BlockNumberFor<T>,
        Deposit<BalanceOf<T>>,
        OptionQuery,
    >;

    /// 集群包含的智能合同
    /// smart contract
    #[pallet::storage]
    #[pallet::getter(fn cluster_contracts)]
    pub type ClusterContracts<T: Config> = StorageDoubleMap<
        _,
        Identity,
        ClusterId,
        Identity,
        WorkId,
        ContractState<BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// 程序使用的智能合同 （节点id，解锁)
    /// smart contract
    #[pallet::storage]
    #[pallet::getter(fn work_contracts)]
    pub type WorkContracts<T: Config> = StorageMap<_, Identity, WorkId, ClusterId, OptionQuery>;

    /// 程序使用的智能合同日志 （节点id，日志）
    /// smart contract log
    #[pallet::storage]
    #[pallet::getter(fn work_contract_state)]
    pub type WorkContractState<T: Config> = StorageDoubleMap<
        _,
        Identity,
        WorkId,
        Identity,
        ClusterId,
        ContractState<BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// 工作任务工作量证明
    /// proof of work of task
    #[pallet::storage]
    #[pallet::getter(fn proofs_of_work)]
    pub type ProofsOfWork<T: Config> = StorageDoubleMap<
        _,
        Identity,
        WorkId,
        Identity,
        BlockNumberFor<T>,
        ProofOfWork,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new cluster has been created. [creator]
        ClusterCreated { creator: T::AccountId },
        /// A new app has been runed. [minter]
        AppRuning { minter: T::AccountId, id: u64 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// App status mismatch
        /// 程序状态不匹配
        AppStatusMismatch,
        /// Cluster is exists
        /// 集群已存在
        ClusterIsExists,
        /// Cluster is not exists
        /// 集群不存在
        ClusterNotExists,
        /// Cluster is not started
        /// 集群未启动
        ClusterNotStarted,
        /// Cluster can not stopped
        /// 集群无法停止
        ClusterCanNotStopped,
        /// Too many apps
        /// 程序数量过多
        TooManyApp,
        /// No cluster
        /// 没有集群
        NoCluster,
        /// App is not exists
        /// 程序不存在
        AppNotExists,
        /// Work is not exists
        /// 工作不存在
        WorkNotExists,
        /// Insufficient balance.
        /// 余额不足
        InsufficientBalance,
        /// Task is not exists
        /// 任务不存在
        TaskNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Worker cluster register
        /// 集群注册
        #[pallet::call_index(001)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_register(
            origin: OriginFor<T>,
            name: Vec<u8>,
            image: Vec<u8>,
            ip: Vec<Vec<u8>>,
            port: Vec<u32>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;

            // 检查集群是否存在
            ensure!(
                K8sClusterAccounts::<T>::contains_key(creator.clone()) == false,
                Error::<T>::ClusterIsExists
            );

            // 插入app
            let cid = NextClusterId::<T>::get();

            // 集群
            let cluster = K8sCluster {
                id: cid.clone(),
                account: creator.clone(),
                start_block: <frame_system::Pallet<T>>::block_number(),
                stop_block: None,
                terminal_block: None,
                name,
                image,
                ip,
                port,
                status: 1,
            };

            // 保存集群用户信息
            K8sClusterAccounts::<T>::insert(creator.clone(), cid.clone());
            // 保存集群信息
            K8sClusters::<T>::insert(cid.clone(), cluster);
            // 初始化资源
            // initialize resource
            Crs::<T>::insert(
                cid.clone(),
                (
                    Cr {
                        cpu: 0,
                        memory: 0,
                        disk: 0,
                    },
                    Cr {
                        cpu: 0,
                        memory: 0,
                        disk: 0,
                    },
                ),
            );
            // 初始化评级
            // initialize score
            Scores::<T>::insert(cid, (1, 5));
            <NextClusterId<T>>::mutate(|id| *id += 1);

            Self::deposit_event(Event::ClusterCreated { creator });
            Ok(().into())
        }

        /// Worker cluster mortgage
        /// 质押硬件
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_mortgage(
            origin: OriginFor<T>,
            id: ClusterId,
            cpu: u16,
            mem: u16,
            disk: u16,
            #[pallet::compact] deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            let cluster = K8sClusters::<T>::get(id).ok_or(Error::<T>::ClusterNotExists)?;

            // 检查是否是集群的主人
            ensure!(
                cluster.account == creator.clone(),
                Error::<T>::ClusterIsExists
            );

            // 添加抵押历史
            Deposits::<T>::insert(
                id,
                <frame_system::Pallet<T>>::block_number(),
                Deposit {
                    deposit,
                    cpu,
                    memory: mem,
                    disk,
                },
            );

            // 更新抵押数据
            Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                let ccr = crs.0.clone();

                // 更新抵押参数
                crs.0 = Cr {
                    cpu: ccr.cpu + cpu,
                    memory: ccr.memory + mem,
                    disk: ccr.disk + disk,
                };
                Ok(())
            })?;

            // 质押保证金
            wetee_assets::Pallet::<T>::reserve(0, creator, deposit).unwrap();

            Ok(().into())
        }

        /// Worker cluster unmortgage
        /// 解抵押
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_unmortgage(
            origin: OriginFor<T>,
            id: ClusterId,
            block_num: BlockNumberFor<T>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            let d = Deposits::<T>::get(id, block_num).unwrap();

            // 添加抵押历史
            Deposits::<T>::remove(id, block_num);

            // 更新抵押数据
            Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                let ccr = crs.0.clone();

                // 更新抵押参数
                crs.0 = Cr {
                    cpu: ccr.cpu - d.cpu,
                    memory: ccr.memory - d.memory,
                    disk: ccr.disk - d.disk,
                };
                Ok(())
            })?;

            // 释放质押保证金
            wetee_assets::Pallet::<T>::unreserve(0, creator, d.deposit).unwrap();

            Ok(().into())
        }

        /// Worker cluster upload proof of work data
        /// 提交集群的工作证明
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_proof_upload(
            origin: OriginFor<T>,
            proof: ProofOfCluster,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // 获取当前账户的集群
            let cluster_id =
                K8sClusterAccounts::<T>::get(who).ok_or(Error::<T>::ClusterNotExists)?;
            let cluster = K8sClusters::<T>::get(cluster_id).ok_or(Error::<T>::ClusterNotExists)?;

            // 检查集群是否已经开始
            ensure!(cluster.status == 1, Error::<T>::ClusterNotStarted);

            // 保存工作证明
            ProofOfClusters::<T>::insert(cluster.id.clone(), proof);

            Ok(().into())
        }

        /// Work proof of work data upload
        /// 提交工作证明
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn work_proof_upload(
            origin: OriginFor<T>,
            work_id: WorkId,
            proof: ProofOfWork,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let cluster_id =
                K8sClusterAccounts::<T>::get(who).ok_or(Error::<T>::ClusterNotExists)?;
            let contract_cluster_id =
                WorkContracts::<T>::get(work_id.clone()).ok_or(Error::<T>::WorkNotExists)?;

            ensure!(contract_cluster_id == cluster_id, Error::<T>::WorkNotExists);

            let number = <frame_system::Pallet<T>>::block_number();

            // 保存工作证明
            ProofsOfWork::<T>::insert(work_id.clone(), number, proof);

            // 支付费用
            match work_id.t {
                // 分阶段支付费用
                1 => {
                    let state = WorkContractState::<T>::get(work_id.clone(), cluster_id)
                        .ok_or(Error::<T>::WorkNotExists)?;
                    // 检查是否是重复提交状态
                    if number - state.block_number < 600u32.into() {
                        return Err(Error::<T>::WorkNotExists.into());
                    } else if number - state.block_number > 1200u32.into() {
                        WorkContractState::<T>::insert(
                            work_id.clone(),
                            cluster_id,
                            ContractState {
                                block_number: number,
                                minted: 1,
                            },
                        );
                    } else {
                        WorkContractState::<T>::insert(
                            work_id.clone(),
                            cluster_id,
                            ContractState {
                                block_number: number,
                                minted: state.minted + 1,
                            },
                        );
                    }
                    let fee = wetee_app::Pallet::<T>::get_fee(work_id.clone())?;
                    let to = Self::get_mint_account(work_id.clone(), cluster_id);
                    wetee_app::Pallet::<T>::pay_run_fee(work_id.clone(), fee, to)?;

                    let app = wetee_app::Pallet::<T>::get_app(work_id.id)?;
                    // 如果app状态为已停止，则删除工作合约
                    if app.status == 2 {
                        WorkContracts::<T>::remove(work_id.clone());
                    }
                }
                2 => {
                    let fee = wetee_task::Pallet::<T>::get_fee(work_id.clone())?;
                    let to = Self::get_mint_account(work_id.clone(), cluster_id);
                    wetee_task::Pallet::<T>::pay_run_fee(work_id, fee, to)?;
                }
                _ => return Err(Error::<T>::WorkNotExists.into()),
            }

            Ok(().into())
        }

        /// Worker cluster withdrawal
        /// 提现余额
        #[pallet::call_index(006)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_withdrawal(
            origin: OriginFor<T>,
            work_id: WorkId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let cluster_id =
                K8sClusterAccounts::<T>::get(who).ok_or(Error::<T>::ClusterNotExists)?;

            let mint_account = Self::get_mint_account(work_id.clone(), cluster_id);
            ensure!(
                wetee_assets::Pallet::<T>::free_balance(
                    wetee_assets::NATIVE_ASSET_ID,
                    &mint_account
                ) >= amount,
                Error::<T>::InsufficientBalance
            );

            Ok(().into())
        }

        /// Worker cluster stop
        /// 停止集群
        #[pallet::call_index(007)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_stop(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // 获取当前账户的集群
            let cluster_id =
                K8sClusterAccounts::<T>::get(who.clone()).ok_or(Error::<T>::ClusterNotExists)?;
            let mut cluster =
                K8sClusters::<T>::get(cluster_id).ok_or(Error::<T>::ClusterNotExists)?;

            // 检查集群是否已经开始
            ensure!(cluster.status == 1, Error::<T>::ClusterNotStarted);

            // 检查是否已经处理完所有的任务
            let cr = Crs::<T>::get(cluster_id).unwrap();
            ensure!(cr.1.cpu == 0, Error::<T>::ClusterCanNotStopped);

            let mut iter = Deposits::<T>::iter_prefix(cluster_id);
            // 解除所有的抵押
            while let Some(value) = iter.next() {
                Deposits::<T>::remove(cluster_id, value.0);
                wetee_assets::Pallet::<T>::unreserve(
                    wetee_assets::NATIVE_ASSET_ID,
                    who.clone(),
                    value.1.deposit,
                )
                .unwrap();
            }

            // 重置抵押数据
            Crs::<T>::insert(
                cluster_id,
                (
                    Cr {
                        cpu: 0,
                        memory: 0,
                        disk: 0,
                    },
                    Cr {
                        cpu: 0,
                        memory: 0,
                        disk: 0,
                    },
                ),
            );

            // 保存集群信息
            cluster.status = 3;
            K8sClusters::<T>::insert(cluster_id, cluster);

            Ok(().into())
        }

        /// Worker cluster report
        /// 投诉集群
        #[pallet::call_index(008)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_report(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Worker app deploy
        /// 部署应用
        pub fn match_app_deploy(
            account: T::AccountId,
            work_id: WorkId,
        ) -> result::Result<(), DispatchError> {
            // 获取app信息
            let mut app = wetee_app::TEEApps::<T>::get(account.clone(), work_id.clone().id)
                .ok_or(Error::<T>::AppNotExists)?;
            let app_cr = app.cr.clone();
            let id = Self::get_random_cluster(work_id.clone(), app_cr.clone(), app.level)?;

            if app.status == 0 {
                // 更新抵押数据
                Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                    let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                    let ccr = crs.1.clone();

                    // 更新抵押参数
                    crs.1 = Cr {
                        cpu: ccr.cpu - app_cr.cpu,
                        memory: ccr.memory - app_cr.memory,
                        disk: ccr.disk - app_cr.disk,
                    };
                    Ok(())
                })?;

                WorkContracts::<T>::insert(work_id.clone(), id);

                let number = <frame_system::Pallet<T>>::block_number();
                // 如果没有集群挖矿记录，则插入记录
                if !ClusterContracts::<T>::contains_key(id, work_id.clone()) {
                    ClusterContracts::<T>::insert(
                        id,
                        work_id.clone(),
                        ContractState {
                            block_number: number,
                            minted: 0,
                        },
                    );
                }
                if !WorkContractState::<T>::contains_key(work_id.clone(), id) {
                    WorkContractState::<T>::insert(
                        work_id.clone(),
                        id,
                        ContractState {
                            block_number: number,
                            minted: 0,
                        },
                    );
                }

                app.status = 1;
                wetee_app::TEEApps::<T>::insert(account, work_id.id.clone(), app);
            }

            Ok(().into())
        }

        pub fn match_task_deploy(
            account: T::AccountId,
            work_id: WorkId,
        ) -> result::Result<(), DispatchError> {
            let mut task = wetee_task::TEETasks::<T>::get(account.clone(), work_id.clone().id)
                .ok_or(Error::<T>::TaskNotExists)?;
            let task_cr = task.cr.clone();
            let id = Self::get_random_cluster(work_id.clone(), task_cr.clone(), task.level)?;

            if task.status == 0 {
                // 更新抵押数据
                Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                    let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                    let ccr = crs.1.clone();

                    // 更新抵押参数
                    crs.1 = Cr {
                        cpu: ccr.cpu - task_cr.cpu,
                        memory: ccr.memory - task_cr.memory,
                        disk: ccr.disk - task_cr.disk,
                    };
                    Ok(())
                })?;

                WorkContracts::<T>::insert(work_id.clone(), id);

                task.status = 1;
                wetee_task::TEETasks::<T>::insert(account, work_id.id.clone(), task);
            }

            Ok(().into())
        }

        /// Get random cluster
        /// 获取随机节点
        pub fn get_random_cluster(
            work_id: WorkId,
            app_cr: Cr,
            level: u8,
        ) -> result::Result<ClusterId, DispatchError> {
            let num = NextClusterId::<T>::get();
            ensure!(num > 0, Error::<T>::NoCluster);

            // 随机选择集群
            let mut randoms = Vec::new();
            let mut scores = Vec::new();
            for i in 1..1000 {
                // 获取随机数
                let random_number = Self::get_random_number(work_id.id + i);
                // 必须保证数字在集群的范围内
                let v = num - random_number % num;
                if !randoms.contains(&v) {
                    let score = Scores::<T>::get(v).unwrap();
                    let cr = Crs::<T>::get(v).unwrap();
                    // 过滤掉已经没有计算资源的集群
                    if work_id.t <= score.0
                        && cr.0.cpu - cr.1.cpu > app_cr.cpu
                        && cr.0.memory - cr.1.memory > app_cr.memory
                        && cr.0.disk - cr.1.disk > app_cr.disk
                        && score.1 == level
                    {
                        randoms.push(v);
                        scores.push(score);
                    }
                }
            }

            // 确认候选集群不为空
            ensure!(!randoms.is_empty(), Error::<T>::NoCluster);

            // 选择列表中最优的集群
            let index = scores
                .iter()
                .enumerate()
                .max_by_key(|(_idx, &val)| val)
                .map(|(idx, _val)| idx)
                .unwrap();

            log::warn!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ {:?} ===> {:?}",
                randoms,randoms[index]
            );
            return Ok(randoms[index]);
        }

        /// Get random number
        /// 获取随机数
        fn get_random_number(seed: TeeAppId) -> u64 {
            let (random_seed, _) = <pallet_insecure_randomness_collective_flip::Pallet<T>>::random(
                &(T::PalletId::get(), seed).encode(),
            );
            let random_number = <u64>::decode(&mut random_seed.as_ref())
                .expect("secure hashes should always be bigger than u64; qed");
            random_number
        }

        /// Get minted app account
        /// 获取应用挖矿账户
        pub fn get_mint_account(work_id: WorkId, cid: ClusterId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(MintId {
                id: work_id.id,
                t: work_id.t,
                cid,
            })
        }
    }
}

// fn unique_elements(arr: Vec<u64>) -> Vec<u64> {
//     let mut visited = Vec::new();
//     for &num in &arr {
//         if !visited.contains(&num) {
//             visited.push(num);
//         }
//     }
//     visited
// }
