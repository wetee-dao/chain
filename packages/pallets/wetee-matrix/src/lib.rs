#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
use frame_support::traits::IsSubType;
pub use pallet::*;
use parity_scale_codec::MaxEncodedLen;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{traits::BlockNumberProvider, RuntimeDebug};
use sp_std::prelude::*;

mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// Node's status.
/// 节点状态
#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Status {
    #[default]
    /// In use.
    /// 激活
    Active = 0,
    /// Does not work properly.
    /// 未激活
    InActive,
}

/// Node specific information
/// 节点信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct NodeInfo<AccountId, BlockNumber> {
    pub id: u128,
    /// creator of Node
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the Node
    /// Node创建的区块
    pub start_block: BlockNumber,
    /// Node account id.
    /// Node 链上账户ID
    pub account_id: AccountId,
    /// name of the Node.
    /// Node 名字
    pub name: Vec<u8>,
    /// name of the Node.
    /// Node 介绍
    pub desc: Vec<u8>,
    /// Purpose of the Node.
    /// Node 目标宗旨
    pub purpose: Vec<u8>,
    //// meta data
    /// Node 元数据 图片等内容
    pub meta_data: Vec<u8>,
    /// im api
    pub im_api: Vec<u8>,
    /// org color
    pub bg: Vec<u8>,
    /// org logo
    pub logo: Vec<u8>,
    /// 节点大图
    pub img: Vec<u8>,
    /// 节点主页
    pub home_url: Vec<u8>,
    /// State of the Node
    /// Node状态
    pub status: Status,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::{DispatchResultWithPostInfo, GetDispatchInfo},
        pallet_prelude::*,
        traits::UnfilteredDispatchable,
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AccountIdConversion;

    /// pallet config
    /// 组件配置文件
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// All calls supported by Node
        /// 组件所有的函数
        type RuntimeCall: Parameter
            + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>
            + From<Call<Self>>
            + IsSubType<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>;

        /// Each Call has its own id
        /// 函数的调用id
        type CallId: Parameter
            + Copy
            + MaybeSerializeDeserialize
            + TypeInfo
            + MaxEncodedLen
            + Default
            + TryFrom<<Self as pallet::Config>::RuntimeCall>;

        /// pallet id
        /// 模块id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// All Nodes that have been created.
    /// 所有节点
    #[pallet::storage]
    #[pallet::getter(fn nodes)]
    pub type Matrix<T: Config> =
        StorageMap<_, Identity, u128, NodeInfo<T::AccountId, BlockNumberFor<T>>>;

    #[pallet::type_value]
    pub fn DefaultForm5000() -> u128 {
        1
    }

    /// The id of the next node to be created.
    /// 获取下一个节点id
    #[pallet::storage]
    #[pallet::getter(fn next_node_id)]
    pub type NextId<T: Config> = StorageValue<_, u128, ValueQuery, DefaultForm5000>;
    /// success event
    /// 成功事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Node create event
        /// Node创建成功事件
        CreatedNode(T::AccountId, u128),
        /// nomal success
        /// 成功的事件
        Success,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Do not have permission to create.
        /// 没有创建的权限
        HaveNoCreatePermission,
        /// Node already exists
        /// 节点已存在
        NodeExists,
        /// Node does not exist.
        /// 节点不存在
        NodeNotExists,
        /// guild create error
        /// 公会创建失败
        GuildCreateError,
        /// guild does not exist.
        /// 公会不存在
        GuildNotExists,
        /// Node unsupported call
        /// 无效的调用
        InVailCall,
        /// Node unsupported pallet
        /// 无效的Pallet
        InVailPallet,
        /// Wrong origin.
        /// 错误的节点
        BadOrigin,
        /// Not the id of this node.
        /// 节点 id 不正确
        NodeIdNotMatch,
        /// The description of the Node is too long.
        /// 名字太长
        NameTooLong,
        /// The description of the Node is too long.
        /// 节点介绍太长
        DescTooLong,
        /// The description of the Node is too long.
        /// 节点目标太长
        PurposeTooLong,
        /// The description of the Node is too long.
        /// 节点目标太长
        MetaDataTooLong,
        /// Numerical calculation overflow error.
        /// 溢出错误
        Overflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a N o de
        /// 从一个通证池,创建一个节点
        #[pallet::call_index(001)]
        #[pallet::weight(T::WeightInfo::create_dao())]
        pub fn create_node(
            origin: OriginFor<T>,
            name: Vec<u8>,
            desc: Vec<u8>,
            purpose: Vec<u8>,
            meta_data: Vec<u8>,
            im_api: Vec<u8>,
            bg: Vec<u8>,
            logo: Vec<u8>,
            img: Vec<u8>,
            home_url: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure!(name.len() <= 30, Error::<T>::NameTooLong);
            ensure!(desc.len() <= 50, Error::<T>::DescTooLong);
            ensure!(purpose.len() <= 50, Error::<T>::PurposeTooLong);
            ensure!(meta_data.len() <= 1024, Error::<T>::MetaDataTooLong);

            let creator = ensure_signed(origin)?;

            // 创建 Node
            let node_id = NextId::<T>::get();
            let now = frame_system::Pallet::<T>::current_block_number();
            Matrix::<T>::insert(
                node_id.clone(),
                NodeInfo {
                    id: node_id.clone(),
                    name: name.clone(),
                    creator: creator.clone(),
                    account_id: Self::node_account(node_id),
                    start_block: now,
                    desc,
                    purpose,
                    status: Status::Active,
                    meta_data,
                    im_api,
                    bg,
                    logo,
                    img,
                    home_url,
                },
            );

            Self::deposit_event(Event::CreatedNode(creator, node_id));
            Ok(().into())
        }

        /// update node info
        /// 更新节点信息
        #[pallet::call_index(002)]
        #[pallet::weight(T::WeightInfo::update_dao())]
        pub fn update_node(
            origin: OriginFor<T>,
            node_id: u128,
            name: Option<Vec<u8>>,
            desc: Option<Vec<u8>>,
            purpose: Option<Vec<u8>>,
            meta_data: Option<Vec<u8>>,
            im_api: Option<Vec<u8>>,
            bg: Option<Vec<u8>>,
            logo: Option<Vec<u8>>,
            img: Option<Vec<u8>>,
            home_url: Option<Vec<u8>>,
            status: Option<Status>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            // 获取节点
            let mut node = Matrix::<T>::get(node_id).ok_or(Error::<T>::NodeNotExists)?;
            ensure!(node.creator == me, Error::<T>::BadOrigin);

            if let Some(name) = name {
                node.name = name;
            }
            if let Some(desc) = desc {
                node.desc = desc;
            }
            if let Some(purpose) = purpose {
                node.purpose = purpose;
            }
            if let Some(meta_data) = meta_data {
                node.meta_data = meta_data;
            }
            if let Some(im_api) = im_api {
                node.im_api = im_api;
            }
            if let Some(bg) = bg {
                node.bg = bg;
            }
            if let Some(logo) = logo {
                node.logo = logo;
            }
            if let Some(img) = img {
                node.img = img;
            }
            if let Some(home_url) = home_url {
                node.home_url = home_url;
            }
            if let Some(status) = status {
                node.status = status;
            }

            // 更新节点
            Matrix::<T>::insert(node_id, node);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取Node账户
        pub fn node_account(node_id: u128) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(node_id)
        }
    }
}
