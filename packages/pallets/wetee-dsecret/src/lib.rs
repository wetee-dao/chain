#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::ConstU32;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    BoundedVec,
};
use sp_std::prelude::Vec;

use wetee_org::{self};
use wetee_primitives::types::ClusterId;

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
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// 代码版本
    #[pallet::storage]
    #[pallet::getter(fn code_mrenclave)]
    pub type CodeMrenclave<T: Config> = StorageValue<_, BoundedVec<u8, ConstU32<64>>, ValueQuery>;

    /// 代码打包签名人
    #[pallet::storage]
    #[pallet::getter(fn code_mrsigner)]
    pub type CodeMrsigner<T: Config> = StorageValue<_, BoundedVec<u8, ConstU32<64>>, ValueQuery>;

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
        CodeUpload {
            mrenclave: Vec<u8>,
            mrsigner: Vec<u8>,
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
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
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
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn upload_code(
            origin: OriginFor<T>,
            mrenclave: BoundedVec<u8, ConstU32<64>>,
            mrsigner: BoundedVec<u8, ConstU32<64>>,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_signed_or_root(origin)?;

            // 更新代码hash
            <CodeMrenclave<T>>::set(mrenclave.clone());
            // 更新代码签名人
            <CodeMrsigner<T>>::set(mrsigner.clone());

            Self::deposit_event(Event::CodeUpload {
                mrenclave: mrenclave.to_vec(),
                mrsigner: mrsigner.to_vec(),
            });

            Ok(().into())
        }

        /// 上传共识节点代码
        /// update consensus node code
        #[pallet::call_index(003)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
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

            ensure!(
                csigs.len() >= pubs.len() * 2 / 3,
                Error::<T>::OffchainSigNotEnough
            );

            for (i, sig) in sigs.iter().enumerate() {
                if !sig.verify(&*report, &pubkeys[i]) {
                    return Err(Error::<T>::OffchainSigError.into());
                }
            }

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
    }
}
