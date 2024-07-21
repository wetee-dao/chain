#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};
use frame_support::traits::UnfilteredDispatchable;
use scale_info::prelude::boxed::Box;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::result;

use wetee_org::{self};
use wetee_primitives::types::DaoAssetId;

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
pub struct Node {
    /// Node dkg public key
    /// 节点公钥
    pub pubkey: BoundedVec<u8, ConstU32<32>>,
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use crate::Event::{CloseSudo, SetSudo, SudoDone};
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

    /// The id of the next node to be created.
    /// 获取下一个 node id
    #[pallet::storage]
    #[pallet::getter(fn next_node_id)]
    pub type NextNodeId<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// dkg 节点列表
    #[pallet::storage]
    #[pallet::getter(fn nodes)]
    pub type Nodes<T: Config> = StorageMap<
        _,
        Identity,
        u64,
        Node,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// root executes external transaction successfully.
        SudoDone {
            sudo: T::AccountId,
            sudo_result: DispatchResult,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        NotSudo,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Execute external transactions as root
        /// 以 root 账户执行函数
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn create_node(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            call: Box<<T as wetee_org::Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {

            Ok(().into())
        }
    }

}
