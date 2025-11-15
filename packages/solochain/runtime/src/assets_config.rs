use crate::{AccountId, Balance, Balances, Runtime, RuntimeEvent, EXISTENTIAL_DEPOSIT};
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU32},
};
use frame_system::EnsureSigned;

const UNITS: Balance = 10_000_000_000;
const DOLLARS: Balance = UNITS;
const MILLICENTS: Balance = DOLLARS / 100_000;

// System parachain deposit formula:
// (items * 20 * DOLLARS + bytes * 100 * MILLICENTS) / 100
const fn system_para_deposit(items: u32, bytes: u32) -> Balance {
	((items as Balance) * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS) / 100
}

parameter_types! {
	pub const AssetDeposit: Balance = system_para_deposit(1, 190); // 2_019_000_000
	pub const AssetAccountDeposit: Balance = system_para_deposit(1, 16); // 2_001_600_000
	pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT; // 100_000_000
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = system_para_deposit(1, 68); // 2_006_800_000
	pub const MetadataDepositPerByte: Balance = system_para_deposit(0, 1); // 100_000
}

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
	type Holder = ();
}
