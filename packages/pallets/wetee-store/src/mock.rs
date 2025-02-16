#![allow(dead_code)]
#![allow(unused_variables)]

use crate::pallet as wetee_store;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, Contains},
    PalletId,
};
use frame_system;
use orml_traits::parameter_type_with_key;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_runtime::traits::Convert;
use sp_runtime::{traits::Zero, BuildStorage};
use sp_std::result::Result;

use wetee_assets::{self as wetee_assets, ext::BasicCurrencyAdapter};
use wetee_primitives::{
    traits::{GovIsJoin, PalletGet},
    types::{CallId, WeAssetId},
};

type Amount = i128;
pub type Balance = u64;
pub type BlockNumber = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: u64 = 0;
pub const BOB: u64 = 1;
pub type AccountId = u64;

frame_support::construct_runtime!(
    pub enum Test{
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},

        Base: wetee_dao::{ Pallet, Call, Event<T>, Storage },
        Asset: wetee_assets::{ Pallet, Call, Event<T>, Storage },
        Fairlanch: wetee_fairlanch::{ Pallet, Call, Event<T>, Storage },
        Store: wetee_store::{ Pallet, Call, Event<T>, Storage },
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

#[derive(
    PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen, Default,
)]
pub struct Vote(pub Balance);

pub struct GovFunc;
impl GovIsJoin<RuntimeCall> for GovFunc {
    fn is_join(_call: RuntimeCall) -> bool {
        false
    }
}

impl PalletGet<RuntimeCall> for GovFunc {
    fn get_pallet_id(_call: RuntimeCall) -> u16 {
        4
    }
}

parameter_types! {
    pub const TokensMaxReserves: u32 = 50;
}

pub struct DustRemovalWhitelist;
impl Contains<u64> for DustRemovalWhitelist {
    fn contains(a: &u64) -> bool {
        get_all_module_accounts().contains(a)
    }
}

pub fn get_all_module_accounts() -> Vec<u64> {
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

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
}

impl wetee_dao::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type CrossCall = ();
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type PalletId = DaoPalletId;
}

impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            _ => Err(()),
        }
    }
}

parameter_types! {
    pub const MaxClassMetadata: u32 = 1;
    pub const MaxTokenMetadata: u32 = 1;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: u64| -> Balance {
        Zero::zero()
    };
}

pub struct MockDustRemovalWhitelist;
impl Contains<u64> for MockDustRemovalWhitelist {
    fn contains(a: &u64) -> bool {
        *a == ALICE
    }
}

type CurrencyId = u64;
parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxCreatableId: WeAssetId = 90000;
    pub const GetNativeCurrencyId: CurrencyId = 1;
}

pub struct CurrencyIdConvert;
impl Convert<(u32, WeAssetId), CurrencyId> for CurrencyIdConvert {
    fn convert(id: (u32, WeAssetId)) -> CurrencyId {
        return id.1;
    }
}
impl Convert<CurrencyId, WeAssetId> for CurrencyIdConvert {
    fn convert(id: CurrencyId) -> WeAssetId {
        return id;
    }
}

impl wetee_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert;
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

impl wetee_store::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub(crate) fn new_test_run() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 10000000), (BOB, 100000000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        // Timestamp::set_timestamp(12345);
    });
    ext
}
