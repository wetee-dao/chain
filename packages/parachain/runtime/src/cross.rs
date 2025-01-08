use crate::*;

use crate::configs::xcm_config::{MaxInstructions, UnitWeightCost, UniversalLocation, XcmConfig};
use frame_support::traits::Everything;
use frame_system::EnsureRoot;
use orml_traits::location::RelativeReserveProvider;
use orml_traits::parameter_type_with_key;
use sp_runtime::traits::Convert;
use wetee_utils::converter::{CurrencyId, CurrencyIdConvert};
use xcm::v4::{prelude::*, Weight};
use xcm_builder::FixedWeightBounds;
use xcm_executor::XcmExecutor;

impl orml_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type SovereignOrigin = EnsureRoot<AccountId>;
}

pub struct AccountIdToLocation;
impl Convert<AccountId, Location> for AccountIdToLocation {
    fn convert(account: AccountId) -> Location {
        Location::new(
            0,
            [AccountId32 {
                network: None,
                id: account.into(),
            }],
        )
    }
}

parameter_types! {
    pub SelfLocation: Location = Location::here();
    pub const MaxAssetsForTransfer: usize = 3;
    pub const BaseXcmWeight: Weight = Weight::from_parts(1000_000_000u64, 0);
}

parameter_type_with_key! {
    pub ParachainMinFee: |_location: Location| -> Option<u128> {
        Some(u128::MAX)
    };
}

impl orml_xtokens::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type CurrencyId = CurrencyId;
    type CurrencyIdConvert = CurrencyIdConvert<Runtime>;
    type AccountIdToLocation = AccountIdToLocation;
    type UniversalLocation = UniversalLocation;
    type SelfLocation = SelfLocation;
    #[cfg(feature = "runtime-benchmarks")]
    type XcmExecutor = primitives::MockXcmExecutor;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type XcmExecutor = XcmExecutor<XcmConfig>;
    type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
    type BaseXcmWeight = BaseXcmWeight;
    type MaxAssetsForTransfer = MaxAssetsForTransfer;
    type MinXcmFee = ParachainMinFee;
    type LocationsFilter = Everything;
    type ReserveProvider = RelativeReserveProvider;
    type RateLimiter = ();
    type RateLimiterId = ();
}

impl orml_unknown_tokens::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}
