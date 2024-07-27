#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::ConstU32;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;
use sp_runtime::RuntimeDebug;

use wetee_org::{self};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

/// DKG node
/// DKG 节点
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Node<AccountId> {
    /// Node root account
    /// 节点管理账户
    pub root: AccountId,
    /// Node dkg public key
    /// 节点公钥
    pub pubkey: AccountId,
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

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
    pub type Nodes<T: Config> = StorageMap<_, Identity, u64, Node<T::AccountId>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// root executes external transaction successfully.
        SudoDone { sudo: T::AccountId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        NotSudo,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 注册 dkg 节点
        /// register dkg node
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn register_node(
            origin: OriginFor<T>,
            pubkey: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let nid = <NextNodeId<T>>::get();

            // 添加节点
            <Nodes<T>>::insert(nid, Node { root: who, pubkey });

            // 增加 node id
            <NextNodeId<T>>::mutate(|id| *id += 1);
            Ok(().into())
        }

        /// 上传共识节点代码
        /// update consensus node code
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn upload_code(
            _origin: OriginFor<T>,
            mrenclave: BoundedVec<u8, ConstU32<64>>,
            mrsigner: BoundedVec<u8, ConstU32<64>>,
        ) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // 更新代码hash
            <CodeMrenclave<T>>::set(mrenclave);
            // 更新代码签名人
            <CodeMrsigner<T>>::set(mrsigner);
            Ok(().into())
        }
    }
}
