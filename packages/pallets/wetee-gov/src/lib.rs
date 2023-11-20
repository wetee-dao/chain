// Copyright 2022 daos-org.
// This file is part of DAOS

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// forked from https://github.com/paritytech/substrate/tree/master/frame/democracy
// Only a small portion of the democracy module's code is used here, and the functionality varies considerably.
// For better compatibility, it should be simple and easy to understand.
// You can set a minimum vote value for each call.
// If the "yes" vote is greater than the "no" vote and the minimum number of votes is met(That is, the probability of voting meets the requirement),
// the call can dispatch.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{DispatchResult as DResult, UnfilteredDispatchable},
    traits::OriginTrait,
    RuntimeDebug,
};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{BlockNumberProvider, Hash, Saturating},
    DispatchError, Percent,
};
use sp_std::boxed::Box;
use sp_std::result;
use traits::*;

use orml_traits::MultiCurrency;

use wetee_org;
use wetee_primitives::traits::{GovIsJoin, PalletGet};
use wetee_primitives::types::DaoAssetId;

use weights::WeightInfo;

pub use pallet::*;

pub type PropIndex = u32;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod traits;
pub mod weights;

pub type PalletsOriginOf<T> =
    <<T as frame_system::Config>::RuntimeOrigin as OriginTrait>::PalletsOrigin;

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum MemberData {
    /// 全局.
    GLOBAL,
    /// 公会.
    GUILD(u64),
    /// 项目.
    PROJECT(u64),
}

/// Voting Statistics.
/// 投票数据统计
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Tally<Balance> {
    /// The number of yes votes
    /// 同意的数量
    pub yes: Balance,
    /// The number of no votes
    /// 不同意的数量
    pub no: Balance,
}

/// vote yes or no
/// 投票
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Opinion {
    /// Agree.
    YES = 0,
    /// Reject.
    NO,
}

