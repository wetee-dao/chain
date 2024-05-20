#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Randomness};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use sp_runtime::traits::AccountIdConversion;
use sp_std::result;

use orml_traits::MultiCurrency;

use wetee_primitives::{traits::WorkExt,types::{ClusterId, ComCr, MintId,Cr, TeeAppId, WorkId, WorkType,ClusterLevel,TEEVersion}};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
mod types;
use types::*;
use weights::WeightInfo;

pub use pallet::*;

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
        + pallet_insecure_randomness_collective_flip::Config
    {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        /// extrinsics 权重信息
        type WeightInfo: WeightInfo;

        /// work ext function
        /// 工作扩展函数
        type WorkExt: WorkExt<Self::AccountId,BalanceOf<Self>>;
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

    #[pallet::type_value]
    pub fn DefaultForm1() -> ClusterId {
        1
    }

    /// The id of the next cluster to be created.
    /// 获取下一个集群id
    #[pallet::storage]
    #[pallet::getter(fn next_cluster_id)]
    pub type NextClusterId<T: Config> = StorageValue<_, ClusterId, ValueQuery, DefaultForm1>;

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
    pub type Crs<T: Config> = StorageMap<_, Identity, ClusterId, (ComCr, ComCr), OptionQuery>;

    /// 节点(评级,评分)
    /// computing resource
    #[pallet::storage]
    #[pallet::getter(fn scores)]
    pub type Scores<T: Config> = StorageMap<_, Identity, ClusterId, (ClusterLevel, u8), OptionQuery>;

    /// 抵押价格
    /// deposit of computing resource
    #[pallet::storage]
    #[pallet::getter(fn deposit_price)]
    pub type DepositPrices<T: Config> = StorageMap<_, Identity, ClusterLevel, DepositPrice, OptionQuery>;

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
        ClusterContractState<BlockNumberFor<T>, T::AccountId>,
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
        ContractState<BlockNumberFor<T>, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::type_value]
    pub fn DefaultForm3() -> u32 {
        600
    }
    /// Work 结算周期
    /// Work settle period
    #[pallet::storage]
    #[pallet::getter(fn stage)]
    pub type Stage<T: Config> = StorageValue<_, u32, ValueQuery, DefaultForm3>;

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

    /// work report
    #[pallet::storage]
    #[pallet::getter(fn report_of_work)]
    pub type ReportOfWork<T: Config> = StorageMap<_, Identity, WorkId, Vec<u8>, OptionQuery>;

    /// work report
    #[pallet::storage]
    #[pallet::getter(fn report_of_work_time)]
    pub type ReportOfWorkTime<T: Config> = StorageMap<_, Identity, WorkId, BlockNumberFor<T>, OptionQuery>;

    /// 投诉信息
    /// reports of work / cluster
    #[pallet::storage]
    #[pallet::getter(fn reports)]
    pub type Reports<T: Config> =
        StorageDoubleMap<_, Identity, ClusterId, Identity, WorkId, Vec<u8>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new cluster has been created. [creator]
        ClusterCreated { creator: T::AccountId },
        /// A new app has been runed. [user]
        WorkRuning { user: T::AccountId, work_id: WorkId, cluster_id: ClusterId },
        /// Work contract has been updated. [user]
        WorkContractUpdated { work_id: WorkId },
        /// Work contract has been withdrawn. [user]
        WorkContractWithdrawaled { work_id: WorkId },
        /// Work stoped
        WorkStoped { work_id: WorkId, cluster_id: ClusterId },
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
        /// Insufficient Minted Balance
        /// 合约余额不足
        InsufficientMintedBalance,
        /// Task is not exists
        /// 任务不存在
        TaskNotExists,
        /// Work is not started
        /// 工作未启动
        WorkNotStarted,
        /// Not allowed
        /// 未允许
        NotAllowed403,
        /// Cluster register miss ip
        /// 集群注册缺少ip
        ClusterRegisterMissIp,
        /// Ip format error
        /// ip格式错误
        IpFormatError,
        /// Insufficient deposit.
        /// 抵押不足
        InsufficientDeposit,
        /// Duplicate deposit.
        /// 重复抵押
        DuplicateDeposit,
        /// Level is not exists
        /// 等级不存在
        LevelNotExists,
        /// No cluster found
        /// 没有找到集群
        NoClusterFound,
        /// Work block number error
        /// 工作块高度错误
        WorkBlockNumberError,
        /// Reason too long
        /// 理由太长
        ReasonTooLong,
        /// Work type not exists
        /// 工作类型不存在
        WorkTypeNotExists,
    }

    #[derive(frame_support::DefaultNoBound)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            DepositPrices::<T>::insert(
                1,
                DepositPrice {
                    cpu_per: 10,
                    cvm_cpu_per: 10,
                    memory_per: 10,
                    cvm_memory_per: 10,
                    disk_per: 10,
                    gpu_per: 10,
                },
            );
        }
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
            ip: Vec<Ip>,
            port: u32,
            level: u8,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            // check ip
            // 检查ip
            ensure!(ip.len() > 0, Error::<T>::ClusterRegisterMissIp);
            ensure!(port > 0, Error::<T>::ClusterRegisterMissIp);

            // check cluster
            // 检查集群是否存在
            ensure!(
                K8sClusterAccounts::<T>::contains_key(creator.clone()) == false,
                Error::<T>::ClusterIsExists
            );

            // check level
            // 检查等级是否存在
            let _ = DepositPrices::<T>::get(level).ok_or(Error::<T>::LevelNotExists)?;

            // get new id
            // 获取最新的id
            let cid = NextClusterId::<T>::get();

            // cluster
            // 集群
            let cluster = K8sCluster {
                id: cid.clone(),
                account: creator.clone(),
                start_block: <frame_system::Pallet<T>>::block_number(),
                stop_block: None,
                terminal_block: None,
                name,
                ip,
                port,
                status: 1,
            };

            // save cluster user info
            // 保存集群用户信息
            K8sClusterAccounts::<T>::insert(creator.clone(), cid.clone());
            // save cluster info
            // 保存集群信息
            K8sClusters::<T>::insert(cid.clone(), cluster);
            // initialize resource
            // 初始化资源
            Crs::<T>::insert(
                cid.clone(),
                (
                    ComCr {
                        cpu: 0,
                        cvm_cpu: 0,
                        mem: 0,
                        cvm_mem: 0,
                        disk: 0,
                        gpu: 0,
                    },
                    ComCr {
                        cpu: 0,
                        cvm_cpu: 0,
                        mem: 0,
                        cvm_mem: 0,
                        disk: 0,
                        gpu: 0,
                    },
                ),
            );
            // 初始化评级
            // initialize score
            Scores::<T>::insert(cid, (level, 5));
            <NextClusterId<T>>::mutate(|id| *id += 1);

            Self::deposit_event(Event::ClusterCreated { creator });
            Ok(().into())
        }

        /// Worker cluster upload proof of work data
        /// 提交集群的工作证明
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_proof_upload(
            origin: OriginFor<T>,
            id: ClusterId,
            proof: ProofOfCluster,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            let cluster = K8sClusters::<T>::get(id).ok_or(Error::<T>::ClusterNotExists)?;

            // check user
            // 检查是否是集群的主人
            ensure!(
                cluster.account == creator.clone(),
                Error::<T>::ClusterIsExists
            );

            let cluster = K8sClusters::<T>::get(id).ok_or(Error::<T>::ClusterNotExists)?;

            // check status
            // 检查集群是否已经开始
            ensure!(cluster.status == 1, Error::<T>::ClusterNotStarted);

            // save proof
            // 保存工作证明
            ProofOfClusters::<T>::insert(cluster.id.clone(), proof);

            Ok(().into())
        }

        /// Worker cluster mortgage
        /// 质押硬件
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_mortgage(
            origin: OriginFor<T>,
            id: ClusterId,
            cpu: u32,
            mem: u32,
            cvm_cpu: u32,
            cvm_mem: u32,
            disk: u32,
            gpu: u32,
            #[pallet::compact] deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
            let cluster = K8sClusters::<T>::get(id).ok_or(Error::<T>::ClusterNotExists)?;

            // check user
            // 检查是否是集群的主人
            ensure!(
                cluster.account == creator.clone(),
                Error::<T>::ClusterIsExists
            );

            let score = Scores::<T>::get(id).ok_or(Error::<T>::LevelNotExists)?;
            let price = Self::get_level_price(score.0, cpu, mem, disk,gpu)?;

            // check deposit
            // 检查抵押金额是否足够
            ensure!(deposit >= price, Error::<T>::InsufficientDeposit);

            // check duplicate
            // 检查是否已经抵押
            ensure!(
                Deposits::<T>::get(id, <frame_system::Pallet<T>>::block_number()).is_none(),
                Error::<T>::DuplicateDeposit
            );

            // add deposit
            // 添加抵押历史
            Deposits::<T>::insert(
                id,
                <frame_system::Pallet<T>>::block_number(),
                Deposit {
                    deposit,
                    cpu,
                    mem,
                    cvm_cpu,
                    cvm_mem,
                    disk,
                    gpu,
                },
            );

            // add cpu mem disk
            // 更新抵押数据
            Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                let ccr = crs.0.clone();

                // 更新抵押参数
                crs.0 = ComCr {
                    cpu: ccr.cpu + cpu,
                    mem: ccr.mem + mem,
                    cvm_cpu: ccr.cvm_cpu + cvm_cpu,
                    cvm_mem: ccr.cvm_mem + cvm_mem,
                    disk: ccr.disk + disk,
                    gpu: ccr.gpu + gpu,
                };

                *c = Some(crs);
                Ok(())
            })?;

            // reserve assets
            // 质押保证金
            wetee_assets::Pallet::<T>::reserve(0, creator, deposit)?;

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
            let d = Deposits::<T>::get(id, block_num).ok_or(Error::<T>::ClusterNotExists)?;

            let cluster = K8sClusters::<T>::get(id).ok_or(Error::<T>::ClusterNotExists)?;

            // check user
            // 检查是否是集群的主人
            ensure!(
                cluster.account == creator.clone(),
                Error::<T>::ClusterIsExists
            );

            // add deposit
            // 添加抵押历史
            Deposits::<T>::remove(id, block_num);

            // add cpu mem disk
            // 更新抵押数据
            Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                let ccr = crs.0.clone();

                // 更新抵押参数
                crs.0 = ComCr {
                    cpu: ccr.cpu - d.cpu,
                    mem: ccr.mem - d.mem,
                    cvm_mem: ccr.cvm_mem - d.cvm_mem,
                    cvm_cpu: ccr.cvm_cpu - d.cvm_cpu,
                    disk: ccr.disk - d.disk,
                    gpu: ccr.gpu - d.gpu,
                };
                *c = Some(crs);
                Ok(())
            })?;

            // release assets
            // 释放质押保证金
            wetee_assets::Pallet::<T>::unreserve(0, creator, d.deposit)?;

            Ok(().into())
        }

        /// Work proof of work data upload
        /// 提交工作证明
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn work_proof_upload(
            origin: OriginFor<T>,
            work_id: WorkId,
            proof: Option<ProofOfWork>,
            report: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let cluster_id =
                K8sClusterAccounts::<T>::get(who).ok_or(Error::<T>::ClusterNotExists)?;
            let contract_cluster_id =
                WorkContracts::<T>::get(work_id.clone()).ok_or(Error::<T>::WorkNotExists)?;

            ensure!(contract_cluster_id == cluster_id, Error::<T>::NotAllowed403);

            let number = <frame_system::Pallet<T>>::block_number();

            if report.is_some() {
                let new_report = report.unwrap();
                let creport = ReportOfWork::<T>::get(work_id.clone());
                if creport.is_none() || creport.unwrap() != new_report {
                    ReportOfWork::<T>::insert(work_id.clone(),new_report);
                    ReportOfWorkTime::<T>::insert(work_id.clone(),number);
                }
            }

            // 如果未提交证明，则直接返回，不继续计费和更新证明
            if proof.is_none() {
                return Ok(().into());
            }

            // check status
            // 保存工作证明
            ProofsOfWork::<T>::insert(work_id.clone(), number, proof.unwrap());
        
            // 查询工作合约状态
            let state = WorkContractState::<T>::get(work_id.clone(), cluster_id).ok_or(Error::<T>::WorkNotExists)?;

            // 查询 work info
            let (owner_account,cr,work_status,_,tee_version) = <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;
            
            // check status
            // 检查work的状态,如果未开始状态，则报错
            if work_status == 0 || work_status == 2 {
                return Err(Error::<T>::WorkNotStarted.into());
            }

            // pay fee
            // 支付费用
            if work_id.wtype == WorkType::APP || work_id.wtype == WorkType::GPU  {
                // Pay fees in stages
                // 分阶段支付费用
                let stage: u32 = Stage::<T>::get();

                // 检查是否是重复提交状态
                if number - state.block_number < stage.into() {
                    // 未到达再次提交工作量证明的时间
                    if work_status == 3 {
                        return Err(Error::<T>::WorkBlockNumberError.into());
                    }  else if work_status == 1 {
                        // 更新合约工作状态
                        WorkContractState::<T>::insert(
                            work_id.clone(),
                            cluster_id,
                            ContractState {
                                block_number: number,
                                minted: state.minted,
                                withdrawal: state.withdrawal,
                            },
                        );
                        // 设置工作的状态
                        <T as pallet::Config>::WorkExt::set_work_status(work_id.clone(), 3)?;
                        return Ok(().into());
                    }
                } else if number - state.block_number > (stage * 2).into() {
                    // More than 2 cycles, only pay once, TODO, reduce service points
                    // TODO，超过2个周期，只支付一次费用，减少服务积分
                }
            }

            let fee = <T as pallet::Config>::WorkExt::calculate_fee(work_id.clone())?;
            let to = Self::get_mint_account(work_id.clone(), cluster_id);
            let status = <T as pallet::Config>::WorkExt::pay_run_fee(
                work_id.clone(),
                to,
                fee,
            )?;
            log::warn!(
                "pay_run_fee ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ {:?} {:?} {:?}",
                work_id.wtype,work_id.id,fee
            );

            if status == 2 {
                Self::try_stop_work(
                    cluster_id, 
                    work_id.clone(), 
                    cr,
                    owner_account,
                    tee_version
                )?;
            }else {
                WorkContractState::<T>::insert(
                    work_id.clone(),
                    cluster_id,
                    ContractState {
                        block_number: number,
                        minted: state.minted + fee,
                        withdrawal: state.withdrawal,
                    },
                );  

                Self::deposit_event(Event::WorkContractUpdated {
                    work_id: work_id.clone(),
                });
            }

            Ok(().into())
        }

        /// Worker cluster withdrawal
        /// 提现余额
        #[pallet::call_index(006)]
        #[pallet::weight(T::DbWeight::get().reads_writes(10, 20)  + Weight::from_all(40_000))]
        pub fn cluster_withdrawal(
            origin: OriginFor<T>,
            work_id: WorkId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let cluster_id =
                K8sClusterAccounts::<T>::get(who.clone()).ok_or(Error::<T>::ClusterNotExists)?;
            let contract_cluster_id =
                WorkContracts::<T>::get(work_id.clone()).ok_or(Error::<T>::WorkNotExists)?;

            ensure!(contract_cluster_id == cluster_id, Error::<T>::NotAllowed403);

            let mint_account = Self::get_mint_account(work_id.clone(), cluster_id);
            ensure!(
                wetee_assets::Pallet::<T>::free_balance(
                    wetee_assets::NATIVE_ASSET_ID,
                    &mint_account
                ) >= amount,
                Error::<T>::InsufficientBalance
            );

            let state = WorkContractState::<T>::get(work_id.clone(), cluster_id)
                .ok_or(Error::<T>::WorkNotExists)?;
            #[cfg(test)]
            println!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ state.minted: {:?}",
                state.minted 
            );
            ensure!(
                state.minted >= amount,
                Error::<T>::InsufficientMintedBalance
            );

            // 将抵押转移到目标账户
            wetee_assets::Pallet::<T>::try_transfer(0, mint_account, who, amount)?;

            WorkContractState::<T>::insert(
                work_id.clone(),
                cluster_id,
                ContractState {
                    block_number: state.block_number,
                    minted: state.minted - amount,
                    withdrawal: state.withdrawal + amount,
                },
            );

            Self::deposit_event(Event::WorkContractWithdrawaled {
                work_id: work_id.clone(),
            });

            Ok(().into())
        }

        /// Worker cluster stop
        /// 停止集群
        #[pallet::call_index(007)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_stop(origin: OriginFor<T>, id: ClusterId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // get user cluster
            // 获取当前账户的集群
            let cluster_id =
                K8sClusterAccounts::<T>::get(who.clone()).ok_or(Error::<T>::ClusterNotExists)?;
            ensure!(cluster_id == id, Error::<T>::ClusterNotExists);

            let mut cluster =
                K8sClusters::<T>::get(cluster_id).ok_or(Error::<T>::ClusterNotExists)?;

            // Check if the cluster has started
            // 检查集群是否已经开始
            ensure!(cluster.status == 1, Error::<T>::ClusterNotStarted);

            // Check if all tasks have been processed
            // 检查是否已经处理完所有的任务
            let cr = Crs::<T>::get(cluster_id).unwrap();
            ensure!(cr.1.cpu == 0, Error::<T>::ClusterCanNotStopped);

            let mut iter = Deposits::<T>::iter_prefix(cluster_id);

            // Release all mortgages
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

            // Reset mortgage data
            // 重置抵押数据
            Crs::<T>::insert(
                cluster_id,
                (
                    ComCr {cpu: 0,mem: 0,cvm_cpu:0,cvm_mem:0,disk: 0,gpu: 0},
                    ComCr {cpu: 0,mem: 0,cvm_cpu:0,cvm_mem:0,disk: 0,gpu: 0},
                ),
            );

            // Stop the cluster
            // 保存集群信息
            cluster.status = 3;
            K8sClusters::<T>::insert(cluster_id, cluster);

            Ok(().into())
        }

        /// Worker cluster report
        /// 投诉集群
        #[pallet::call_index(008)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_report(
            origin: OriginFor<T>,
            cluster_id: ClusterId,
            work_id: WorkId,
            reason: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(reason.len() < 255, Error::<T>::ReasonTooLong);

            if let Some(cluster) = K8sClusters::<T>::get(cluster_id) {
                ensure!(cluster.status != 0, Error::<T>::ClusterNotStarted);
            } else {
                return Err(Error::<T>::ClusterNotExists.into());
            }

            let (owner_account,_,_,status,_) = <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;
            ensure!(owner_account == who, Error::<T>::NotAllowed403);
            ensure!(status != 0, Error::<T>::WorkNotStarted);

            Reports::<T>::insert(cluster_id, work_id, reason);

            Ok(().into())
        }

        /// Worker report stop
        /// 停止投诉
        #[pallet::call_index(009)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn report_close(
            origin: OriginFor<T>,
            cluster_id: ClusterId,
            work_id: WorkId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            if let Some(cluster) = K8sClusters::<T>::get(cluster_id) {
                ensure!(cluster.status != 0, Error::<T>::ClusterNotStarted);
            } else {
                return Err(Error::<T>::ClusterNotExists.into());
            }

            let (owner_account,_,_,status,_) = <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;
            ensure!(owner_account == who, Error::<T>::NotAllowed403);
            ensure!(status != 0, Error::<T>::WorkNotStarted);

            Reports::<T>::remove(cluster_id, work_id);
            Ok(().into())
        }
    
        /// Work stop
        /// 停止应用
        #[pallet::call_index(010)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn work_stop(
            origin: OriginFor<T>,
            work_id: WorkId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let (owner_account,cr,_,_,tee_version) = <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;
            ensure!(owner_account == who, Error::<T>::NotAllowed403);

            let cid = WorkContracts::<T>::get(work_id.clone()).ok_or(Error::<T>::WorkNotExists)?;
   
            // 删除合约
            Self::try_stop_work(
                cid, 
                work_id.clone(), 
                cr,
                owner_account,
                tee_version
            )?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Work deploy
        /// 部署应用
        pub fn match_deploy(
            work_id: WorkId,
            match_id: Option<TeeAppId>,
        ) -> result::Result<bool, DispatchError> {
            let (account,cr,level,status,tee_version) = <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;
            let disk_all = cr.disk.iter().map(|d| d.size).fold(0, |acc, size| acc + size);
            let mut cpu = 0;
            let mut mem = 0;
            let mut cvm_cpu = 0;
            let mut cvm_mem = 0;
            if tee_version == TEEVersion::CVM {
                cvm_cpu = cr.cpu;
                cvm_mem = cr.mem;
            } else if tee_version == TEEVersion::SGX {
                cpu = cr.cpu;
                mem = cr.mem;
            }
            let app_cr = ComCr{
                cpu: cpu,
                mem: mem,
                cvm_cpu: cvm_cpu,
                cvm_mem: cvm_mem,
                disk: disk_all,
                gpu: cr.gpu,
            };
            
            let id =
                Self::get_random_cluster(work_id.clone(), app_cr.clone(), level, match_id)?;

            // If the id is 0, it means there is no matching node and it will be put into the next block calculation
            // id 为 0 表示没有匹配的节点，放入下一个区块计算
            if id == 0 {
                return Ok(false);
            }

            if status == 0 || status == 4 {
                // update app cr
                // 更新抵押数据
                Crs::<T>::try_mutate_exists(id, |c| -> result::Result<(), DispatchError> {
                    let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                    let ccr = crs.1.clone();
 
                    // 更新抵押参数
                    crs.1 = ComCr {
                        cpu: ccr.cpu + app_cr.cpu,
                        mem: ccr.mem + app_cr.mem,
                        cvm_cpu: ccr.cvm_cpu + app_cr.cvm_cpu,
                        cvm_mem: ccr.cvm_mem + app_cr.cvm_mem,
                        disk: ccr.disk + app_cr.disk,
                        gpu: ccr.gpu + app_cr.gpu,
                    };
                    *c = Some(crs);
                    Ok(())
                })?;

                // 添加合约
                WorkContracts::<T>::insert(work_id.clone(), id);

                // 获取当前区块高度
                let number = <frame_system::Pallet<T>>::block_number();

                // 如果没有集群挖矿记录，则插入记录
                if !ClusterContracts::<T>::contains_key(id, work_id.clone()) {
                    ClusterContracts::<T>::insert(
                        id,
                        work_id.clone(),
                        ClusterContractState {
                            user: account.clone(),
                            work_id: work_id.clone(),
                            start_number: number,
                        },
                    );
                }

                // 如果没有合约状态记录，则插入记录
                if !WorkContractState::<T>::contains_key(work_id.clone(), id) {
                    WorkContractState::<T>::insert(
                        work_id.clone(),
                        id,
                        ContractState {
                            minted: 0u32.into(),
                            withdrawal: 0u32.into(),
                            block_number: number,
                        },
                    );
                }

                // 设置工作的状态
                <T as pallet::Config>::WorkExt::set_work_status(work_id.clone(), 1)?;

                // Runing event
                // 运行事件
                Self::deposit_event(Event::WorkRuning {
                    user: account,
                    work_id,
                    cluster_id:id,
                });
            }

            Ok(true)
        }

        /// Get random cluster
        /// 获取随机节点
        pub fn get_random_cluster(
            work_id: WorkId,
            app_cr: ComCr,
            level: ClusterLevel,
            match_id: Option<ClusterId>,
        ) -> result::Result<ClusterId, DispatchError> {
            let num = NextClusterId::<T>::get() - 1;
            if num == 0 {
                return Ok(0);
            }

            if match_id.is_some() {
                return Ok(match_id.unwrap());
            }

            // 随机选择集群
            let mut randoms = Vec::new();
            let mut scores = Vec::new();

            #[cfg(test)]
            println!("+++++++++++++++++++++++++++ {:?}", app_cr);

            // 获取随机数
            let random_base = Self::get_random_number(work_id.id);
            for i in 1..100 {
                let random_number = random_base + i;
                // 必须保证数字在集群的范围内 集群数字是从1开始的
                let mut v = random_number % num;
                // 避免数字溢出
                if v != 2 ^ 64 - 1 {
                    v = v + 1;
                }
                if !randoms.contains(&v) {
                    let score = Scores::<T>::get(v).ok_or(Error::<T>::ClusterNotExists)?;
                    let cr = Crs::<T>::get(v).ok_or(Error::<T>::ClusterNotExists)?;
                    #[cfg(test)]
                    println!(
                        "---------------------------num {:?} v: {:?} score: {:?} cr: {:?}",
                        num, v, score, cr
                    );
                    // 过滤掉已经没有计算资源的集群
                    if level == score.0
                        && cr.0.cpu - cr.1.cpu > app_cr.cpu
                        && cr.0.mem - cr.1.mem > app_cr.mem
                        && cr.0.cvm_cpu - cr.1.cvm_cpu > app_cr.cvm_cpu
                        && cr.0.cvm_mem - cr.1.cvm_mem > app_cr.cvm_mem
                        && cr.0.disk - cr.1.disk > app_cr.disk
                    {
                        randoms.push(v);
                        scores.push(score);
                    }
                }
                if randoms.len() >= 10 {
                    break;
                }
            }

            #[cfg(test)]
            println!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++get_random_cluster num: {:?} randoms: {:?}",
                num,randoms
            );

            // 确认候选集群不为空
            if randoms.is_empty() {
                return Ok(0);
            }

            // 选择列表中最优的集群
            let index = scores
                .iter()
                .enumerate()
                .max_by_key(|(_idx, &val)| val)
                .map(|(idx, _val)| idx)
                .unwrap();

            log::warn!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++get_random_cluster {:?} ===> {:?}",
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
                wtype: work_id.wtype,
                cid,
            })
        }

        /// Get level price
        /// 获取节点价格
        pub fn get_level_price(
            level: u8,
            cpu: u32,
            mem: u32,
            disk: u32,
            gpu: u32,
        ) -> result::Result<BalanceOf<T>, DispatchError> {
            let p = DepositPrices::<T>::get(level).ok_or(Error::<T>::LevelNotExists)?;
            return Ok(BalanceOf::<T>::from(
                cpu as u32 * p.cpu_per as u32
                    + mem as u32 * p.memory_per as u32
                    + disk as u32 * p.disk_per as u32
                    + gpu as u32 * p.gpu_per as u32,
            ));
        }

        /// try to stop work
        /// 尝试停止工作
        pub fn try_stop_work(
            cluster_id: ClusterId,
            work_id: WorkId,
            cr: Cr,
            owner_account: T::AccountId,
            tee_version: TEEVersion,
        ) -> result::Result<(), DispatchError> {
            // 如果app状态为已停止，则删除工作合约
            WorkContracts::<T>::remove(work_id.clone());
            ClusterContracts::<T>::remove(cluster_id,work_id.clone());
            // 更新抵押数据
            Crs::<T>::try_mutate_exists(
                cluster_id,
                |c| -> result::Result<(), DispatchError> {
                    let mut crs = c.take().ok_or(Error::<T>::ClusterNotExists)?;
                    let ccr = crs.1.clone();

                    let disk_all = cr.clone().disk.iter().map(|d| d.size).fold(0, |acc, size| acc + size);

                    // 计算抵押参数
                    let mut cpu = 0;
                    let mut mem = 0;
                    let mut cvm_cpu = 0;
                    let mut cvm_mem = 0;
                    if tee_version == TEEVersion::CVM {
                        cvm_cpu = cr.cpu;
                        cvm_mem = cr.mem;
                    } else if tee_version == TEEVersion::SGX {
                        cpu = cr.cpu;
                        mem = cr.mem;
                    }

                    // 更新抵押参数
                    crs.1 = ComCr {
                        cpu: ccr.cpu - cpu,
                        mem: ccr.mem - mem,
                        cvm_cpu: ccr.cvm_cpu - cvm_cpu,
                        cvm_mem: ccr.cvm_mem - cvm_mem,
                        disk: ccr.disk - disk_all,
                        gpu: ccr.gpu - cr.gpu,
                    };
                    *c = Some(crs);
                    Ok(())
                },
            )?;

            // 删除应用
            <T as pallet::Config>::WorkExt::try_stop(owner_account,work_id.clone())?;

            Ok(())
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
