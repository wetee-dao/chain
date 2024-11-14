#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::{pallet_prelude::*, sp_runtime::SaturatedConversion};
use orml_traits::MultiCurrency;
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
use wetee_primitives::types::WeAssetId;

mod weights;
pub use weights::WeightInfo;

const UNIT: u128 = 1_000_000_000_000;
const INITIAL_REWARD: u128 = 100 * UNIT;

#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Wstaking<Balance, BlockNumber> {
    pub amount: Balance,
    pub at: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResultWithPostInfo;
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
    #[pallet::getter(fn staking)]
    pub type Stakings<T: Config> = StorageDoubleMap<
        _,
        Identity,
        T::AccountId,
        Identity,
        WeAssetId,
        Wstaking<BalanceOf<T>, BlockNumberFor<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn reward_pointer)]
    pub type StakingTotal<T: Config> = StorageMap<_, Identity, WeAssetId, BalanceOf<T>, ValueQuery>;

    /// next block reward
    /// 下一次奖励的区块高度
    /// 24小时执行一次奖励
    #[pallet::storage]
    #[pallet::getter(fn last_block_reward)]
    pub type NextBlockRewards<T: Config> = StorageDoubleMap<
        _,
        Identity,
        BlockNumberFor<T>,
        Identity,
        T::AccountId,
        Vec<(WeAssetId, BalanceOf<T>)>,
        OptionQuery,
    >;

    // user reward
    // 用户累计奖励
    #[pallet::storage]
    #[pallet::getter(fn user_reward)]
    pub type UserReward<T: Config> =
        StorageMap<_, Identity, T::AccountId, (BlockNumberFor<T>, BalanceOf<T>), ValueQuery>;

    /// epoch_reward_total
    /// 奖励奖励池
    #[pallet::storage]
    #[pallet::getter(fn epoch_reward_total)]
    pub type EpochRewardTotal<T: Config> =
        StorageMap<_, Identity, WeAssetId, BalanceOf<T>, ValueQuery>;

    /// economics
    /// 经济模型
    /// 0 => node mint reward
    /// 1 => tee mint reward
    /// 3 => app mint reward
    /// WeAssetId => staking reward
    #[pallet::storage]
    #[pallet::getter(fn economics)]
    pub type Economics<T: Config> = StorageMap<_, Identity, WeAssetId, (Vec<u8>, u8), ValueQuery>;

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
        /// 重新质押错误
        ReSkakingError,
        /// 质押不存在
        StakingNotExists,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // 奖励周期 1天 一共 14400 个区块
            let epoch_block = 14400;
            // 减半周期 4年(一共 1460 天)
            let reduction_interval: u128 = 1460;
            // 获取当前是多少天(14400 是一天的区块数)
            let epoch = n.saturated_into::<u128>() % epoch_block;
            // 区块总奖励
            let reward_amount = INITIAL_REWARD / (1 + (epoch / reduction_interval));
            // 经济模型
            let economics = Economics::<T>::iter().collect::<Vec<_>>();
            if let Some(index) = economics.iter().position(|(k, _)| *k == 0) {
                // 节点挖矿奖励
                let mint_reward_amount =
                    reward_amount / 100 * (economics.get(index).unwrap().1 .1 as u128);
                if let Some(block_author) = pallet_authorship::Pallet::<T>::author() {
                    let amount: BalanceOf<T> = mint_reward_amount.saturated_into::<BalanceOf<T>>();

                    // 奖励出块奖励
                    let _ = wetee_assets::Pallet::<T>::try_deposit(
                        wetee_assets::NATIVE_ASSET_ID,
                        block_author,
                        amount,
                    );
                }
            }

            // 获取所有 epoch 的总质押
            let epoch_reward_total = EpochRewardTotal::<T>::iter().collect::<Vec<_>>();

            // 奖励质押满一天的用户
            let mut iter = NextBlockRewards::<T>::iter_prefix(n);
            let next_block = n + 14400u32.into();
            while let Some(v) = iter.next() {
                let (user, assets) = v;
                let mut reward: u128 = 0;
                for (asset_id, amount) in assets {
                    // 获取经济模型
                    let asset = economics.iter().find(|(k, _)| *k == asset_id).unwrap();

                    // 当前 epoch 的某种 token 的总奖励
                    let asset_reward = reward_amount / 100 * (asset.1 .1 as u128) * epoch_block;

                    // 获取 epoch 的总质押量
                    let total = epoch_reward_total
                        .iter()
                        .find(|(k, _)| *k == asset_id)
                        .unwrap();

                    // 计算当前帐户的奖励
                    reward += asset_reward / total.1.saturated_into::<u128>()
                        * amount.saturated_into::<u128>();
                }

                // 存储用户质押数据，用于下一个周期的奖励
                let mut now_staking = Stakings::<T>::iter_prefix(user.clone());
                let mut stakings: Vec<(WeAssetId, BalanceOf<T>)> = Default::default();
                while let Some(v) = now_staking.next() {
                    let (asset_id, staking) = v;
                    stakings.push((asset_id, staking.amount));
                }

                // 触发下一次奖励
                let _ = NextBlockRewards::<T>::insert(next_block, user.clone(), stakings);
                // 存储用户奖励
                UserReward::<T>::insert(
                    user,
                    (next_block, reward.saturated_into::<BalanceOf<T>>()),
                );
            }

            // 删除已经处理的数据
            let _ = NextBlockRewards::<T>::clear_prefix(n, 0, None);
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::xxxx())]
        pub fn v_staking(
            origin: OriginFor<T>,
            assert_id: WeAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let pre_stakings = Stakings::<T>::iter_key_prefix(who.clone()).collect::<Vec<_>>();
            if pre_stakings.len() > 0 {
                return match Self::try_restaking(who.clone(), assert_id, amount) {
                    Ok(()) => Ok(().into()),
                    Err(_e) => Err(Error::<T>::ReSkakingError.into()),
                };
            }

            let n = frame_system::Pallet::<T>::block_number();
            let next_block = n + 14400u32.into();
            let _ = Stakings::<T>::insert(&who, assert_id, Wstaking { amount, at: n });

            // 触发下一次奖励
            let mut stakings: Vec<(WeAssetId, BalanceOf<T>)> = Default::default();
            stakings.push((assert_id, amount));
            let _ = NextBlockRewards::<T>::insert(next_block, who.clone(), stakings);

            // 存储用户奖励
            UserReward::<T>::insert(who, (next_block, 0u32.saturated_into::<BalanceOf<T>>()));

            Ok(().into())
        }

        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::xxxx())]
        pub fn v_unstaking(
            origin: OriginFor<T>,
            assert_id: WeAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn try_restaking(
            user: T::AccountId,
            assert_id: WeAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // 奖励周期 1天 一共 14400 个区块
            let epoch_block: u32 = 14400;
            // 减半周期 4年(一共 1460 天)
            let reduction_interval: u128 = 1460;

            let now = frame_system::Pallet::<T>::block_number();
            let next_block = now + epoch_block.into();

            // 获取当前是多少天(14400 是一天的区块数)
            let epoch = now.saturated_into::<u128>() / epoch_block as u128;
            // 区块总奖励
            let reward_amount = INITIAL_REWARD / (1 + (epoch / reduction_interval));

            // 获取经济模型
            let economics = Economics::<T>::iter().collect::<Vec<_>>();

            // 获取所有 epoch 的总质押
            let epoch_reward_total = EpochRewardTotal::<T>::iter().collect::<Vec<_>>();

            let user_reward = UserReward::<T>::get(user.clone());
            let assets = NextBlockRewards::<T>::get(user_reward.0, user.clone()).unwrap();

            // 实际质押的时间
            let real_staking_block = now - (user_reward.0 - epoch_block.into());

            let mut reward: u128 = 0;
            for (asset_id, asset_amount) in assets {
                // 获取经济模型
                let asset = economics.iter().find(|(k, _)| *k == asset_id).unwrap();

                // 当前 epoch 的某种 token 的总奖励
                let asset_reward = reward_amount / 100
                    * (asset.1 .1 as u128)
                    * real_staking_block.saturated_into::<u128>();

                // 获取 epoch 的总质押量
                let total = epoch_reward_total
                    .iter()
                    .find(|(k, _)| *k == asset_id)
                    .unwrap();

                // 计算当前帐户的奖励
                reward += asset_reward / total.1.saturated_into::<u128>()
                    * asset_amount.saturated_into::<u128>();
            }

            // 存储用户奖励
            UserReward::<T>::insert(
                user.clone(),
                (next_block, reward.saturated_into::<BalanceOf<T>>()),
            );
            let _ = NextBlockRewards::<T>::remove(user_reward.0, user.clone());

            // 更新用户质押数据
            Stakings::<T>::try_mutate(
                user.clone(),
                assert_id,
                |staking| -> Result<(), DispatchError> {
                    let v = staking.take().ok_or(Error::<T>::StakingNotExists)?;
                    *staking = Some(Wstaking {
                        amount: v.amount + amount,
                        at: now,
                    });
                    Ok(())
                },
            )?;

            // 存储用户质押数据，用于下一个周期的奖励
            let mut now_staking = Stakings::<T>::iter_prefix(user.clone());
            let mut stakings: Vec<(WeAssetId, BalanceOf<T>)> = Default::default();
            while let Some(v) = now_staking.next() {
                let (asset_id, staking) = v;
                stakings.push((asset_id, staking.amount));
            }

            // 触发下一次奖励
            let _ = NextBlockRewards::<T>::insert(next_block, user.clone(), stakings);

            Ok(())
        }
    }
}
