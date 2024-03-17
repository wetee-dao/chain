use crate::*;

use frame_support::{traits::Contains, PalletId};
use orml_traits::parameter_type_with_key;
pub use scale_info::TypeInfo;
use sp_runtime::traits::Zero;
use wetee_assets::{self as wetee_assets, asset_adaper_in_pallet::BasicCurrencyAdapter};
use wetee_primitives::{
    traits::{AfterCreate, GovIsJoin},
    types::{CallId, DaoAssetId},
};

/// WETEE Start
type Amount = i128;

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub ServiceWeight: Option<Weight> = Some(Perbill::from_percent(20) * BlockWeights::get().max_block);
}

impl pallet_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MessageProcessor = WorkerMessageProcessor;
    type Size = u32;
    type QueueChangeHandler = WorkerQueueChangeHandler;
    type QueuePausedQuery = WorkerQueuePauser;
    type HeapSize = ConstU32<{ 64 * 1024 }>;
    type MaxStale = ConstU32<128>;
    type ServiceWeight = ServiceWeight;
}

pub struct GovFunc;
impl GovIsJoin<RuntimeCall> for GovFunc {
    fn is_join(call: RuntimeCall) -> bool {
        match call {
            RuntimeCall::WeteeGuild(func) => match func {
                wetee_guild::Call::guild_join { .. } => true,
                _ => false,
            },
            RuntimeCall::WeteeProject(func) => match func {
                wetee_project::Call::project_join_request { .. } => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl wetee_gov::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Pledge<Balance>;
    type GovFunc = GovFunc;
    type WeightInfo = ();
}

impl wetee_project::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const TokensMaxReserves: u32 = 50;
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
    fn contains(_a: &AccountId) -> bool {
        false
    }
}

impl orml_tokens::Config for Runtime {
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
    pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
}

pub struct CreatedHook;
impl AfterCreate<AccountId, DaoAssetId> for CreatedHook {
    fn run_hook(acount_id: AccountId, dao_id: DaoAssetId) {
        // 以 WETEE 创建者设置为WETEE初始的 root 账户
        wetee_sudo::Account::<Runtime>::insert(dao_id, acount_id);
    }
}

impl wetee_org::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type AfterCreate = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type PalletId = DaoPalletId;
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
impl Contains<AccountId> for MockDustRemovalWhitelist {
    fn contains(_a: &AccountId) -> bool {
        false
    }
}

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxCreatableId: DaoAssetId = 90000;
}

impl wetee_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiAsset = Tokens;
    type NativeAsset = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
}

impl wetee_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl wetee_guild::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxApprovals: u32 = 100;
}

impl wetee_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl wetee_app::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type AfterCreate = WorkerQueueHook;
}

impl wetee_task::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type AfterCreate = WorkerQueueHook;
}

impl wetee_gpu::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type AfterCreate = WorkerQueueHook;
}

impl wetee_worker::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type WorkExt = WorkExtIns;
}

impl pallet_utility::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

// WETEE END
