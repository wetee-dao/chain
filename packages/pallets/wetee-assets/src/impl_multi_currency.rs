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

impl<T: Config> MultiCurrency<T::AccountId> for Pallet<T> {
    type CurrencyId = DaoAssetId;
    type Balance = BalanceOf<T>;

    fn minimum_balance(asset_id: Self::CurrencyId) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            <T as pallet::Config>::NativeAsset::minimum_balance()
        } else {
            <T as pallet::Config>::MultiAsset::minimum_balance(asset_id)
        }
    }

    fn total_issuance(asset_id: Self::CurrencyId) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::total_issuance()
        } else {
            <T as pallet::Config>::MultiAsset::total_issuance(asset_id)
        }
    }

    fn total_balance(asset_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::total_balance(who)
        } else {
            <T as pallet::Config>::MultiAsset::total_balance(asset_id, who)
        }
    }

    fn free_balance(asset_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::free_balance(who)
        } else {
            <T as pallet::Config>::MultiAsset::free_balance(asset_id, who)
        }
    }

    fn ensure_can_withdraw(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::ensure_can_withdraw(who, amount)
        } else {
            <T as pallet::Config>::MultiAsset::ensure_can_withdraw(asset_id, who, amount)
        }
    }

    fn transfer(
        asset_id: Self::CurrencyId,
        from: &T::AccountId,
        to: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        #[cfg(test)]
        println!(
            "\nTransfer =>> Asset_id:{:?} ||| Free_amount: {:?}",
            asset_id, amount
        );

        if amount.is_zero() || from == to {
            return Ok(());
        }

        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::transfer(from, to, amount)?;
        } else {
            <T as pallet::Config>::MultiAsset::transfer(asset_id, from, to, amount)?;
        }

        Self::deposit_event(Event::Transferred(
            asset_id,
            from.clone(),
            to.clone(),
            amount,
        ));
        Ok(())
    }

    fn deposit(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        if amount.is_zero() {
            return Ok(());
        }

        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::deposit(who, amount)?;
        } else {
            <T as pallet::Config>::MultiAsset::deposit(asset_id, who, amount)?;
        }

        Self::deposit_event(Event::Deposited(asset_id, who.clone(), amount));
        Ok(())
    }

    fn withdraw(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        if amount.is_zero() {
            return Ok(());
        }
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::withdraw(who, amount)?;
        } else {
            <T as pallet::Config>::MultiAsset::withdraw(asset_id, who, amount)?;
        }
        Self::deposit_event(Event::Withdrawn(asset_id, who.clone(), amount));
        Ok(())
    }

    fn can_slash(asset_id: Self::CurrencyId, who: &T::AccountId, amount: Self::Balance) -> bool {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::can_slash(who, amount)
        } else {
            <T as pallet::Config>::MultiAsset::can_slash(asset_id, who, amount)
        }
    }

    fn slash(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::slash(who, amount)
        } else {
            <T as pallet::Config>::MultiAsset::slash(asset_id, who, amount)
        }
    }
}

impl<T: Config> MultiCurrencyExtended<T::AccountId> for Pallet<T> {
    type Amount = AmountOf<T>;

    fn update_balance(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        by_amount: Self::Amount,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::update_balance(who, by_amount)?;
        } else {
            <T as pallet::Config>::MultiAsset::update_balance(asset_id, who, by_amount)?;
        }
        Self::deposit_event(Event::BalanceUpdated(asset_id, who.clone(), by_amount));
        Ok(())
    }
}

impl<T: Config> MultiLockableCurrency<T::AccountId> for Pallet<T> {
    type Moment = BlockNumberFor<T>;

    fn set_lock(
        lock_id: LockIdentifier,
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::set_lock(lock_id, who, amount)
        } else {
            <T as pallet::Config>::MultiAsset::set_lock(lock_id, asset_id, who, amount)
        }
    }

    fn extend_lock(
        lock_id: LockIdentifier,
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::extend_lock(lock_id, who, amount)
        } else {
            <T as pallet::Config>::MultiAsset::extend_lock(lock_id, asset_id, who, amount)
        }
    }

    fn remove_lock(
        lock_id: LockIdentifier,
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::remove_lock(lock_id, who)
        } else {
            <T as pallet::Config>::MultiAsset::remove_lock(lock_id, asset_id, who)
        }
    }
}

impl<T: Config> MultiReservableCurrency<T::AccountId> for Pallet<T> {
    fn can_reserve(asset_id: Self::CurrencyId, who: &T::AccountId, value: Self::Balance) -> bool {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::can_reserve(who, value)
        } else {
            <T as pallet::Config>::MultiAsset::can_reserve(asset_id, who, value)
        }
    }

    fn slash_reserved(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::slash_reserved(who, value)
        } else {
            <T as pallet::Config>::MultiAsset::slash_reserved(asset_id, who, value)
        }
    }

    fn reserved_balance(asset_id: Self::CurrencyId, who: &T::AccountId) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::reserved_balance(who)
        } else {
            <T as pallet::Config>::MultiAsset::reserved_balance(asset_id, who)
        }
    }

    fn reserve(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> DispatchResult {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::reserve(who, value)
        } else {
            <T as pallet::Config>::MultiAsset::reserve(asset_id, who, value)
        }
    }

    fn unreserve(
        asset_id: Self::CurrencyId,
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Self::Balance {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::unreserve(who, value)
        } else {
            <T as pallet::Config>::MultiAsset::unreserve(asset_id, who, value)
        }
    }

    fn repatriate_reserved(
        asset_id: Self::CurrencyId,
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError> {
        if asset_id == NATIVE_ASSET_ID {
            T::NativeAsset::repatriate_reserved(slashed, beneficiary, value, status)
        } else {
            <T as pallet::Config>::MultiAsset::repatriate_reserved(
                asset_id,
                slashed,
                beneficiary,
                value,
                status,
            )
        }
    }
}
