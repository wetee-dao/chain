#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::traits::Randomness;
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::result;

use orml_traits::MultiCurrency;

use wetee_primitives::types::{TeeAppId, WorkerId};

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
    pub id: u64,
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

/// 计算资源
/// computing resource
#[derive(PartialEq, Eq, Default, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Cr {
    pub cpu: u16,
    pub memory: u16,
    pub disk: u16,
}

/// 质押数据
/// deposit of computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Deposit<Balance> {
    pub deposit: Balance,
    pub cpu: u16,
    pub memory: u16,
    pub disk: u16,
}

/// 集群证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProofOfCluster {
    /// 节点id
    /// 节点id
    pub cid: u64,
}

/// 工作证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, RuntimeDebug, TypeInfo)]
pub struct ProofOfWork {
    /// 节点id
    /// 节点id
    pub cid: u64,
    /// worker id
    /// worker id
    pub wid: WorkerId,
    /// 任务日志地址及hash
    /// task log address and hash
    pub logs: Vec<u8>,
    /// 任务cpu 内存 占用
    /// task cpu memory usage
    pub cr: Cr,
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
        StorageMap<_, Identity, T::AccountId, u64, OptionQuery>;

    /// 集群工作量证明
    /// K8sCluster proof of work
    pub type ProofsOfCluster<T: Config> = StorageMap<_, Identity, u64, ProofOfCluster, OptionQuery>;

    /// 工作任务工作量证明
    /// proof of work of task
    pub type ProofsOfWork<T: Config> = StorageMap<_, Identity, WorkerId, ProofOfWork, OptionQuery>;

    /// The id of the next cluster to be created.
    /// 获取下一个集群id
    #[pallet::storage]
    #[pallet::getter(fn next_cluster_id)]
    pub type NextClusterId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// 集群信息
    #[pallet::storage]
    #[pallet::getter(fn k8s_clusters)]
    pub type K8sClusters<T: Config> =
        StorageMap<_, Identity, u64, K8sCluster<T::AccountId, BlockNumberFor<T>>, OptionQuery>;

    /// 计算资源 抵押/使用
    /// computing resource
    #[pallet::storage]
    #[pallet::getter(fn crs)]
    pub type Crs<T: Config> = StorageMap<_, Identity, u64, (Cr, Cr), OptionQuery>;

    /// 节点(评级,评分)
    /// computing resource
    #[pallet::storage]
    #[pallet::getter(fn scores)]
    pub type Scores<T: Config> = StorageMap<_, Identity, u64, (u8, u8), OptionQuery>;

    /// 抵押信息
    /// deposit of computing resource
    #[pallet::storage]
    #[pallet::getter(fn deposits)]
    pub type Deposits<T: Config> = StorageDoubleMap<
        _,
        Identity,
        u64,
        Identity,
        BlockNumberFor<T>,
        Deposit<BalanceOf<T>>,
        OptionQuery,
    >;

    /// 程序使用的智能合同
    /// smart contract
    #[pallet::storage]
    #[pallet::getter(fn match_contract)]
    pub type MatchContract<T: Config> = StorageMap<_, Identity, u64, (u8, u8), OptionQuery>;

    /// 智能合约工作证明
    /// Contract proof of work
    #[pallet::storage]
    #[pallet::getter(fn contract_proofs)]
    pub type ContractProofs<T: Config> =
        StorageDoubleMap<_, Identity, u64, Identity, BlockNumberFor<T>, ProofOfWork, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClusterCreated { creator: T::AccountId },
        AppRuning { minter: T::AccountId, id: u64 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        AppStatusMismatch,
        ClusterIsExists,
        ClusterNotExists,
        TooManyApp,
        NoCluster,
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

            // 保存集群
            K8sClusterAccounts::<T>::insert(creator.clone(), cid.clone());
            K8sClusters::<T>::insert(cid.clone(), cluster);
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
            Scores::<T>::insert(cid, (1, 5));
            Self::deposit_event(Event::ClusterCreated { creator });

            Ok(().into())
        }

