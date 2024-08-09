use core::result;

use crate::types::{ClusterLevel, Cr, TEEVersion, WorkId, WorkStatus};
use sp_runtime::DispatchError;

pub struct BadOrigin;

impl From<BadOrigin> for &'static str {
    fn from(_: BadOrigin) -> &'static str {
        "无效的用户"
    }
}

pub trait UHook<AccountId, DaoAssetId> {
    fn run_hook(a: AccountId, b: DaoAssetId);
}

impl<AccountId: Clone, DaoAssetId: Clone> UHook<AccountId, DaoAssetId> for () {
    fn run_hook(_a: AccountId, _b: DaoAssetId) {}
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
    ) -> result::Result<(AccountId, Cr, ClusterLevel, WorkStatus, TEEVersion), DispatchError>;
    fn set_work_status(w: WorkId, status: u8) -> result::Result<bool, DispatchError>;
    fn calculate_fee(work: WorkId) -> result::Result<Balance, DispatchError>;
    fn pay_run_fee(work: WorkId, to: AccountId, fee: Balance) -> result::Result<u8, DispatchError>;
    fn try_stop(account: AccountId, work: WorkId) -> result::Result<bool, DispatchError>;
}
