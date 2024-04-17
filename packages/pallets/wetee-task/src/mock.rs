#![allow(dead_code)]
#![allow(unused_variables)]

use crate as wetee_task;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, Contains},
    PalletId,
};
use frame_system;
use orml_traits::parameter_type_with_key;
use sp_runtime::{traits::Zero, BuildStorage};
use sp_std::result::Result;
use wetee_assets::asset_adaper_in_pallet::BasicCurrencyAdapter;
use wetee_primitives::{
    traits::UHook,
    types::{DaoAssetId, WorkId},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;

type Amount = i128;
type Balance = u64;
pub type AccountId = u64;
pub type BlockNumber = u64;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},

        Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        WeteeAsset: wetee_assets::{ Pallet, Call, Event<T>, Storage },
        WETEE: wetee_org::{ Pallet, Call, Event<T>, Storage },
        WeteeTask: wetee_task::{ Pallet, Call, Event<T>, Storage },
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
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            _ => Ok(0u64),
        }
    }
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

impl wetee_org::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = u64;
    type PalletId = DaoPalletId;
    type OrgHook = ();
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
}

pub struct WorkerQueueHook;
impl UHook<WorkId, AccountId> for WorkerQueueHook {
    fn run_hook(id: WorkId, dao_id: DaoAssetId) {}
}

impl wetee_task::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type UHook = WorkerQueueHook;
}

parameter_types! {
    pub const TokensMaxReserves: u32 = 50;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: u64| -> Balance {
        Zero::zero()
    };
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
    fn contains(a: &AccountId) -> bool {
        get_all_module_accounts().contains(a)
    }
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
    vec![]
}

impl orml_tokens::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CurrencyHooks = ();
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = DaoAssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type MaxLocks = MaxLocks;
    type MaxReserves = TokensMaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemovalWhitelist = DustRemovalWhitelist;
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxCreatableId: DaoAssetId = 100000;
}

impl wetee_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiAsset = Tokens;
    type NativeAsset = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
}

pub fn new_test_run() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 10000000), (BOB, 10000000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