        /// Worker cluster mortgage
        /// 质押
        /// mortgage of computing resource
        #[pallet::call_index(002)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_mortgage(
            origin: OriginFor<T>,
            id: u64,
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
            id: u64,
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
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Worker cluster withdrawal
        /// 提现余额
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_withdrawal(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Worker cluster stop
        #[pallet::call_index(006)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_stop(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }

        /// Worker cluster report
        #[pallet::call_index(007)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cluster_report(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // let creator = ensure_signed(origin)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn match_app_deploy(
            _account: T::AccountId,
            msg_id: WorkerId,
        ) -> result::Result<(), DispatchError> {
            // let num = NextClusterId::<T>::get();
            let num = 9999;
            ensure!(num > 0, Error::<T>::NoCluster);

            // 随机选择集群
            let mut randoms = Vec::new();
            let mut scores = Vec::new();
            for i in 1..100 {
                let random_number = Self::get_random(msg_id.id + i);
                let v = num - random_number % num;
                if !randoms.contains(&v) {
                    let score = Scores::<T>::get(v).unwrap();
                    let cr = Crs::<T>::get(v).unwrap();
                    // 过滤掉已经没有计算资源的集群
                    if msg_id.t <= score.0
                        && cr.0.cpu - cr.1.cpu > 0
                        && cr.0.memory - cr.1.memory > 0
                        && cr.0.disk - cr.1.disk > 0
                    {
                        randoms.push(v);
                        scores.push(score);
                    }
                }
            }

            // 确认有候选集群
            if randoms.is_empty() {
                return Err(Error::<T>::NoCluster.into());
            }

            let index = randoms
                .iter()
                .enumerate()
                .max_by_key(|(_idx, &val)| val)
                .map(|(idx, _val)| idx)
                .unwrap();

            let id = randoms[index];
            log::warn!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ {:?} ===> {:?}",
                randoms,id
            );

            // 通过积分和类型选择最优集群

            // let item_ids = Self::get_active_items(100);

            // let tobedeploy_books = TobedeployBook::batch(item_ids);

            // // 合并处理撮合
            // let tobedeploys = Self::merge_and_match(tobedeploy_books);

            // for item_id in item_ids {
            //     TobedeployB
            // ook::insert(item_id, tobedeploys[item_id]);
            // }

            // fn merge_and_match(books: BTreeMap<ItemId, Vec<Tobedeploy>>) -> BTreeMap<ItemId, Vec<Tobedeploy>> {
            // // 合并订单并排序
            // let mut tobedeploys = Vec::new();
            // for book in books.values() {
            //     tobedeploys.append(&mut book);
            // }
            // tobedeploys.sort_by(|a, b| b.price.cmp(&a.price));

            // // 分块撮合
            // let chunk_size = 1000;
            // for chunk in tobedeploys.chunks(chunk_size) {
            //     // 多线程分块处理
            //     crossbeam::scope(|scope| {
            //         // 并行撮合
            //         let handles = chunk.iter().map(|buy| {
            //             scope.spawn(move |_| {
            //                 Self::match_tobedeploys_in_chunk(buy, &tobedeploys);
            //             })
            //         });
            //         for handle in handles {
            //             handle.join().unwrap();
            //         }
            //     })
            //     .unwrap();
            // }

            // // 分拆回订单簿
            // let mut books: BTreeMap<ItemId, Vec<Tobedeploy>> = BTreeMap::new();
            // for tobedeploy in tobedeploys {
            //     books
            //         .entry(tobedeploy.item_id)
            //         .or_insert_with(Vec::new)
            //         .push(tobedeploy);
            // }
            // books
            // }
            Ok(().into())
        }

        pub fn match_app_task() -> result::Result<(), DispatchError> {
            Ok(().into())
        }

        fn get_random(seed: TeeAppId) -> u64 {
            let (random_seed, _) = <pallet_insecure_randomness_collective_flip::Pallet<T>>::random(
                &(T::PalletId::get(), seed).encode(),
            );
            let random_number = <u64>::decode(&mut random_seed.as_ref())
                .expect("secure hashes should always be bigger than u32; qed");
            random_number
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
