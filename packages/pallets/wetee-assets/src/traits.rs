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

use sp_std::result;

pub trait CurrenciesHandler<
    CurrencyId,
    DicoAssetMetadata,
    DispatchErr,
    AccountId,
    Balance,
    DispatchResult,
>
{
    fn get_metadata(currency: CurrencyId) -> result::Result<DicoAssetMetadata, DispatchErr>;
    fn do_create(
        user: AccountId,
        currency_id: CurrencyId,
        metadata: DicoAssetMetadata,
        amount: Balance,
        is_swap_deposit: bool,
    ) -> DispatchResult;
}

pub trait AssetIdMapping<CurrencyId, MultiLocation> {
    fn get_multi_location(asset_id: CurrencyId) -> Option<MultiLocation>;
    fn get_currency_id(multi_location: MultiLocation) -> Option<CurrencyId>;
    fn get_weight_rate_multiple(location: MultiLocation) -> Option<u128>;
}
