use crate::*;

// use wetee_primitives::types::WeAssetId;

// impl orml_xtokens::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Balance = Balance;
//     type CurrencyId = WeAssetId;
//     type CurrencyIdConvert = CurrencyIdConvert<ParachainInfo, Runtime>;
//     type AccountIdToLocation = AccountIdToLocation;
//     type UniversalLocation = KusamaUniversalLocation;
//     type SelfLocation = SelfLocation;
//     #[cfg(feature = "runtime-benchmarks")]
//     type XcmExecutor = bifrost_primitives::MockXcmExecutor;
//     #[cfg(not(feature = "runtime-benchmarks"))]
//     type XcmExecutor = XcmExecutor<XcmConfig>;
//     type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
//     type BaseXcmWeight = BaseXcmWeight;
//     type MaxAssetsForTransfer = MaxAssetsForTransfer;
//     type MinXcmFee = ParachainMinFee;
//     type LocationsFilter = Everything;
//     type ReserveProvider = RelativeReserveProvider;
//     type RateLimiter = ();
//     type RateLimiterId = ();
// }
