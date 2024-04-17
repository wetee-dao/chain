#![allow(dead_code)]
use super::*;

use crate as wetee_guild;
use frame_support::{construct_runtime, derive_impl, parameter_types, traits::Contains, PalletId};
use orml_traits::parameter_type_with_key;
use sp_runtime::{traits::Zero, BuildStorage};
use wetee_gov::traits::PledgeTrait;

use wetee_assets::{self as wetee_assets, asset_adaper_in_pallet::BasicCurrencyAdapter};
use wetee_primitives::{
    traits::{GovIsJoin, PalletGet, UHook},
    types::{CallId, DaoAssetId},
};

type Amount = i128;
pub type Balance = u64;
pub type BlockNumber = u64;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: u64 = 0;
pub const BOB: u64 = 1;

construct_runtime!(
    pub enum Test{
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},

        WETEE: wetee_org::{ Pallet, Call, Event<T>, Storage },
        WeteeAsset: wetee_assets::{ Pallet, Call, Event<T>, Storage },
        WETEESudo: wetee_sudo::{ Pallet, Call, Event<T>, Storage },
        WETEEGuild: wetee_guild::{ Pallet, Call, Event<T>, Storage },
        WETEEGov: wetee_gov::{ Pallet, Call, Event<T>, Storage },
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

impl PledgeTrait<u64, u64, u64, u64, DispatchError> for Vote {
    fn try_vote(
        &self,
        _who: &u64,
        _dao_id: &u64,
        _vote_model: u8,
    ) -> Result<(u64, u64), DispatchError> {
        Ok((100u64, 100u64))
    }

    fn vote_end_do(&self, _who: &u64, _dao_id: &u64) -> Result<(), DispatchError> {
        Ok(())
    }
}

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

impl wetee_gov::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Vote;
    type WeightInfo = ();
    type GovFunc = GovFunc;
}

impl wetee_guild::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
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
    type CurrencyId = DaoAssetId;
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

pub struct CreatedHook;
impl UHook<u64, DaoAssetId> for CreatedHook {
    fn run_hook(acount_id: u64, dao_id: DaoAssetId) {
        // 以 WETEE 创建者设置为WETEE初始的 root 账户
        wetee_sudo::Account::<Test>::insert(dao_id, acount_id);
    }
}

impl wetee_org::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type OrgHook = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type PalletId = DaoPalletId;
}

impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            // dao
            RuntimeCall::WETEEGuild(func) => match func {
                wetee_guild::Call::guild_join { .. } => Ok(401 as CallId),
                _ => Err(()),
            },
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

impl wetee_sudo::Config for Test {
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
