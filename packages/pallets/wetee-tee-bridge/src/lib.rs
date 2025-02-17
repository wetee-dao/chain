#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::fungible::Inspect;
use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

use sp_std::result;

use wetee_primitives::{
    handle_dispatch_error,
    traits::WorkExt,
    types::{ApiMeta, InkArg, WorkId, NATIVE_ASSET_ID},
};

use wetee_dao::{self};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
use weights::WeightInfo;

pub use pallet::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum TEECallType {
    Ink,
    Evm,
    Pallet,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TEECall<AccountId> {
    // tee call id
    pub id: u128,
    // tee call from chain index
    pub chain_id: Option<u64>,
    // tee call from contract
    pub org_contract: AccountId,
    // tee call from contract
    pub org_caller: AccountId,
    // tee call type
    pub call_type: TEECallType,
    // tee call to
    pub work_id: WorkId,
    // tee call method index
    pub method: u16,
    // tee call args
    pub args: Vec<u8>,
    // callback method index
    pub callback_method: [u8; 4],
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use wetee_primitives::types::ClusterId;

    type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Inspect<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + wetee_dao::Config
        + pallet_contracts::Config
        + wetee_assets::Config
        + wetee_worker::Config
    {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// work ext function
        /// 工作扩展函数
        type WorkExt: WorkExt<Self::AccountId, BalanceOf<Self>>;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::type_value]
    pub fn DefaultForm1() -> u128 {
        1
    }

    #[pallet::storage]
    #[pallet::getter(fn next_id)]
    pub type NextId<T: Config> = StorageValue<_, u128, ValueQuery, DefaultForm1>;

    #[pallet::storage]
    #[pallet::getter(fn tee_calls)]
    pub type TEECalls<T: Config> = StorageDoubleMap<
        _,
        Identity,
        ClusterId,
        Identity,
        u128,
        TEECall<T::AccountId>,
        OptionQuery,
    >;

    /// App
    /// 应用
    #[pallet::storage]
    #[pallet::getter(fn api_metas)]
    pub type ApiMetas<T: Config> = StorageMap<_, Identity, WorkId, ApiMeta>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// tee call started
        TEECallStarted {
            cluster_id: ClusterId,
            call_id: u128,
        },
        /// tee call successed
        TEECallSuccessed {
            cluster_id: ClusterId,
            call_id: u128,
        },
        /// tee call started
        TEECallFailed {
            cluster_id: ClusterId,
            call_id: u128,
            error: Vec<u8>,
        },
        /// tee call started
        TEECallBackFailed {
            cluster_id: ClusterId,
            call_id: u128,
            error: Vec<u8>,
        },
        /// Ink call successed
        InkCallSuccessed {
            work_id: WorkId,
            contract: T::AccountId,
            method: [u8; 4],
            args: Vec<InkArg>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Not a sudo account, nor a dao account.
        Call404,
        /// Call error.
        CallBackError,
        /// Call ink contract error.
        CallInkError,
        // Not allowed.
        NotAllowed403,
        // Worker status error.
        WorkStatusError,
        /// Worker not found.
        WorkNotFound404,
        /// Worker not found.
        WorkNotStarted,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // ink call tee callback function
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::ink_callback())]
        pub fn ink_callback(
            origin: OriginFor<T>,
            cluster_id: ClusterId,
            call_id: u128,
            args: Vec<InkArg>,
            value: BalanceOf<T>,
            error: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // get call
            let call = TEECalls::<T>::get(cluster_id, call_id).unwrap();

            // get deploy key of worker
            let deploy = wetee_worker::DeployKeys::<T>::get(call.work_id.clone());
            if deploy.is_none() || deploy.unwrap() != who {
                return Err(Error::<T>::NotAllowed403.into());
            }

            // check call type
            if call.call_type != TEECallType::Ink {
                return Err(Error::<T>::Call404.into());
            }

            // call tee error
            if error.is_some() {
                Self::deposit_event(Event::TEECallFailed {
                    cluster_id,
                    call_id,
                    error: error.unwrap(),
                });
                TEECalls::<T>::remove(cluster_id, call_id);
                return Ok(().into());
            }

            let mut call_data = Vec::new();
            call_data.append(&mut call.callback_method.encode());
            args.into_iter().for_each(|arg| {
                call_data.append(&mut arg.encode2vec());
            });

            let gas_limit = Weight::MAX;

            // call contract
            let call_result = pallet_contracts::Pallet::<T>::bare_call(
                who,
                call.org_contract,
                value,
                gas_limit,
                None,
                call_data,
                pallet_contracts::DebugInfo::UnsafeDebug,
                pallet_contracts::CollectEvents::UnsafeCollect,
                pallet_contracts::Determinism::Enforced,
            );

            // get work account
            let (owner_account, _, _, _, _, _) =
                <T as pallet::Config>::WorkExt::work_info(call.work_id.clone())?;

            // get fee
            let gas = Self::weight_to_fee(call_result.gas_consumed);

            // fee to burn
            // 销毁手续费
            // TODO 帐户余额检测
            wetee_assets::Pallet::<T>::burn_with_number(
                NATIVE_ASSET_ID,
                owner_account,
                gas.into(),
            )?;

            // remove call
            TEECalls::<T>::remove(cluster_id, call_id);

            // dispatch event
            match call_result.result {
                Ok(_success) => {
                    Self::deposit_event(Event::TEECallSuccessed {
                        cluster_id,
                        call_id,
                    });
                }
                Err(error) => {
                    let msg = handle_dispatch_error(error);
                    Self::deposit_event(Event::TEECallBackFailed {
                        cluster_id,
                        call_id,
                        error: msg.as_bytes().to_vec(),
                    });
                }
            }

            Ok(Pays::No.into())
        }

        // ink call tee callback function
        #[pallet::call_index(002)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::call_ink())]
        pub fn call_ink(
            origin: OriginFor<T>,
            work_id: WorkId,
            contract: T::AccountId,
            method: [u8; 4],
            args: Vec<InkArg>,
            value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // get deploy key of worker
            let deploy = wetee_worker::DeployKeys::<T>::get(work_id.clone());
            if deploy.is_none() || deploy.unwrap() != who {
                return Err(Error::<T>::NotAllowed403.into());
            }

            let mut call_data = Vec::new();
            call_data.append(&mut method.encode());
            args.clone().into_iter().for_each(|arg| {
                call_data.append(&mut arg.encode2vec());
            });

            let gas_limit = Weight::MAX;

            // call contract
            let call_result = pallet_contracts::Pallet::<T>::bare_call(
                who,
                contract.clone(),
                value,
                gas_limit,
                None,
                call_data,
                pallet_contracts::DebugInfo::UnsafeDebug,
                pallet_contracts::CollectEvents::UnsafeCollect,
                pallet_contracts::Determinism::Enforced,
            );

            // get work account
            let (owner_account, _, _, _, _, _) =
                <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;

            // get fee
            let gas = Self::weight_to_fee(call_result.gas_consumed);

            // fee to burn
            // 销毁手续费
            // TODO 帐户余额检测
            wetee_assets::Pallet::<T>::burn_with_number(
                NATIVE_ASSET_ID,
                owner_account,
                gas.into(),
            )?;

            if call_result.result.is_err() {
                return Err(Error::<T>::CallInkError.into());
            }

            Self::deposit_event(Event::InkCallSuccessed {
                work_id,
                contract,
                method,
                args,
            });

            Ok(Pays::No.into())
        }

        #[pallet::call_index(008)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_tee_api())]
        pub fn set_tee_api(
            origin: OriginFor<T>,
            // App id
            // 应用id
            work_id: WorkId,
            // Env
            // 环境变量
            meta: ApiMeta,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // check work owner
            let (owner_account, _, _, _, _, _) =
                <T as pallet::Config>::WorkExt::work_info(work_id.clone())?;

            ensure!(owner_account == who, Error::<T>::NotAllowed403);

            // set api meta
            <ApiMetas<T>>::set(work_id, Some(meta));

            Ok(().into())
        }

        #[pallet::call_index(009)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::delete_call())]
        pub fn delete_call(
            origin: OriginFor<T>,
            cluster_id: ClusterId,
            call_id: u128,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // get call
            let call = TEECalls::<T>::get(cluster_id, call_id).unwrap();

            // check work owner
            let (owner_account, _, _, _, _, _) =
                <T as pallet::Config>::WorkExt::work_info(call.work_id.clone())?;

            ensure!(owner_account == who, Error::<T>::NotAllowed403);

            // remove call
            TEECalls::<T>::remove(cluster_id, call_id);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // handle call from ink
        pub fn call_from_ink(
            // tee caller contract
            org_contract: T::AccountId,
            // tee caller
            org_caller: T::AccountId,
            // tee call to
            work_id: WorkId,
            // tee call method index
            method: u16,
            // tee call method index
            callback_method: [u8; 4],
            // tee call args
            args: Vec<u8>,
        ) -> result::Result<u128, DispatchError> {
            let id = <NextId<T>>::get();

            // 获取集群id
            // get cluster id
            let cid_result = wetee_worker::Pallet::<T>::work_contracts(work_id.clone());
            // TODO 集群不存在
            if cid_result.is_none() {
                return Err(Error::<T>::WorkNotFound404.into());
            }

            let cid = cid_result.unwrap();

            // 插入tee call
            // insert tee call
            let tee_call = TEECall {
                id,
                chain_id: None,
                org_contract: org_contract.clone(),
                org_caller: org_caller.clone(),
                call_type: TEECallType::Ink,
                work_id,
                method,
                callback_method,
                args,
            };
            <TEECalls<T>>::insert(cid, id, tee_call);

            Self::deposit_event(Event::<T>::TEECallStarted {
                cluster_id: cid,
                call_id: id,
            });

            <NextId<T>>::set(id + 1);
            Ok(id)
        }

        // call gas fee
        fn weight_to_fee(weight: Weight) -> u64 {
            weight.ref_time() * 20 + weight.proof_size() * 20
        }
    }
}
