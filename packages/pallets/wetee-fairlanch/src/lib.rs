#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::sp_runtime::SaturatedConversion;
use orml_traits::MultiCurrency;
pub use pallet::*;
use wetee_primitives::types::WeAssetId;

mod weights;
pub use weights::WeightInfo;

const UNIT: u128 = 1_000_000_000_000;
const INITIAL_REWARD: u128 = 100 * UNIT;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::{BlockNumberFor, *};

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    /// pallet config
    /// 组件配置文件
    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_authorship::Config + wetee_assets::Config
    {
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

    /// Staking
    #[pallet::storage]
    #[pallet::getter(fn code_signature)]
    pub type Staking<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Identity,
        WeAssetId,
        BlockNumberFor<T>,
        ValueQuery,
    >;

    /// success event
    /// 成功事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// nomal success
        /// 成功的事件
        Success,
    }

    #[pallet::error]
    pub enum Error<T> {
        HaveNoCreatePermission,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            if let Some(block_author) = pallet_authorship::Pallet::<T>::author() {
                let reduction_interval: u128 = 21024000;

                let reward_amount = INITIAL_REWARD
                    / 10
                    / (1 + (n.into() / reduction_interval).saturated_into::<u128>());

                let amount: BalanceOf<T> = reward_amount.saturated_into::<BalanceOf<T>>();

                // 奖励出块
                let _ = wetee_assets::Pallet::<T>::try_deposit(
                    wetee_assets::NATIVE_ASSET_ID,
                    block_author,
                    amount,
                );
            }

            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::xxxx())]
        pub fn xxxx(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }
    }
}
