use core::result;

use crate::types::DaoAssetId;
use sp_runtime::DispatchError;

pub struct BadOrigin;

impl From<BadOrigin> for &'static str {
    fn from(_: BadOrigin) -> &'static str {
        "无效的用户"
    }
}

pub trait SetCollectiveMembers<AccountId: Clone + Ord, DispathErr> {
    fn set_members_sorted(
        dao_id: DaoAssetId,
        members: &[AccountId],
        prime: Option<AccountId>,
    ) -> result::Result<(), DispathErr>;
}

pub trait AfterCreate<AccountId, DaoAssetId> {
    fn run_hook(a: AccountId, b: DaoAssetId);
}

impl<AccountId: Clone, DaoAssetId: Clone> AfterCreate<AccountId, DaoAssetId> for () {
    fn run_hook(_a: AccountId, _b: DaoAssetId) {}
}

impl<AccountId: Clone + Ord> SetCollectiveMembers<AccountId, DispatchError> for () {
    fn set_members_sorted(
        _dao_id: DaoAssetId,
        _members: &[AccountId],
        _prime: Option<AccountId>,
    ) -> result::Result<(), DispatchError> {
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
