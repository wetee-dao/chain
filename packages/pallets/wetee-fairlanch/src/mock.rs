#![allow(dead_code)]
#![allow(unused_variables)]

use crate as wetee_fairlanch;
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
use wetee_primitives::{traits::UHook, types::WeAssetId};

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
        DAO: wetee_dao::{ Pallet, Call, Event<T>, Storage },
        Asset: wetee_assets::{ Pallet, Call, Event<T>, Storage },
        Fairlanch: wetee_fairlanch::{ Pallet, Call, Event<T>, Storage },
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

pub struct OrgHook;
impl UHook<AccountId, WeAssetId> for OrgHook {
    fn run_hook(id: AccountId, dao_id: WeAssetId) {}
}

impl wetee_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = u64;
    type PalletId = DaoPalletId;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type OrgHook = OrgHook;
}

pub struct AuraAccountAdapter;
impl frame_support::traits::FindAuthor<AccountId> for AuraAccountAdapter {
    fn find_author<'a, I>(digests: I) -> Option<AccountId>
    where
        I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
    {
        return Some(BOB);
    }
}

parameter_types! {
    pub const FairlanchPalletId: PalletId = PalletId(*b"fair0000");
    pub const EpochBlock: u32 = 50;
}

impl wetee_fairlanch::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type FindAuthor = AuraAccountAdapter;
    type PalletId = FairlanchPalletId;
    type EpochBlock = EpochBlock;
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
    type CurrencyId = WeAssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type MaxLocks = MaxLocks;
    type MaxReserves = TokensMaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemovalWhitelist = DustRemovalWhitelist;
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxCreatableId: WeAssetId = 100000;
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
        balances: vec![(ALICE, 100000000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
