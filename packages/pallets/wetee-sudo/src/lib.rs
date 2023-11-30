#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
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

/// Info regarding an ongoing referendum.
/// 全民公投的状态
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct SudoTask<BlockNumber, Call> {
    /// 公投id
    pub id: u32,
    /// The hash of the proposal being voted on.
    /// 投票成功后执行内容
    pub proposal: Call,
    /// 投票开始时间
    pub time: BlockNumber,
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

    /// WETEE Root account id.
    /// 组织最高权限 id
    #[pallet::storage]
    #[pallet::getter(fn sudo_account)]
    pub type Account<T: Config> = StorageMap<_, Identity, DaoAssetId, T::AccountId>;

    /// WETEE Root account id.
    /// 组织最高权限 id
    #[pallet::storage]
    #[pallet::getter(fn close_dao)]
    pub type CloseDao<T: Config> = StorageMap<_, Identity, DaoAssetId, bool>;

    /// sudo模块调用历史
    #[pallet::storage]
    #[pallet::getter(fn sudo_tasks)]
    pub type SudoTasks<T: Config> = StorageMap<
        _,
        Identity,
        DaoAssetId,
        BoundedVec<
            SudoTask<BlockNumberFor<T>, <T as wetee_org::Config>::RuntimeCall>,
            ConstU32<100>,
        >,
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
        /// Set root account or reopen sudo.
        SetSudo {
            dao_id: DaoAssetId,
            sudo_account: T::AccountId,
        },
        /// close root account.
        CloseSudo { dao_id: DaoAssetId },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        NotSudo,
        RootNotExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Execute external transactions as root
        /// 以 root 账户执行函数
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn sudo(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            call: Box<<T as wetee_org::Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {
            Self::check_enable(dao_id)?;
            let sudo = Self::check_sudo(dao_id, origin)?;

            // 记录执行历史
            let mut tasks = SudoTasks::<T>::get(dao_id);
            tasks
                .try_insert(
                    tasks.len(),
                    SudoTask {
                        id: tasks.len() as u32,
                        proposal: *call.clone(),
                        time: frame_system::Pallet::<T>::block_number(),
                    },
                )
                .map_err(|_| Error::<T>::RootNotExists)?;
            SudoTasks::<T>::insert(dao_id, tasks);

            let res = call.dispatch_bypass_filter(
                frame_system::RawOrigin::Signed(wetee_org::Pallet::<T>::dao_approve(dao_id, 0))
                    .into(),
            );

            Self::deposit_event(SudoDone {
                sudo,
                sudo_result: res.map(|_| ()).map_err(|e| e.error),
            });
            Ok(().into())
        }

        /// set sudo account
        /// 设置超级用户账户
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_sudo_account())]
        pub fn set_sudo_account(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            sudo_account: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            Self::check_enable(dao_id)?;

            let _sudo = Self::check_sudo(dao_id, origin)?;
            Account::<T>::insert(dao_id, sudo_account.clone());
            Self::deposit_event(SetSudo {
                dao_id,
                sudo_account,
            });
            Ok(().into())
        }

        /// close sudo
        /// 关闭 sudo 功能
        #[pallet::call_index(003)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::close_sudo())]
        pub fn close_sudo(origin: OriginFor<T>, dao_id: DaoAssetId) -> DispatchResultWithPostInfo {
            let _sudo = Self::check_sudo(dao_id, origin)?;
            CloseDao::<T>::insert(dao_id, true);

            Self::deposit_event(CloseSudo { dao_id });
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 测试账户是否为 WETEE root 账户
        fn check_sudo(
            dao_id: DaoAssetId,
            o: OriginFor<T>,
        ) -> result::Result<T::AccountId, DispatchError> {
            let who = ensure_signed(o)?;
            let dao = wetee_org::Pallet::<T>::daos(dao_id).unwrap();
            let sudo_account = Account::<T>::get(dao_id).unwrap_or(dao.creator);

            // 确认是否是sudo用户
            ensure!(who == sudo_account, Error::<T>::NotSudo);
            Ok(who)
        }

        fn check_enable(dao_id: DaoAssetId) -> result::Result<bool, DispatchError> {
            let is_close = CloseDao::<T>::get(dao_id);

            // 确定没有关闭sudo
            ensure!(
                !(is_close.is_some() && is_close.unwrap()),
                Error::<T>::RootNotExists
            );
            Ok(true)
        }
    }
}
