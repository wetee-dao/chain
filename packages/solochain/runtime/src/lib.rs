#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
pub mod configs;

extern crate alloc;
use alloc::vec::Vec;
use sp_runtime::{
    generic, impl_opaque_keys,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiAddress, MultiSignature,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub mod genesis_config_presets;

// Import the WETEE pallet.
use pallet_revive::evm::runtime::EthExtra;
mod call;
mod vote;
pub use vote::*;
mod worker;
pub use worker::*;
mod wetee;
pub use wetee::*;
mod revive;
// pub use contracts::*;
// mod contract_extension;

pub use wetee_app::Call as AppCall;
pub use wetee_assets::Call as AssetsCall;
pub use wetee_dao::Call as DaoCall;
pub use wetee_dsecret::Call as SecretCall;
pub use wetee_fairlanch::Call as FairlanchCall;
pub use wetee_gov::Call as GovCall;
pub use wetee_gpu::Call as GpuCall;
pub use wetee_guild::Call as GuildCall;
pub use wetee_matrix::Call as MatrixCall;
pub use wetee_project::Call as ProjectCall;
pub use wetee_sudo::Call as SudoCall;
pub use wetee_task::Call as TaskCall;
pub use wetee_tee_bridge::Call as TeeBridgeCall;
pub use wetee_treasury::Call as TreasuryCall;
pub use wetee_worker::Call as WorkerCall;
// End WETEE pallet.

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    use sp_runtime::{
        generic,
        traits::{BlakeTwo256, Hash as HashT},
    };

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque block hash type.
    pub type Hash = <BlakeTwo256 as HashT>::Output;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("WeTEE-dev"),
    impl_name: alloc::borrow::Cow::Borrowed("WeTEE-dev-runtime"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 110,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

mod block_times {
    /// This determines the average expected block time that we are targeting. Blocks will be
    /// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
    /// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
    /// slot_duration()`.
    ///
    /// Change this to adjust the block time.
    pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

    // NOTE: Currently it is not possible to change the slot duration after the chain has started.
    // Attempting to do so will brick block production.
    pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub const BLOCK_HASH_COUNT: BlockNumber = 2400;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The `TransactionExtension` to the basic transaction logic.
pub type TxExtension = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
    frame_system::WeightReclaim<Runtime>,
);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EthExtraImpl;

impl EthExtra for EthExtraImpl {
    type Config = Runtime;
    type Extension = TxExtension;

    fn get_eth_extension(nonce: u32, tip: Balance) -> Self::Extension {
        (
            frame_system::CheckNonZeroSender::<Runtime>::new(),
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckEra::from(crate::generic::Era::Immortal),
            frame_system::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
            frame_system::WeightReclaim::<Runtime>::new(),
        )
    }
}

/// Unchecked extrinsic type as expected by this runtime.
// pub type UncheckedExtrinsic =
//     generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;
pub type UncheckedExtrinsic =
    pallet_revive::evm::runtime::UncheckedExtrinsic<Address, Signature, EthExtraImpl>;

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, TxExtension>;

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

impl TryFrom<RuntimeCall> for pallet_revive::Call<Runtime> {
    type Error = ();
    fn try_from(value: RuntimeCall) -> Result<Self, Self::Error> {
        match value {
            RuntimeCall::Revive(call) => Ok(call),
            _ => Err(()),
        }
    }
}

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp;

    #[runtime::pallet_index(2)]
    pub type Aura = pallet_aura;

    #[runtime::pallet_index(3)]
    pub type Grandpa = pallet_grandpa;

    #[runtime::pallet_index(4)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(5)]
    pub type TransactionPayment = pallet_transaction_payment;

    #[runtime::pallet_index(6)]
    pub type Sudo = pallet_sudo;

    #[runtime::pallet_index(7)]
    pub type Authorship = pallet_authorship;

    // WETEE
    #[runtime::pallet_index(101)]
    pub type Tokens = orml_tokens;
    #[runtime::pallet_index(102)]
    pub type RandomnessCollectiveFlip = pallet_insecure_randomness_collective_flip;
    #[runtime::pallet_index(103)]
    pub type WeMessageQueue = wetee_message_queue;
    #[runtime::pallet_index(104)]
    pub type Utility = pallet_utility;
    #[runtime::pallet_index(105)]
    pub type Dao = wetee_dao;
    #[runtime::pallet_index(106)]
    pub type Asset = wetee_assets;
    #[runtime::pallet_index(107)]
    pub type WeSudo = wetee_sudo;
    #[runtime::pallet_index(108)]
    pub type Guild = wetee_guild;
    #[runtime::pallet_index(109)]
    pub type Project = wetee_project;
    #[runtime::pallet_index(110)]
    pub type Gov = wetee_gov;
    #[runtime::pallet_index(111)]
    pub type Treasury = wetee_treasury;
    #[runtime::pallet_index(112)]
    pub type App = wetee_app;
    #[runtime::pallet_index(113)]
    pub type Task = wetee_task;
    #[runtime::pallet_index(114)]
    pub type Gpu = wetee_gpu;
    #[runtime::pallet_index(115)]
    pub type Worker = wetee_worker;
    #[runtime::pallet_index(8)]
    pub type Revive = pallet_revive;
    #[runtime::pallet_index(117)]
    pub type DSecret = wetee_dsecret;
    #[runtime::pallet_index(118)]
    pub type Bridge = wetee_tee_bridge;
    #[runtime::pallet_index(119)]
    pub type Matrix = wetee_matrix;
    #[runtime::pallet_index(120)]
    pub type Fairlanch = wetee_fairlanch;
    #[runtime::pallet_index(124)]
    pub type Store = wetee_store;
    // WETEE end
}
