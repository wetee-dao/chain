use frame_support::{
    storage_alias,
    traits::{Get, UncheckedOnRuntimeUpgrade},
};

#[cfg(feature = "try-runtime")]
use alloc::vec::Vec;

use crate::AssetDeposit;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::pallet_prelude::RuntimeDebug;
use parity_scale_codec::{Decode, Encode};
use wetee_primitives::types::ClusterId;

// #[docify::export]
#[derive(
    Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
)]
pub struct Value<BlockNumber, Balance> {
    pub k1: ClusterId,
    pub k2: BlockNumber,
    pub value: Option<AssetDeposit<Balance>>,
}

/// Collection of storage item formats from the previous storage version.
///
/// Required so we can read values in the v0 storage format during the migration.
mod v1 {
    use super::*;
    use crate::*;

    #[storage_alias]
    pub type Deposits<T: crate::Config> = StorageDoubleMap<
        crate::Pallet<T>,
        Identity,
        ClusterId,
        Identity,
        BlockNumberFor<T>,
        Deposit<BalanceOf<T>>,
        OptionQuery,
    >;
}

pub struct WorkerMigrateV1ToV2<T: crate::Config>(core::marker::PhantomData<T>);

impl<T: crate::Config> UncheckedOnRuntimeUpgrade for WorkerMigrateV1ToV2<T> {
    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
        let prev_count = v1::Deposits::<T>::iter().count();
        Ok((prev_count as u32).encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
        Ok(())
    }

    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        v1::Deposits::<T>::iter().for_each(|(k1, k2, deposit)| {
            crate::DepositedAssets::<T>::insert(
                k1,
                k2,
                AssetDeposit {
                    asset_id: 0,
                    deposit: deposit.deposit,
                    usd: deposit.deposit,
                    cpu: deposit.cpu,
                    mem: deposit.mem,
                    cvm_cpu: deposit.cvm_cpu,
                    cvm_mem: deposit.cvm_mem,
                    disk: deposit.disk,
                    gpu: deposit.gpu,
                },
            );
        });

        T::DbWeight::get().reads_writes(10000, 10000)
    }
}

pub type Migration<T> = frame_support::migrations::VersionedMigration<
    2, // The migration will only execute when the on-chain storage version is 0
    3, // The on-chain storage version will be set to 1 after the migration is complete
    WorkerMigrateV1ToV2<T>,
    crate::pallet::Pallet<T>,
    <T as frame_system::Config>::DbWeight,
>;
