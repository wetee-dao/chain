#![allow(dead_code)]
#![allow(unused_variables)]

use crate as wetee_gov;
use crate::PledgeTrait;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, Contains},
    PalletId,
};
use orml_traits::parameter_type_with_key;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, BuildStorage, DispatchError};
use sp_std::result::Result;
use wetee_assets::asset_adaper_in_pallet::BasicCurrencyAdapter;
use wetee_primitives::{
    traits::{GovIsJoin, PalletGet, UHook},
    types::{CallId, DaoAssetId},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
pub type Block = frame_system::mocking::MockBlock<Test>;
type Amount = i128;
pub type Balance = u64;
pub type BlockNumber = u64;

pub const ALICE: u64 = 0;
pub const BOB: u64 = 1;

pub const DAO_ID: u64 = 5000;
pub const P_ID: u32 = 0;

parameter_types! {
    pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
}

frame_support::construct_runtime!(
    pub enum Test{
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},

        WETEE: wetee_org::{ Pallet, Call, Event<T>, Storage },
        WeteeAsset: wetee_assets::{ Pallet, Call, Event<T>, Storage },
        WETEESudo: wetee_sudo::{ Pallet, Call, Event<T>, Storage },
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

pub struct DustRemovalWhitelist;
impl Contains<u64> for DustRemovalWhitelist {
    fn contains(a: &u64) -> bool {
        get_all_module_accounts().contains(a)
    }
}

pub fn get_all_module_accounts() -> Vec<u64> {
    vec![]
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: u64| -> Balance {
        Zero::zero()
    };
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
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type DustRemovalWhitelist = DustRemovalWhitelist;
}

impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        match call {
            RuntimeCall::WETEEGov(func) => match func {
                wetee_gov::Call::submit_proposal { .. } => Ok(401 as CallId),
                wetee_gov::Call::deposit_proposal { .. } => Ok(403 as CallId),
                wetee_gov::Call::vote_for_prop { .. } => Ok(404 as CallId),
                wetee_gov::Call::cancel_vote { .. } => Ok(405 as CallId),
                wetee_gov::Call::run_proposal { .. } => Ok(406 as CallId),
                wetee_gov::Call::unlock { .. } => Ok(407 as CallId),
                wetee_gov::Call::set_max_pre_props { .. } => Ok(409 as CallId),
                wetee_gov::Call::update_vote_model { .. } => Ok(415 as CallId),
                wetee_gov::Call::set_periods { .. } => Ok(415 as CallId),
                _ => Err(()),
            },
            _ => Err(()),
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

pub struct CreatedHook;
impl UHook<u64, DaoAssetId> for CreatedHook {
    fn run_hook(acount_id: u64, dao_id: DaoAssetId) {
        // 以 WETEE 创建者设置为WETEE初始的 root 账户
        wetee_sudo::Account::<Test>::insert(dao_id, acount_id);
    }
}

impl wetee_org::Config for Test {
    type PalletId = DaoPalletId;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type OrgHook = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
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

#[derive(
    PartialEq,
    Eq,
    Encode,
    Decode,
    sp_core::RuntimeDebug,
    Clone,
    TypeInfo,
    Copy,
    MaxEncodedLen,
    Default,
)]
pub struct Vote(pub Balance);

impl PledgeTrait<u64, u64, u64, u64, DispatchError> for Vote {
    fn try_vote(
        &self,
        _who: &u64,
        _dao_id: &u64,
        vote_model: u8,
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
    fn get_pallet_id(call: RuntimeCall) -> u16 {
        4
    }
}

impl wetee_gov::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Vote;
    type WeightInfo = ();
    type GovFunc = GovFunc;
}

impl wetee_sudo::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

pub fn new_test_run() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 100000), (BOB, 10000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
