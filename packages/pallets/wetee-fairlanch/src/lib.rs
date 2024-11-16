#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::{
    pallet_prelude::*,
    sp_runtime::{traits::AccountIdConversion, SaturatedConversion},
    traits::FindAuthor,
    PalletId,
};
use orml_traits::MultiCurrency;
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};

use wetee_primitives::types::WeAssetId;

mod weights;
pub use weights::WeightInfo;

// 1 WTE = 1_000_000_000_000
const WTE: u128 = 1_000_000_000_000;
// 奖励周期 1天 一共 14400 个区块
const EPOCH_BLOCK: u32 = 14400;

// 待领取的 vtoken
#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Wstaking<Balance, BlockNumber> {
    pub amount: Balance,
    pub at: BlockNumber,
}

// 质押中操作函数
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum ReStakeFunc {
    // stake new asset
    Stake,
    // unstake asset
    UnStake,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_system::pallet_prelude::{BlockNumberFor, *};

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[derive(frame_support::DefaultNoBound)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // 0 => node mint reward, initial value 10%
            Economics::<T>::insert(0, 10);
            // 第一个块的奖励为 1 WTE
            // 总量 28,800,000 WTE
            BlockReward::<T>::set((0, WTE.saturated_into::<BalanceOf<T>>()));
        }
    }

    /// pallet config
    /// 组件配置文件
    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_assets::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// Find the author of a block.
        type FindAuthor: FindAuthor<Self::AccountId>;

        /// pallet id
        /// 模块id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// current block reward
    #[pallet::storage]
    #[pallet::getter(fn block_reward)]
    pub type BlockReward<T: Config> = StorageValue<_, (u128, BalanceOf<T>), ValueQuery>;

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
    pub type NextStakingRewards<T: Config> = StorageDoubleMap<
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
    pub type Economics<T: Config> = StorageMap<_, Identity, WeAssetId, u8, OptionQuery>;

    /// vtoken transfer rate
    /// vtoken 转换为 token 的比例
    #[pallet::storage]
    #[pallet::getter(fn vtoken_token )]
    pub type Vtoken2token<T: Config> =
        StorageMap<_, Identity, WeAssetId, (WeAssetId, (u128, u128)), OptionQuery>;

    /// vtoken transfer rate
    /// vtoken 转换为 token 的比例
    // #[pallet::storage]
    // #[pallet::getter(fn token_vtoken )]
    // pub type Token2vtoken<T: Config> =
    //     StorageMap<_, Identity, WeAssetId, (WeAssetId, (u128, u128)), OptionQuery>;

    // /// vtoken transfer rate
    // /// vtoken 转换为 token 的比例
    // #[pallet::storage]
    // #[pallet::getter(fn vtoken_unstaking )]
    // pub type VtokenUnStaking<T: Config> = StorageDoubleMap<
    //     _,
    //     Identity,
    //     T::AccountId,
    //     Identity,
    //     u128,
    //     UnVstaking<BalanceOf<T>, BlockNumberFor<T>>,
    //     OptionQuery,
    // >;

    /// success event
    /// 成功事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// nomal success
        /// 成功的事件
        BlockReward {
            block: BlockNumberFor<T>,
            author: T::AccountId,
            reward: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 重新质押错误
        ReSkakingError,
        /// 质押不存在
        StakingNotExists,
        /// Vtoken not exists
        VtokenNotExists,
        /// 金额数值错误
        Amount403,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // 获取当前是多少天(14400 是一天的区块数)
            let epoch = n.saturated_into::<u128>() / EPOCH_BLOCK as u128;
            let (curr_epoch, mut curr_reward) = BlockReward::<T>::get();

            // 如果进入了新的周期
            if epoch > curr_epoch {
                // a=1，公比 r=0.9995 1460天时（4年），奖励为0.9995^1460=0.99951460≈0.500474，即半年减半
                curr_reward = curr_reward * 9995u32.into() / 10000u32.into();
                BlockReward::<T>::set((epoch, curr_reward));
            }

            // 区块总奖励
            let reward_amount = curr_reward.saturated_into::<u128>();

            // 经济模型
            let economics = Economics::<T>::iter().collect::<Vec<_>>();

            // 节点奖励
            if let Some(index) = economics.iter().position(|(k, _)| *k == 0) {
                let author = Self::author();
                if let Some(block_author) = author {
                    // 节点挖矿奖励
                    let mint_reward_amount =
                        reward_amount / 100 * (economics.get(index).unwrap().1 as u128);
                    let amount: BalanceOf<T> = mint_reward_amount.saturated_into::<BalanceOf<T>>();

                    // 奖励出块奖励
                    let _ = wetee_assets::Pallet::<T>::try_deposit(
                        wetee_assets::NATIVE_ASSET_ID,
                        block_author.clone(),
                        amount,
                    );

                    Self::deposit_event(Event::BlockReward {
                        block: n,
                        author: block_author,
                        reward: amount,
                    });
                }
            }

            // 获取所有 epoch 的总质押
            let epoch_reward_total = EpochRewardTotal::<T>::iter().collect::<Vec<_>>();

            // 奖励质押满一天的用户
            let mut iter = NextStakingRewards::<T>::iter_prefix(n);
            let next_block = n + EPOCH_BLOCK.into();
            while let Some(v) = iter.next() {
                let (user, assets) = v;
                let mut reward: u128 = 0;
                for (asset_id, amount) in assets {
                    // 获取经济模型
                    let asset = economics.iter().find(|(k, _)| *k == asset_id).unwrap();

                    // 当前 epoch 的某种 token 的总奖励
                    let asset_reward =
                        reward_amount / 100 * (asset.1 as u128) * EPOCH_BLOCK as u128;

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
                let _ = NextStakingRewards::<T>::insert(next_block, user.clone(), stakings);
                // 存储用户奖励
                UserReward::<T>::insert(
                    user,
                    (next_block, reward.saturated_into::<BalanceOf<T>>()),
                );
            }

            // 删除已经处理的数据
            let _ = NextStakingRewards::<T>::clear_prefix(n, 0, None);
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::xxxx())]
        pub fn v_staking(
            origin: OriginFor<T>,
            vassert_id: WeAssetId,
            vamount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // 获取 vtoken 对应的 token
            let (assert_id, (pool, vpool)) =
                Vtoken2token::<T>::get(vassert_id).ok_or(Error::<T>::VtokenNotExists)?;
            let amount = vamount * pool.saturated_into::<BalanceOf<T>>()
                / vpool.saturated_into::<BalanceOf<T>>();

            // 转账 asset 到质押池
            let to = Self::staking_pool_account(vassert_id);
            wetee_assets::Pallet::<T>::try_transfer(vassert_id, who.clone(), to, vamount)?;

            // 获取当前的质押数据
            let pre_stakings = Stakings::<T>::iter_key_prefix(who.clone()).collect::<Vec<_>>();
            if pre_stakings.len() > 0 {
                return match Self::try_restaking(who.clone(), assert_id, ReStakeFunc::Stake, amount)
                {
                    Ok(()) => Ok(().into()),
                    Err(_e) => Err(Error::<T>::ReSkakingError.into()),
                };
            }

            // 获取当前区块
            let n = frame_system::Pallet::<T>::block_number();
            let next_block = n + EPOCH_BLOCK.into();
            let _ = Stakings::<T>::insert(&who, assert_id, Wstaking { amount, at: n });

            // 触发下一次奖励
            let mut stakings: Vec<(WeAssetId, BalanceOf<T>)> = Default::default();
            stakings.push((assert_id, amount));
            let _ = NextStakingRewards::<T>::insert(next_block, who.clone(), stakings);

            // 存储用户奖励
            UserReward::<T>::insert(who, (next_block, 0u32.saturated_into::<BalanceOf<T>>()));

            Ok(().into())
        }

        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::xxxx())]
        pub fn v_unstaking(
            origin: OriginFor<T>,
            vassert_id: WeAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // 获取 vtoken 对应的 token
            let (assert_id, (pool, vpool)) =
                Vtoken2token::<T>::get(vassert_id).ok_or(Error::<T>::VtokenNotExists)?;
            let vamount = amount * vpool.saturated_into::<BalanceOf<T>>()
                / pool.saturated_into::<BalanceOf<T>>();

            // 从质押池转帐 vtoken 到用户
            let to = Self::staking_pool_account(vassert_id);
            wetee_assets::Pallet::<T>::try_transfer(vassert_id, to, who.clone(), vamount)?;

            // 获取当前的质押数据
            let pre_staking =
                Stakings::<T>::get(who.clone(), assert_id).ok_or(Error::<T>::StakingNotExists)?;

            // 超过质押量的 unstaking 不允许
            if pre_staking.amount < amount {
                return Err(Error::<T>::Amount403.into());
            }

            // 取现
            match Self::try_restaking(who.clone(), assert_id, ReStakeFunc::UnStake, amount) {
                Ok(()) => Ok(().into()),
                Err(_e) => Err(Error::<T>::ReSkakingError.into()),
            }
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn try_restaking(
            user: T::AccountId,
            assert_id: WeAssetId,
            func: ReStakeFunc,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // 奖励周期 1天 一共 14400 个区块
            let now = frame_system::Pallet::<T>::block_number();
            let next_block = now + EPOCH_BLOCK.into();

            // 获取当前是多少天(14400 是一天的区块数)
            let epoch = now.saturated_into::<u128>() / EPOCH_BLOCK as u128;
            let (curr_epoch, mut curr_reward) = BlockReward::<T>::get();

            // 如果进入了新的周期
            if epoch > curr_epoch {
                // a=1，公比 r=0.9995 1460天时（4年），奖励为0.9995^1460=0.99951460≈0.500474，即半年减半
                curr_reward = curr_reward * 9995u32.into() / 10000u32.into();
                BlockReward::<T>::set((epoch, curr_reward));
            }

            // 区块总奖励
            let reward_amount = curr_reward.saturated_into::<u128>();

            // 获取经济模型
            let economics = Economics::<T>::iter().collect::<Vec<_>>();

            // 获取所有 epoch 的总质押
            let epoch_reward_total = EpochRewardTotal::<T>::iter().collect::<Vec<_>>();

            let user_reward = UserReward::<T>::get(user.clone());
            let assets = NextStakingRewards::<T>::get(user_reward.0, user.clone()).unwrap();

            // 实际质押的时间
            let real_staking_block = now - (user_reward.0 - EPOCH_BLOCK.into());

            let mut reward: u128 = 0;
            for (asset_id, asset_amount) in assets {
                // 获取经济模型
                let asset = economics.iter().find(|(k, _)| *k == asset_id).unwrap();

                // 当前 epoch 的某种 token 的总奖励
                let asset_reward = reward_amount / 100
                    * (asset.1 as u128)
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
            let _ = NextStakingRewards::<T>::remove(user_reward.0, user.clone());

            let asset_staking = Stakings::<T>::get(user.clone(), assert_id).unwrap();
            match func {
                ReStakeFunc::Stake => {
                    Stakings::<T>::insert(
                        user.clone(),
                        assert_id,
                        Wstaking {
                            amount: asset_staking.amount + amount,
                            at: now,
                        },
                    );
                }
                ReStakeFunc::UnStake => {
                    let now_amount = asset_staking.amount - amount;
                    // 如果某个币种的质押量已经为0，则删除该币种的质押数据
                    if now_amount == 0u32.into() {
                        Stakings::<T>::remove(user.clone(), assert_id);
                    } else {
                        Stakings::<T>::insert(
                            user.clone(),
                            assert_id,
                            Wstaking {
                                amount: now_amount,
                                at: now,
                            },
                        );
                    }
                }
            };

            // 存储用户质押数据，用于下一个周期的奖励
            let mut now_staking = Stakings::<T>::iter_prefix(user.clone());
            let mut stakings: Vec<(WeAssetId, BalanceOf<T>)> = Default::default();
            while let Some(v) = now_staking.next() {
                let (asset_id, staking) = v;
                stakings.push((asset_id, staking.amount));
            }

            // 触发下一次奖励
            let _ = NextStakingRewards::<T>::insert(next_block, user.clone(), stakings);

            Ok(())
        }

        /// 获取出块节点
        pub fn author() -> Option<T::AccountId> {
            let digest = <frame_system::Pallet<T>>::digest();
            let pre_runtime_digests = digest.logs.iter().filter_map(|d| d.as_pre_runtime());
            T::FindAuthor::find_author(pre_runtime_digests)
        }

        /// 获取质押池的帐户
        pub fn staking_pool_account(id: WeAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(id)
        }

        /// 质押帐户解析出 asset id
        pub fn staking_pool_from_account(id: T::AccountId) -> WeAssetId {
            let (_, asset_id) = PalletId::try_from_sub_account::<WeAssetId>(&id).unwrap();
            asset_id
        }
    }
}
