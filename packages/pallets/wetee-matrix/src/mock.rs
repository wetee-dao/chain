use crate as wetee_dao;
use frame_support::{derive_impl, parameter_types, traits::Contains, PalletId};
use sp_runtime::BuildStorage;
use sp_std::result::Result;
use wetee_primitives::{traits::CrossCall, types::WorkId};

// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test{
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},

        WETEE: wetee_dao::{ Pallet, Call, Event<T>, Storage },
    }
);

pub struct BlockEverything;
impl Contains<RuntimeCall> for BlockEverything {
    fn contains(_: &RuntimeCall) -> bool {
        false
    }
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<Balance>;
}

impl TryFrom<RuntimeCall> for u64 {
    type Error = ();
    fn try_from(_call: RuntimeCall) -> Result<Self, Self::Error> {
        Ok(0u64)
    }
}

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
}

pub struct WorkerQueueHook;
impl CrossCall<(WorkId, u64), ()> for WorkerQueueHook {
    fn call(
        (_id, _dao_id): (WorkId, u64),
    ) -> sp_std::result::Result<(), sp_runtime::DispatchError> {
        Ok(())
    }
}
impl wetee_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletId = DaoPalletId;
    type CallId = u64;
    type WeightInfo = ();
}

pub fn new_test_run() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    t.into()
}
