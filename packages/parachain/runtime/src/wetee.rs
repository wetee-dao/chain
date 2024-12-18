use crate::*;

use crate::configs::RuntimeBlockWeights;
use frame_support::{traits::Contains, PalletId};
use orml_traits::parameter_type_with_key;
use sp_runtime::traits::Zero;
use wetee_assets::{self as wetee_assets, ext::BasicCurrencyAdapter};
use wetee_primitives::{
    traits::{GovIsJoin, UHook},
    types::{CallId, WeAssetId, NATIVE_ASSET_ID},
};
use wetee_utils::converter::{CurrencyId, CurrencyIdConvert};

/// WETEE Start
type Amount = i128;

impl pallet_insecure_randomness_collective_flip::Config for Runtime {}

parameter_types! {
    pub ServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl wetee_message_queue::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MessageProcessor = WorkerMessageProcessor;
    type Size = u32;
    type QueueChangeHandler = WorkerQueueChangeHandler;
    type QueuePausedQuery = WorkerQueuePauser;
    type HeapSize = ConstU32<{ 64 * 1024 }>;
    type MaxStale = ConstU32<128>;
    type ServiceWeight = ServiceWeight;
    type IdleMaxServiceWeight = ();
}

pub struct GovFunc;
impl GovIsJoin<RuntimeCall> for GovFunc {
    fn is_join(call: RuntimeCall) -> bool {
        match call {
            RuntimeCall::Guild(func) => match func {
                wetee_guild::Call::guild_join { .. } => true,
                _ => false,
            },
            RuntimeCall::Project(func) => match func {
                wetee_project::Call::project_join_request { .. } => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl wetee_gov::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Pledge = Pledge;
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
    type CurrencyId = CurrencyId;
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
impl UHook<AccountId, WeAssetId> for CreatedHook {
    fn run_hook(acount_id: AccountId, dao_id: WeAssetId) {
        // 以 WETEE 创建者设置为WETEE初始的 root 账户
        wetee_sudo::Account::<Runtime>::insert(dao_id, acount_id);
    }
}

impl wetee_dao::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type OrgHook = CreatedHook;
    type WeightInfo = ();
    type MaxMembers = ConstU32<1000000>;
    type PalletId = DaoPalletId;
}

parameter_types! {
    pub const MaxClassMetadata: u32 = 1;
    pub const MaxTokenMetadata: u32 = 1;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
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
    pub const MaxCreatableId: WeAssetId = 900000000;
    pub NativeCurrencyId: CurrencyId = CurrencyId {
        para_id: ParachainInfo::parachain_id().into(),
        currency_id: NATIVE_ASSET_ID,
    };
}

impl wetee_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxCreatableId = MaxCreatableId;
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = NativeCurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert<Runtime>;
}

impl wetee_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl wetee_dsecret::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();

    type OffchainSignature = Signature;
    type OffchainPublic = <Signature as Verify>::Signer;

    #[cfg(feature = "runtime-benchmarks")]
    type Helper = ();
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
    type UHook = WorkerQueueHook;
}

impl wetee_task::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type UHook = WorkerQueueHook;
}

impl wetee_gpu::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type UHook = WorkerQueueHook;
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

parameter_types! {
    pub const MatrixPalletId: PalletId = PalletId(*b"wematrix");
}
impl wetee_matrix::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type CallId = CallId;
    type PalletId = MatrixPalletId;
    type WeightInfo = ();
}

impl wetee_tee_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type WorkExt = WorkExtIns;
}

// pub struct FairlanchHook;
// impl UHook<AccountId, Fairlanch> for FairlanchHook {
//     fn run_hook(id: WorkId, _: AccountId) {
//         // 添加消息到队列
//         WeMessageQueue::enqueue_message(vec2bytes(&id.encode()), MessageOrigin::FairLaunch);
//     }
// }

parameter_types! {
    pub const FairlanchPalletId: PalletId = PalletId(*b"fair0000");
    pub const EpochBlock: u32 = 14400;
}

impl wetee_fairlanch::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type PalletId = FairlanchPalletId;
    type EpochBlock = EpochBlock;
}
// WETEE END