/// Information about votes.
/// 投票信息
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct VoteInfo<DaoId, Pledge, BlockNumber, VoteWeight, Opinion, PropIndex> {
    /// The id of the Dao where the vote is located.
    /// 投票所在组织
    pub dao_id: DaoId,
    /// The specific thing that the vote pledged.
    /// 抵押
    pub pledge: Pledge,
    /// Object or agree.
    /// 是否同意
    pub opinion: Opinion,
    /// voting weight.
    /// 投票权重
    pub vote_weight: VoteWeight,
    /// Block height that can be unlocked.
    /// 投票解锁阶段
    pub unlock_block: BlockNumber,
    /// The prop id corresponding to the vote.
    /// 投票的全民公投
    pub prop_index: PropIndex,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct PreProp<BlockNumber, Call, Hash, AccountId> {
    pub id: PropIndex,
    pub hash: Hash,
    pub call: Call,
    pub member_data: MemberData,
    pub creater: AccountId,
    pub period_index: u32,
    pub start: BlockNumber,
}

/// Info regarding an ongoing prop.
/// 全民公投的状态
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Prop<BlockNumber, Call, Balance> {
    /// 公投id
    pub id: PropIndex,
    /// The hash of the proposal being voted on.
    /// 投票成功后执行内容
    pub proposal: Call,
    /// 投票开始时间
    pub start: BlockNumber,
    /// 提案渠道
    pub period_index: u32,
    /// The current tally of votes in this prop.
    /// 投票统计
    pub tally: Tally<Balance>,
    /// 投票范围
    pub member_data: MemberData,
    /// 是否已经结束
    pub status: PropStatus,
}

/// 投票轨道
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Period<BlockNumber, Balance> {
    /// 投票轨道名
    pub name: Vec<u8>,
    /// pallet_index
    /// 模块编号
    pub pallet_index: u16,
    /// 导入期或准备期
    /// 在这个阶段，提案人或其他人需要支付一笔 “决定押金”。
    pub prepare_period: BlockNumber,
    /// 最长决策期
    /// 就像上面说到的，如果在 28 天内的某个时间点，
    /// Approval 和 Support 均达到了通过阈值时，
    /// 公投就进入下一个 Confirming 阶段
    pub max_deciding: BlockNumber,
    /// Confirming 阶段的长度视轨道参数决定
    /// 在 Confirminig 阶段，如果 Approval 和 Support
    /// 两个比率能保持高于通过阈值 “安全” 地渡过这个时期（例如 1 天），
    /// 那么该公投就算正式通过了。
    pub confirm_period: BlockNumber,
    /// 一项公投在投票通过后，再安全地度过了执行期，与该公投相关的代码就会自动在链上执行。
    pub decision_period: BlockNumber,
    /// 提案结束后多久能解锁
    pub min_enactment_period: BlockNumber,
    /// 决定押金
    pub decision_deposit: Balance,
    /// 投票成功百分比
    pub min_approval: u8,
    /// 投票率
    pub min_support: u8,
    /// 最大能执行的金额
    /// 如果金额范围不合理，就无法成功执行提案
    pub max_balance: Balance,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum PropStatus {
    Ongoing = 0,
    Approved,
    Rejected,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    // use frame_system::pallet_prelude::*;

    pub(crate) type BalanceOf<T> = <<T as wetee_assets::Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_assets::Config + wetee_org::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// What to stake when voting in a prop.
        type Pledge: Clone
            + Default
            + Copy
            + Parameter
            + Member
            + PledgeTrait<
                BalanceOf<Self>,
                Self::AccountId,
                DaoAssetId,
                BlockNumberFor<Self>,
                DispatchError,
            >;

        /// 判断是否是加入工会和项目的投票
        /// Determine whether it is a vote to join the guild and project.
        type GovFunc: GovIsJoin<<Self as wetee_org::Config>::RuntimeCall>
            + PalletGet<<Self as wetee_org::Config>::RuntimeCall>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Number of public proposals so for.
    #[pallet::storage]
    #[pallet::getter(fn pre_prop_count)]
    pub type PrePropCount<T: Config> = StorageMap<_, Identity, DaoAssetId, PropIndex, ValueQuery>;

    #[pallet::type_value]
    pub fn MaxPrePropsOnEmpty() -> PropIndex {
        100u32
    }

    /// Maximum number of public proposals at one time.
    #[pallet::storage]
    #[pallet::getter(fn max_pre_props)]
    pub type MaxPreProps<T: Config> =
        StorageMap<_, Identity, DaoAssetId, u32, ValueQuery, MaxPrePropsOnEmpty>;

    /// 投票轨道
    #[pallet::storage]
    #[pallet::getter(fn periods)]
    pub type Periods<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        BoundedVec<Period<BlockNumberFor<T>, BalanceOf<T>>, ConstU32<100>>,
        ValueQuery,
    >;

    /// 投票轨道
    #[pallet::storage]
    #[pallet::getter(fn defalut_period)]
    pub type DefaultPeriods<T: Config> = StorageValue<
        _,
        BoundedVec<Period<BlockNumberFor<T>, BalanceOf<T>>, ConstU32<100>>,
        ValueQuery,
    >;

    /// The public proposals.
    /// Unsorted.
    /// The second item is the proposal's hash.
    #[pallet::storage]
    #[pallet::getter(fn pre_props)]
    pub type PreProps<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        Vec<
            PreProp<
                BlockNumberFor<T>,
                <T as wetee_org::Config>::RuntimeCall,
                T::Hash,
                T::AccountId,
            >,
        >,
        ValueQuery,
    >;

    /// 提案
    /// Those who have locked a deposit.
    /// TWOX-NOTE: Safe, as increasing integer keys are safe.
    #[pallet::storage]
    #[pallet::getter(fn deposit_of)]
    pub type DepositOf<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DaoAssetId,
        Identity,
        PropIndex,
        (Vec<T::AccountId>, BalanceOf<T>),
    >;

    /// 全民投票
    /// Prop specific information.
    #[pallet::storage]
    #[pallet::getter(fn prop_info)]
    pub type Props<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DaoAssetId,
        Identity,
        PropIndex,
        Prop<BlockNumberFor<T>, <T as wetee_org::Config>::RuntimeCall, BalanceOf<T>>,
    >;

    /// Amount of proposal locked.
    #[pallet::storage]
    #[pallet::getter(fn reserve_of)]
    pub type ReserveOf<T: Config> =
        StorageMap<_, Identity, T::AccountId, Vec<(BalanceOf<T>, BlockNumberFor<T>)>, ValueQuery>;

    /// Number of props so far.
    #[pallet::storage]
    #[pallet::getter(fn prop_count)]
    pub type PropCount<T: Config> = StorageMap<_, Identity, DaoAssetId, PropIndex, ValueQuery>;

    /// WETEE 投票模式默认 0，1 TOKEN 1 票
    #[pallet::storage]
    #[pallet::getter(fn vote_model)]
    pub type VoteModel<T: Config> = StorageMap<_, Identity, DaoAssetId, u8, ValueQuery>;

    /// Everyone's voting information.
    #[pallet::storage]
    #[pallet::getter(fn votes_of)]
    pub type VotesOf<T: Config> = StorageMap<
        _,
        Identity,
        T::AccountId,
        Vec<VoteInfo<DaoAssetId, T::Pledge, BlockNumberFor<T>, BalanceOf<T>, Opinion, PropIndex>>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// initiate a proposal.
        Proposed(DaoAssetId, T::Hash),
        /// Others support initiating proposals.
        Recreate(DaoAssetId, BalanceOf<T>),
        /// Open a prop.
        StartTable(DaoAssetId, PropIndex),
        /// Vote for the prop.
        Vote(DaoAssetId, PropIndex, T::Pledge),
        /// Cancel a vote on a prop.
        CancelVote(DaoAssetId, PropIndex),
        /// Vote and execute the transaction corresponding to the proposa.
        EnactProposal {
            dao_id: DaoAssetId,
            index: PropIndex,
            result: DResult,
        },
        /// Unlock
        Unlock(T::AccountId, DaoAssetId, T::Pledge),
        /// Unlock
        Unreserved(T::AccountId, BalanceOf<T>),
        /// Set Origin for each Call.
        SetMinVoteWeight(DaoAssetId, T::CallId, BalanceOf<T>),
        /// Set the maximum number of proposals at the same time.
        SetMaxPreProps {
            dao_id: DaoAssetId,
            max: u32,
        },
        VoteModelUpdate {
            dao_id: DaoAssetId,
            model: u8,
        },
        PeriodUpdate {
            dao_id: DaoAssetId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Integer computation overflow.
        Overflow,
        /// Insufficient amount of deposit.
        DepositTooLow,
        /// Maximum number of proposals reached.
        TooManyProposals,
        /// Proposal does not exist.
        ProposalMissing,
        /// There are no proposals in progress.
        NoneWaiting,
        /// Prop does not exist.
        PropNotExists,
        /// Prop ends.
        PropFinished,
        /// Prop voting is underway.
        VoteNotEnd,
        /// Delayed execution time.
        InDelayTime,
        /// Prop voting has ended.
        VoteEnd,
        /// 投票
        VoteRedundancy,
        /// Voting closed but proposal rejected.
        VoteEndButNotPass,
        /// It's not time to open a new prop.
        NotTableTime,
        /// Bad origin.
        VoteWeightTooLow,
        /// 是的
        PledgeNotEnough,
        /// 没有权限
        Gov403,
        /// 没有找到
        Gov404,
        /// 错误的DAO组织
        BadDaoOrigin,
    }

    #[derive(frame_support::DefaultNoBound)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _config: sp_std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            let mut ps: Vec<Period<BlockNumberFor<T>, BalanceOf<T>>> = Vec::new();
            ps.push(Period {
                name: "root".into(),
                pallet_index: 4,
                decision_deposit: 1u32.into(),
                prepare_period: 10u32.into(),
                max_deciding: 100u32.into(),
                confirm_period: 10u32.into(),
                decision_period: 10u32.into(),
                min_enactment_period: 10u32.into(),
                min_approval: 1,
                min_support: 1,
                max_balance: 0u32.into(),
            });
            ps.push(Period {
                name: "gov".into(),
                pallet_index: 4,
                decision_deposit: 1u32.into(),
                prepare_period: 10u32.into(),
                max_deciding: 100u32.into(),
                confirm_period: 10u32.into(),
                decision_period: 10u32.into(),
                min_enactment_period: 10u32.into(),
                min_approval: 1,
                min_support: 1,
                max_balance: 0u32.into(),
            });
            ps.push(Period {
                name: "treasury".into(),
                pallet_index: 4,
                decision_deposit: 1u32.into(),
                prepare_period: 10u32.into(),
                max_deciding: 100u32.into(),
                confirm_period: 10u32.into(),
                decision_period: 10u32.into(),
                min_enactment_period: 10u32.into(),
                min_approval: 1,
                min_support: 1,
                max_balance: 1000u32.into(),
            });
            let bps = BoundedVec::try_from(ps).unwrap();
            DefaultPeriods::<T>::set(bps)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create a proposal
        /// 创建一个提案
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::submit_proposal())]
        pub fn submit_proposal(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            member_data: MemberData,
            proposal: Box<<T as wetee_org::Config>::RuntimeCall>,
            period_index: u32,
        ) -> DispatchResultWithPostInfo {
            let ps = Periods::<T>::get(dao_id);
            let uperiod_index: usize = period_index.try_into().unwrap();
            let period = ps.get(uperiod_index).unwrap();

            // 判断提案通道是否存在
            ensure!(uperiod_index < ps.len(), wetee_org::Error::<T>::InVailCall);

            // 判断提案通道是否和提案匹配
            let pallet_id = T::GovFunc::get_pallet_id(*proposal.clone());
            ensure!(
                pallet_id == period.pallet_index,
                wetee_org::Error::<T>::InVailCall
            );

            // 获取提案通道
            // let period = ps.get(uperiod_index).unwrap();
            let who = ensure_signed(origin)?;

            // 判断函数是否可以提案
            if T::GovFunc::is_join(*proposal.clone()) {
                Self::check_auth_for_proposal(dao_id, who.clone())?;
            } else {
                // 确认用户属于可提案的用户范围
                Self::check_auth_for_vote(dao_id, member_data.clone(), who.clone())?;
            }

            // 确认当前函数为 sudo/gov 可调用函数
            let call_id: T::CallId =
                TryFrom::<<T as wetee_org::Config>::RuntimeCall>::try_from(*proposal.clone())
                    .unwrap_or_default();

            // let ucall_id: u32 = call_id.into();
            // (call_id - call_id % 100) / 100;
            // let ucall_id: u32 = <<T as wetee_org::Config>::CallId as frame_support::traits::Get>::get(call_id);

            // 确认提案为当前资产支持的 调用
            #[cfg(not(feature = "runtime-benchmarks"))]
            ensure!(
                call_id != T::CallId::default(),
                wetee_org::Error::<T>::InVailCall
            );

            let proposal_hash = T::Hashing::hash_of(&proposal);
            let proposal_index = Self::pre_prop_count(dao_id);
            let real_prop_count = PreProps::<T>::decode_len(dao_id).unwrap_or(0) as u32;
            let max_proposals = MaxPreProps::<T>::get(dao_id);
            let now = Self::now();

            // 确定提案数是否超过了最大提案
            ensure!(
                real_prop_count < max_proposals,
                Error::<T>::TooManyProposals
            );

            // 更新提案数量
            PrePropCount::<T>::insert(dao_id, proposal_index + 1);

            // 添加提案
            <PreProps<T>>::append(
                dao_id,
                PreProp {
                    id: proposal_index,
                    hash: proposal_hash,
                    call: *proposal,
                    member_data,
                    creater: who,
                    period_index,
                    start: now,
                },
            );

            Self::deposit_event(Event::<T>::Proposed(dao_id, proposal_hash));
            Ok(().into())
        }

        /// Open a prop.
        /// 开始全民公投
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn deposit_proposal(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            propose_id: u32,
            #[pallet::compact] deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // 获取提案列表
            let mut pre_props = Self::pre_props(dao_id);
            let propose_index = pre_props.iter().position(|p| p.id == propose_id).unwrap();

            ensure!(pre_props.len() > propose_index, Error::<T>::Gov404);

            // 获取提案
            let p = pre_props.swap_remove(propose_index);
            let prop_index = Self::try_add_prop(
                dao_id,
                who,
                p.start,
                p.period_index,
                p.member_data,
                p.call,
                deposit,
            );

            Self::deposit_event(Event::<T>::StartTable(dao_id, prop_index.unwrap()));

            Ok(().into())
        }

        /// Vote for the prop
        /// 为全民公投投票
        #[pallet::call_index(004)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn vote_for_prop(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            prop_index: PropIndex,
            pledge: T::Pledge,
            opinion: Opinion,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = Self::now();
            let mut vote_weight = BalanceOf::<T>::from(0u32);

            // 检查用户是否已经参与了投票，只能投票一次
            let votes = VotesOf::<T>::get(&who);
            votes
                .binary_search_by(|v| v.prop_index.cmp(&prop_index))
                .err()
                .ok_or(Error::<T>::VoteRedundancy)?;

            Props::<T>::try_mutate_exists(
                dao_id,
                prop_index,
                |h| -> result::Result<(), DispatchError> {
                    let mut info = h.take().ok_or(Error::<T>::PropNotExists)?;
                    if info.status == PropStatus::Ongoing {
                        // 确认用户属于可投票的用户范围
                        Self::check_auth_for_vote(dao_id, info.member_data.clone(), who.clone())?;
                        let period = Self::get_period(dao_id, info.period_index)?;
                        if info.start + period.max_deciding > now {
                            let vote_model = <VoteModel<T>>::try_get(dao_id).unwrap_or_default();
                            let vote_result = pledge.try_vote(&who, &dao_id, vote_model)?;
                            vote_weight = vote_result.0;

                            let duration = vote_result.1;
                            match opinion {
                                Opinion::NO => {
                                    info.tally.no += vote_weight;
                                }
                                Opinion::YES => {
                                    info.tally.yes += vote_weight;
                                }
                            };

                            VotesOf::<T>::append(
                                &who,
                                VoteInfo {
                                    dao_id,
                                    pledge,
                                    opinion,
                                    vote_weight,
                                    unlock_block: now + duration,
                                    prop_index,
                                },
                            );
                        } else {
                            return Err(Error::<T>::VoteEnd)?;
                        }
                    } else {
                        return Err(Error::<T>::PropFinished)?;
                    }
                    *h = Some(info);
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::<T>::Vote(dao_id, prop_index, pledge));
            Ok(().into())
        }

        /// Cancel a vote on a prop
        /// 取消一个投票
        #[pallet::call_index(005)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn cancel_vote(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            index: PropIndex,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Props::<T>::try_mutate_exists(
                dao_id,
                index,
                |h| -> result::Result<(), DispatchError> {
                    let mut info = h.take().ok_or(Error::<T>::PropNotExists)?;
                    let now = Self::now();
                    let period = Self::get_period(dao_id, info.period_index)?;

                    if info.status == PropStatus::Ongoing {
                        if info.start + period.max_deciding + period.confirm_period > now {
                            let mut votes = VotesOf::<T>::get(&who);
                            votes.retain(|h| {
                                if h.prop_index == index
                                    && h.pledge.vote_end_do(&who, &dao_id).is_ok()
                                {
                                    match h.opinion {
                                        Opinion::NO => {
                                            info.tally.no =
                                                info.tally.no.saturating_sub(h.vote_weight);
                                        }
                                        _ => {
                                            info.tally.yes =
                                                info.tally.yes.saturating_sub(h.vote_weight);
                                        }
                                    };
                                    false
                                } else {
                                    true
                                }
                            });
                            VotesOf::<T>::insert(&who, votes);
                        } else {
                            return Err(Error::<T>::VoteEnd)?;
                        }
                    } else {
                        return Err(Error::<T>::PropFinished)?;
                    }
                    *h = Some(info);
                    Ok(())
                },
            )?;
            Self::deposit_event(Event::<T>::CancelVote(dao_id, index));

            Ok(().into())
        }

        /// Vote and execute the transaction corresponding to the proposa
        /// 执行一个投票通过提案
        #[pallet::call_index(006)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn run_proposal(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            index: PropIndex,
        ) -> DispatchResultWithPostInfo {
            ensure_signed(origin)?;
            let now = Self::now();
            let mut approved = false;

            let state = Props::<T>::get(dao_id, index).ok_or(Error::<T>::PropNotExists)?;

            // 如果已经结束了
            if state.status != PropStatus::Ongoing {
                return Err(Error::<T>::PropFinished)?;
            }

            let period = Self::get_period(dao_id, state.period_index)?;
            if state.start + period.max_deciding + period.confirm_period > now {
                // 如果投票还没结束
                return Err(Error::<T>::VoteNotEnd)?;
            } else if state.start
                + period.max_deciding
                + period.confirm_period
                + period.decision_period
                > now
            {
                // 如果投票结束了，但是还在等待执行期内
                return Err(Error::<T>::InDelayTime)?;
            }

            // 判断获取的投票数是否足够
            if state.tally.yes.saturating_add(state.tally.no)
                >= Percent::from_percent(period.min_support)
                    * wetee_assets::Pallet::<T>::total_issuance(dao_id)
            {
                if state.tally.yes >= state.tally.no {
                    approved = true;
                    let res = state.proposal.dispatch_bypass_filter(
                        frame_system::RawOrigin::Signed(wetee_org::Pallet::<T>::dao_approve(
                            dao_id,
                            state.period_index,
                        ))
                        .into(),
                    );
                    Self::deposit_event(Event::EnactProposal {
                        dao_id,
                        index,
                        result: res.map(|_| ()).map_err(|e| e.error),
                    });
                } else {
                    let _res = state.proposal.dispatch_bypass_filter(
                        frame_system::RawOrigin::Signed(wetee_org::Pallet::<T>::dao_reject(
                            dao_id,
                            state.period_index,
                        ))
                        .into(),
                    );
                }

                // 更新状态
                Props::<T>::try_mutate(dao_id, index, |h| -> result::Result<(), DispatchError> {
                    let mut info = h.take().ok_or(Error::<T>::PropNotExists)?;
                    if approved {
                        info.status = PropStatus::Approved;
                    } else {
                        info.status = PropStatus::Rejected;
                    }
                    *h = Some(info);
                    Ok(())
                })?;
            } else {
                return Err(Error::<T>::VoteWeightTooLow)?;
            }

            Ok(().into())
        }

        /// Unlock
        #[pallet::call_index(007)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn unlock(origin: OriginFor<T>, dao_id: DaoAssetId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = Self::now();

            // 解锁质押
            {
                let mut total = BalanceOf::<T>::from(0u32);
                let mut reserve_info = ReserveOf::<T>::get(&who);
                reserve_info.retain(|h| {
                    if h.1 > now {
                        true
                    } else {
                        wetee_assets::Pallet::<T>::unreserve(dao_id, who.clone(), h.0).unwrap();
                        total += h.0;
                        false
                    }
                });
                ReserveOf::<T>::insert(&who, reserve_info);
                Self::deposit_event(Event::<T>::Unreserved(who.clone(), total));
            }

            // 解锁投票
            {
                let mut votes = VotesOf::<T>::get(&who);
                votes.retain(|h| {
                    // 未完成的投票不能直接解锁
                    // 未到解锁期的投票不能解锁
                    // 解锁失败的投票不能解锁
                    let prop = Props::<T>::get(h.dao_id, h.prop_index).unwrap();
                    if prop.status == PropStatus::Ongoing
                        || h.unlock_block > now
                        || h.pledge.vote_end_do(&who, &h.dao_id).is_err()
                    {
                        true
                    } else {
                        Self::deposit_event(Event::<T>::Unlock(who.clone(), h.dao_id, h.pledge));
                        false
                    }
                });
                VotesOf::<T>::insert(&who, votes);
            }

            Ok(().into())
        }

        /// Set the maximum number of proposals at the same time
        #[pallet::call_index(009)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn set_max_pre_props(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            max: u32,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(me)?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            MaxPreProps::<T>::insert(dao_id, max);
            Self::deposit_event(Event::<T>::SetMaxPreProps { dao_id, max });

            Ok(().into())
        }

        #[pallet::call_index(015)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn update_vote_model(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            model: u8,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(me.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            <VoteModel<T>>::insert(dao_id, model);
            Self::deposit_event(Event::<T>::VoteModelUpdate { dao_id, model });

            Ok(().into())
        }

        #[pallet::call_index(016)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 2)  + Weight::from_all(40_000))]
        pub fn set_periods(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            periods: Vec<Period<BlockNumberFor<T>, BalanceOf<T>>>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(me.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            let bperiods = BoundedVec::try_from(periods).unwrap();
            Periods::<T>::set(dao_id, bperiods);

            Self::deposit_event(Event::<T>::PeriodUpdate { dao_id });

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取当前投票的作用范围
        pub fn try_get_members(
            dao_id: DaoAssetId,
            member_data: MemberData,
        ) -> result::Result<BoundedVec<T::AccountId, T::MaxMembers>, DispatchError> {
            let ms: BoundedVec<T::AccountId, T::MaxMembers> = match member_data {
                MemberData::GLOBAL => <wetee_org::Members<T>>::get(dao_id),
                MemberData::GUILD(v) => <wetee_org::GuildMembers<T>>::get(dao_id, v),
                MemberData::PROJECT(v) => <wetee_org::ProjectMembers<T>>::get(dao_id, v),
            };
            Ok(ms)
        }

        /// 获取用户是否有 提案 的权利
        pub fn check_auth_for_proposal(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let ms = <wetee_org::Members<T>>::get(dao_id);
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Gov403)?;

            Ok(index)
        }

        /// 获取用户是否有 提案//投票 的权利
        pub fn check_auth_for_vote(
            dao_id: DaoAssetId,
            member_data: MemberData,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let ms: BoundedVec<T::AccountId, T::MaxMembers> = match member_data {
                MemberData::GLOBAL => <wetee_org::Members<T>>::get(dao_id),
                MemberData::GUILD(v) => <wetee_org::GuildMembers<T>>::get(dao_id, v),
                MemberData::PROJECT(v) => <wetee_org::ProjectMembers<T>>::get(dao_id, v),
            };
            let index = ms.binary_search(&who).ok().ok_or(Error::<T>::Gov403)?;

            Ok(index)
        }

        /// 添加提案
        pub fn try_add_prop(
            dao_id: DaoAssetId,
            who: T::AccountId,
            start: BlockNumberFor<T>,
            period_index: u32,
            member_data: MemberData,
            proposal: <T as wetee_org::Config>::RuntimeCall,
            deposit: BalanceOf<T>,
        ) -> result::Result<PropIndex, DispatchError> {
            let now = Self::now();

            let period = Self::get_period(dao_id, period_index)?;

            // 判断决定押金是否足够
            ensure!(
                deposit >= period.decision_deposit,
                Error::<T>::DepositTooLow
            );

            // 判断是否可以用
            ensure!(
                start + period.prepare_period <= now,
                Error::<T>::NotTableTime
            );

            // 添加提案抵押
            wetee_assets::Pallet::<T>::reserve(dao_id, who.clone(), deposit)?;

            // <DepositOf<T>>::insert(dao_id, propose_id, (&[&who][..], deposit));
            // <PreProps<T>>::insert(dao_id, pre_props);

            // 确认用户属于可提案的用户范围
            Self::check_auth_for_vote(dao_id, member_data.clone(), who.clone())?;

            // 获取抵押
            // let mut prop_index: Option<PropIndex> = None;

            // if <DepositOf<T>>::take(dao_id, prop_index).is_some() {
            let prop_index = Some(Self::inject_prop(
                dao_id,
                proposal,
                now,
                period_index,
                member_data,
            ));
            // }

            // if prop_index.is_none() {
            //     Err(Error::<T>::NoneWaiting)?
            // }

            // 抵押
            <DepositOf<T>>::insert(dao_id, prop_index.unwrap(), (&[&who][..], deposit));

            Ok(prop_index.unwrap())
        }

        /// 获取投票轨道
        pub fn get_period(
            dao_id: DaoAssetId,
            period_index: u32,
        ) -> result::Result<Period<BlockNumberFor<T>, BalanceOf<T>>, DispatchError> {
            let mut ps = Periods::<T>::get(dao_id);
            // 未设置组织内轨道，采用默认投票轨道
            if ps.len() == 0 {
                ps = DefaultPeriods::<T>::get()
            }

            let uperiod_index: usize = period_index.try_into().unwrap();

            // 判断提案通道是否存在
            ensure!(uperiod_index < ps.len(), wetee_org::Error::<T>::InVailCall);

            Ok(ps.get(uperiod_index).unwrap().clone())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn backing_for(dao_id: DaoAssetId, proposal: PropIndex) -> Option<BalanceOf<T>> {
        Self::deposit_of(dao_id, proposal).map(|(l, d)| d.saturating_mul((l.len() as u32).into()))
    }

    fn inject_prop(
        dao_id: DaoAssetId,
        proposal: <T as wetee_org::Config>::RuntimeCall,
        now: BlockNumberFor<T>,
        period_index: u32,
        member_data: MemberData,
    ) -> PropIndex {
        let ref_index = Self::prop_count(dao_id);
        PropCount::<T>::insert(dao_id, ref_index + 1);
        let item = Prop {
            id: ref_index,
            start: now,
            proposal,
            period_index,
            tally: Default::default(),
            member_data,
            status: PropStatus::Ongoing,
        };

        <Props<T>>::insert(dao_id, ref_index, item);
        ref_index
    }

    fn now() -> BlockNumberFor<T> {
        frame_system::Pallet::<T>::current_block_number()
    }
}
