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

use super::*;

impl<T: Config>
    CurrenciesHandler<
        DaoAssetId,
        DaoAssetMeta,
        DispatchError,
        T::AccountId,
        BalanceOf<T>,
        DispatchResult,
    > for Pallet<T>
{
    fn get_metadata(asset_id: DaoAssetId) -> result::Result<DaoAssetMeta, DispatchError> {
        let asset_info_opt = DaoAssetsInfo::<T>::get(asset_id);
        let asset_info = match asset_info_opt {
            Some(x) => x,
            _ => {
                if cfg!(any(feature = "std", feature = "runtime-benchmarks", test)) {
                    return Ok(DaoAssetMeta {
                        name: [].into(),
                        symbol: [].into(),
                        decimals: 12,
                    });
                } else {
                    return Err(Error::<T>::AssetNotExists)?;
                }
            }
        };
        Ok(asset_info.metadata)
    }

    fn do_create(
        user: T::AccountId,
        asset_id: DaoAssetId,
        metadata: DaoAssetMeta,
        amount: BalanceOf<T>,
        _is_swap_deposit: bool,
    ) -> DispatchResult {
        ensure!(
            !Self::is_exists_metadata(asset_id)
                && <T as pallet::Config>::MultiAsset::total_issuance(asset_id)
                    == BalanceOf::<T>::from(0u32),
            Error::<T>::AssetAlreadyExists
        );

        ensure!(
            !Self::is_asset_id_too_large(asset_id),
            Error::<T>::CurrencyIdTooLarge
        );

        #[cfg(test)]
        println!(
            "\n初始化 TOKEN 池 =>> Asset_id:{:?} ||| Free_amount: {:?}",
            asset_id, amount
        );

        <T as pallet::Config>::MultiAsset::deposit(asset_id, &user, amount)?;

        DaoAssetsInfo::<T>::insert(
            asset_id,
            DaoAssetInfo {
                owner: user.clone(),
                metadata,
            },
        );
        Self::deposit_event(Event::CreateAsset(user, asset_id, amount));

        Ok(())
    }
}
