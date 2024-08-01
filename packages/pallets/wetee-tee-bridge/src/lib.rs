#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{fungible::Inspect, ConstU32};
use parity_scale_codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;
use sp_runtime::RuntimeDebug;
use sp_std::result;

use orml_traits::MultiCurrency;
use wetee_primitives::types::WorkId;

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
    pub org_id: AccountId,
    // tee call type
    pub call_type: TEECallType,
    // tee call to
    pub work_id: WorkId,
    // tee call method index
    pub method: u16,
    // tee call params
    pub params: Vec<u8>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    type BalanceOf<T> = <<T as pallet_contracts::Config>::Currency as Inspect<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + wetee_org::Config + pallet_contracts::Config + wetee_assets::Config
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

    #[pallet::storage]
    #[pallet::getter(fn next_id)]
    pub type NextId<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tee_calls)]
    pub type TEECalls<T: Config> =
        StorageMap<_, Identity, u128, TEECall<T::AccountId>, OptionQuery>;

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
        Call404,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::sudo())]
        pub fn ink_callback(
            origin: OriginFor<T>,
            call_id: u128,
            data: Vec<u8>,
            value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let call = TEECalls::<T>::get(call_id).unwrap();

            let gas_limit = Weight::from_all(40_000);
            let result = pallet_contracts::Pallet::<T>::bare_call(
                who,
                call.org_id,
                value,
                gas_limit,
                None,
                data,
                pallet_contracts::DebugInfo::UnsafeDebug,
                pallet_contracts::CollectEvents::UnsafeCollect,
                pallet_contracts::Determinism::Enforced,
            );

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn call_from_ink(
            org: T::AccountId,
            // tee call to
            work_id: WorkId,
            // tee call method index
            method: u16,
            // tee call params
            params: Vec<u8>,
        ) -> result::Result<bool, DispatchError> {
            let id = <NextId<T>>::get();

            let tee_call = TEECall {
                id,
                chain_id: None,
                org_id: org.clone(),
                call_type: TEECallType::Ink,
                work_id,
                method,
                params,
            };
            <TEECalls<T>>::insert(id, tee_call);

            <NextId<T>>::set(id + 1);
            Ok(true)
        }
    }
}
