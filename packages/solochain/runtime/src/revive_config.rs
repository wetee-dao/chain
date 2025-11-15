use crate::{
    Address, Balance, Balances, EthExtraImpl, Runtime, RuntimeCall, RuntimeEvent,
    RuntimeHoldReason, RuntimeOrigin, Signature, Timestamp,
};

use frame_support::{
    parameter_types,
    traits::{ConstBool, ConstU32, ConstU64},
};
use frame_system::EnsureSigned;
use pallet_assets_precompiles::{InlineIdConfig, ERC20};
use sp_runtime::{FixedU128, Perbill};

// Unit = the base number of indivisible units for balances
const UNIT: Balance = 1_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;

const fn deposit(items: u32, bytes: u32) -> Balance {
    (items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

parameter_types! {
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerChildTrieItem: Balance = deposit(1, 0) / 100;
    pub const DepositPerByte: Balance = deposit(0, 1);
    pub const DefaultDepositLimit: Balance = deposit(1024, 1024 * 1024);
    pub const CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
    pub const MaxEthExtrinsicWeight: FixedU128 = FixedU128::from_rational(1,2);
}

impl pallet_revive::Config for Runtime {
    type Time = Timestamp;
    type Balance = Balance;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type RuntimeOrigin = RuntimeOrigin;
    type DepositPerItem = DepositPerItem;
    type DepositPerChildTrieItem = DepositPerChildTrieItem;
    type DepositPerByte = DepositPerByte;
    type WeightInfo = pallet_revive::weights::SubstrateWeight<Self>;
    type Precompiles = (ERC20<Self, InlineIdConfig<0x120>, ()>,);
    type AddressMapper = pallet_revive::AccountId32Mapper<Self>;
    type RuntimeMemory = ConstU32<{ 128 * 1024 * 1024 }>;
    type PVFMemory = ConstU32<{ 512 * 1024 * 1024 }>;
    type UnsafeUnstableInterface = ConstBool<true>;
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type UploadOrigin = EnsureSigned<Self::AccountId>;
    type InstantiateOrigin = EnsureSigned<Self::AccountId>;
    type ChainId = ConstU64<420_420_420>;
    type NativeToEthRatio = ConstU32<100_000_000>; // 10^(18 - 10) Eth is 10^18, Native is 10^10.
    type FindAuthor = <Runtime as pallet_authorship::Config>::FindAuthor;
    type AllowEVMBytecode = ConstBool<false>;
    type FeeInfo = pallet_revive::evm::fees::Info<Address, Signature, EthExtraImpl>;
    type MaxEthExtrinsicWeight = MaxEthExtrinsicWeight;
    type DebugEnabled = ConstBool<false>;
}
