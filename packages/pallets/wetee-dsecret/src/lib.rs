#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_std::prelude::Vec;

use wetee_primitives::types::{ClusterId, WorkId};

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

    #[cfg(feature = "runtime-benchmarks")]
    pub trait BenchmarkHelper<Public, AccountId, Signature> {
        fn signer() -> (Public, AccountId);
        fn sign(signer: &Public, message: &[u8]) -> Signature;
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl
        BenchmarkHelper<
            sp_runtime::MultiSigner,
            sp_runtime::AccountId32,
            sp_runtime::MultiSignature,
        > for ()
    {
        fn signer() -> (sp_runtime::MultiSigner, sp_runtime::AccountId32) {
            let public = sp_io::crypto::sr25519_generate(0.into(), None);
            let account = sp_runtime::MultiSigner::Sr25519(public).into_account();
            (public.into(), account)
        }
        fn sign(signer: &sp_runtime::MultiSigner, message: &[u8]) -> sp_runtime::MultiSignature {
            sp_runtime::MultiSignature::Sr25519(
                sp_io::crypto::sr25519_sign(0.into(), &signer.clone().try_into().unwrap(), message)
                    .unwrap(),
            )
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_org::Config + wetee_worker::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// Off-Chain signature type.
        ///
        /// Can verify whether an `Self::OffchainPublic` created a signature.
        type OffchainSignature: Verify<Signer = Self::OffchainPublic> + Parameter;

        /// Off-Chain public key.
        ///
        /// Must identify as an on-chain `Self::AccountId`.
        type OffchainPublic: IdentifyAccount<AccountId = Self::AccountId>;

        #[cfg(feature = "runtime-benchmarks")]
        /// A set of helper functions for benchmarking.
        type Helper: BenchmarkHelper<Self::OffchainPublic, Self::AccountId, Self::OffchainSignature>;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// 代码版本
    #[pallet::storage]
    #[pallet::getter(fn code_signature)]
    pub type CodeSignature<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

    /// 代码打包签名人
    #[pallet::storage]
    #[pallet::getter(fn code_signer)]
    pub type CodeSigner<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

    /// The id of the next node to be created.
    /// 获取下一个 node id
    #[pallet::storage]
    #[pallet::getter(fn next_node_id)]
    pub type NextNodeId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// dkg 节点列表
    #[pallet::storage]
    #[pallet::getter(fn nodes)]
    pub type Nodes<T: Config> = StorageMap<_, Identity, u64, T::AccountId, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NodeRegister {
            node: T::AccountId,
        },
        CodeUpdated {
            signature: Vec<u8>,
            signer: Vec<u8>,
        },
        ClusterProofUpload {
            cid: ClusterId,
            report: Vec<u8>,
            pubs: Vec<T::AccountId>,
            sigs: Vec<T::OffchainSignature>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// 无效的用户，无权调用
        Call403,
        /// 无效的签名
        OffchainSigError,
        /// 签名数量不够
        OffchainSigNotEnough,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 注册 dkg 节点
        /// register dkg node
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::register_node())]
        pub fn register_node(
            origin: OriginFor<T>,
            sender: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_signed_or_root(origin)?;

            let nid = <NextNodeId<T>>::get();

            // 添加节点
            <Nodes<T>>::insert(nid, sender.clone());

            // 增加 node id
            <NextNodeId<T>>::put(nid + 1);
            Self::deposit_event(Event::NodeRegister { node: sender });

            Ok(().into())
        }

        /// 上传共识节点代码
        /// update consensus node code
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::upload_code())]
        pub fn upload_code(
            origin: OriginFor<T>,
            signature: Vec<u8>,
            signer: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_signed_or_root(origin)?;

            // 更新代码hash
            <CodeSignature<T>>::set(signature.clone());
            // 更新代码签名人
            <CodeSigner<T>>::set(signer.clone());

            Self::deposit_event(Event::CodeUpdated {
                signature: signature.to_vec(),
                signer: signer.to_vec(),
            });

            Ok(().into())
        }

        /// 上传共识节点代码
        /// update consensus node code
        #[pallet::call_index(003)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::upload_cluster_proof())]
        pub fn upload_cluster_proof(
            origin: OriginFor<T>,
            cid: ClusterId,
            report: Vec<u8>,
            pubs: Vec<T::AccountId>,
            sigs: Vec<T::OffchainSignature>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let mut sender_in_pubs = false;
            let mut pubkeys: Vec<T::AccountId> = Vec::new();
            let mut csigs: Vec<T::OffchainSignature> = Vec::new();

            // dsecret 节点列表
            // dsecret node list
            let dpubs = Nodes::<T>::iter_values().collect::<Vec<_>>();
            for j in 0..dpubs.len() {
                for i in 0..pubs.len() {
                    if dpubs[j] == pubs[i] {
                        pubkeys.push(pubs[i].clone());
                        csigs.push(sigs[i].clone());
                    }
                }
                if dpubs[j] == who {
                    sender_in_pubs = true;
                }
            }

            // 必须是节点列表中的节点提交申请
            if !sender_in_pubs {
                return Err(Error::<T>::Call403.into());
            }

            // 签名数量必须大于节点列表的 2/3
            ensure!(
                csigs.len() >= pubs.len() * 2 / 3,
                Error::<T>::OffchainSigNotEnough
            );

            // 验证签名
            for (i, sig) in sigs.iter().enumerate() {
                if !sig.verify(&*report, &pubkeys[i]) {
                    return Err(Error::<T>::OffchainSigError.into());
                }
            }

            // 保存证明
            wetee_worker::ProofOfClusters::<T>::insert(cid.clone(), report.clone());
            wetee_worker::ProofOfClusterTimes::<T>::insert(
                cid.clone(),
                <frame_system::Pallet<T>>::block_number(),
            );

            Self::deposit_event(Event::ClusterProofUpload {
                cid,
                report,
                pubs,
                sigs,
            });

            Ok(().into())
        }

        /// 上传 devloper，report hash 启动应用
        #[pallet::call_index(004)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::work_launch())]
        pub fn work_launch(
            origin: OriginFor<T>,
            work: WorkId,
            report: Option<Vec<u8>>,
            deploy_key: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // dsecret 节点列表
            // dsecret node list
            let mut sender_in_pubs = false;
            let dpubs = Nodes::<T>::iter_values().collect::<Vec<_>>();
            for j in 0..dpubs.len() {
                if dpubs[j] == who {
                    sender_in_pubs = true;
                }
            }

            // 必须是节点列表中的节点提交申请
            if !sender_in_pubs {
                return Err(Error::<T>::Call403.into());
            }

            // 启动应用
            wetee_worker::Pallet::<T>::work_launch(work, report, deploy_key)?;

            Ok(().into())
        }
    }
}
