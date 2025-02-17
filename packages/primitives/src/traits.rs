use crate::types::{ClusterLevel, Cr, TEEVersion, WorkId, WorkStatus};
use scale_info::prelude::vec::Vec;
use sp_runtime::DispatchError;
use sp_std::result::Result;

pub struct BadOrigin;

impl From<BadOrigin> for &'static str {
    fn from(_: BadOrigin) -> &'static str {
        "invalid origin"
    }
}

pub trait CrossCall<A, B> {
    fn call(a: A) -> Result<B, DispatchError>;
}

impl<AccountId: Clone, WeAssetId: Clone> CrossCall<(AccountId, WeAssetId), ()> for () {
    fn call(_a: (AccountId, WeAssetId)) -> Result<(), DispatchError> {
        Ok(())
    }
}

impl<AccountId: Clone, WeAssetId: Clone>
    CrossCall<(AccountId, WeAssetId, Vec<u8>, Vec<u8>, u8, u128), ()> for ()
where
    AccountId: Clone,
    WeAssetId: Clone,
{
    fn call(
        _args: (AccountId, WeAssetId, Vec<u8>, Vec<u8>, u8, u128),
    ) -> Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}

pub trait GovIsJoin<RuntimeCall> {
    fn is_join(cll: RuntimeCall) -> bool;
}

impl<RuntimeCall: Clone> GovIsJoin<RuntimeCall> for () {
    fn is_join(_call: RuntimeCall) -> bool {
        return true;
    }
}

pub trait PalletGet<RuntimeCall> {
    fn get_pallet_id(call: RuntimeCall) -> u16;
}

impl<RuntimeCall: Clone> PalletGet<RuntimeCall> for () {
    fn get_pallet_id(_call: RuntimeCall) -> u16 {
        return 0;
    }
}

pub trait WorkExt<AccountId, Balance> {
    fn work_info(
        work: WorkId,
    ) -> Result<
        (
            AccountId,
            Cr,
            ClusterLevel,
            WorkStatus,
            TEEVersion,
            Option<u128>,
        ),
        DispatchError,
    >;
    fn set_work_status(w: WorkId, status: u8) -> Result<bool, DispatchError>;
    fn calculate_fee(work: WorkId) -> Result<Balance, DispatchError>;
    fn pay_run_fee(work: WorkId, to: AccountId, fee: Balance) -> Result<u8, DispatchError>;
    fn try_stop(account: AccountId, work: WorkId) -> Result<bool, DispatchError>;
}
