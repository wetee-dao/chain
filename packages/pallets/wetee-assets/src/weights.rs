
// Copyright 2021-2022 LISTEN TEAM.
// This file is part of LISTEN

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Forked from https://github.com/open-web3-stack/open-runtime-module-library/tree/master/currencies.
// Most of this module uses code from the orml, but due to business differences, we made some feature additions.
// In this module, we can create asset, set metadata and burn our tokens, open cross transfer function
// and set cross-chain transfer weight for assets.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_currencies.
pub trait WeightInfo {
	fn create_asset() -> Weight;
	fn set_metadata() -> Weight;
	fn burn() -> Weight;
	fn transfer() -> Weight;
	fn transfer_native_currency() -> Weight;
	fn update_balance() -> Weight;
}

/// Weights for pallet_currencies using the Substrate node and recommended hardware.
pub struct DicoWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for DicoWeight<T> {
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn create_asset() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn update_balance() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn create_asset() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn burn() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_native_currency() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
	// Storage: Currencies DicoAssetsInfo (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn update_balance() -> Weight {
		Weight::from_parts(20_0000_0000,0)
	}
}