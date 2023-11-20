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

pub struct Asset<T, GetCurrencyId>(marker::PhantomData<T>, marker::PhantomData<GetCurrencyId>);

impl<T, GetCurrencyId> BasicCurrency<T::AccountId> for Asset<T, GetCurrencyId>
where
    T: Config,
    GetCurrencyId: Get<DaoAssetId>,
{
    type Balance = BalanceOf<T>;

    fn minimum_balance() -> Self::Balance {
        <Pallet<T>>::minimum_balance(GetCurrencyId::get())
    }

    fn total_issuance() -> Self::Balance {
        <Pallet<T>>::total_issuance(GetCurrencyId::get())
    }

    fn total_balance(who: &T::AccountId) -> Self::Balance {
        <Pallet<T>>::total_balance(GetCurrencyId::get(), who)
    }

    fn free_balance(who: &T::AccountId) -> Self::Balance {
        <Pallet<T>>::free_balance(GetCurrencyId::get(), who)
    }

    fn ensure_can_withdraw(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
        <Pallet<T>>::ensure_can_withdraw(GetCurrencyId::get(), who, amount)
    }

    fn transfer(from: &T::AccountId, to: &T::AccountId, amount: Self::Balance) -> DispatchResult {
        <Pallet<T> as MultiCurrency<T::AccountId>>::transfer(GetCurrencyId::get(), from, to, amount)
    }

    fn deposit(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
        <Pallet<T>>::deposit(GetCurrencyId::get(), who, amount)
    }

    fn withdraw(who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
        <Pallet<T>>::withdraw(GetCurrencyId::get(), who, amount)
    }

    fn can_slash(who: &T::AccountId, amount: Self::Balance) -> bool {
        <Pallet<T>>::can_slash(GetCurrencyId::get(), who, amount)
    }

    fn slash(who: &T::AccountId, amount: Self::Balance) -> Self::Balance {
        <Pallet<T>>::slash(GetCurrencyId::get(), who, amount)
    }
}

impl<T, GetCurrencyId> BasicCurrencyExtended<T::AccountId> for Asset<T, GetCurrencyId>
where
    T: Config,
    GetCurrencyId: Get<DaoAssetId>,
{
    type Amount = AmountOf<T>;

    fn update_balance(who: &T::AccountId, by_amount: Self::Amount) -> DispatchResult {
        <Pallet<T> as MultiCurrencyExtended<T::AccountId>>::update_balance(
            GetCurrencyId::get(),
            who,
            by_amount,
        )
    }
}

impl<T, GetCurrencyId> BasicLockableCurrency<T::AccountId> for Asset<T, GetCurrencyId>
where
    T: Config,
    GetCurrencyId: Get<DaoAssetId>,
{
    type Moment = BlockNumberFor<T>;

    fn set_lock(
        lock_id: LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        <Pallet<T> as MultiLockableCurrency<T::AccountId>>::set_lock(
            lock_id,
            GetCurrencyId::get(),
            who,
            amount,
        )
    }

    fn extend_lock(
        lock_id: LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        <Pallet<T> as MultiLockableCurrency<T::AccountId>>::extend_lock(
            lock_id,
            GetCurrencyId::get(),
            who,
            amount,
        )
    }

    fn remove_lock(lock_id: LockIdentifier, who: &T::AccountId) -> DispatchResult {
        <Pallet<T> as MultiLockableCurrency<T::AccountId>>::remove_lock(
            lock_id,
            GetCurrencyId::get(),
            who,
        )
    }
}

impl<T, GetCurrencyId> BasicReservableCurrency<T::AccountId> for Asset<T, GetCurrencyId>
where
    T: Config,
    GetCurrencyId: Get<DaoAssetId>,
{
    fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::can_reserve(
            GetCurrencyId::get(),
            who,
            value,
        )
    }

    fn slash_reserved(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::slash_reserved(
            GetCurrencyId::get(),
            who,
            value,
        )
    }

    fn reserved_balance(who: &T::AccountId) -> Self::Balance {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::reserved_balance(
            GetCurrencyId::get(),
            who,
        )
    }

    fn reserve(who: &T::AccountId, value: Self::Balance) -> DispatchResult {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::reserve(
            GetCurrencyId::get(),
            who,
            value,
        )
    }

    fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::unreserve(
            GetCurrencyId::get(),
            who,
            value,
        )
    }

    fn repatriate_reserved(
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError> {
        <Pallet<T> as MultiReservableCurrency<T::AccountId>>::repatriate_reserved(
            GetCurrencyId::get(),
            slashed,
            beneficiary,
            value,
            status,
        )
    }
}
