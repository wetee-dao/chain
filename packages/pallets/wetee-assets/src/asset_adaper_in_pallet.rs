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

/// Adapt other currency traits implementation to `BasicCurrency`.
pub struct BasicCurrencyAdapter<T, Currency, Amount, Moment>(
    marker::PhantomData<(T, Currency, Amount, Moment)>,
);

type PalletBalanceOf<A, Currency> = <Currency as PalletCurrency<A>>::Balance;

// Adapt `frame_support::traits::Currency`
impl<T, AccountId, Currency, Amount, Moment> BasicCurrency<AccountId>
    for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
    Currency: PalletCurrency<AccountId>,
    T: Config,
{
    type Balance = PalletBalanceOf<AccountId, Currency>;

    fn minimum_balance() -> Self::Balance {
        Currency::minimum_balance()
    }

    fn total_issuance() -> Self::Balance {
        Currency::total_issuance()
    }

    fn total_balance(who: &AccountId) -> Self::Balance {
        Currency::total_balance(who)
    }

    fn free_balance(who: &AccountId) -> Self::Balance {
        Currency::free_balance(who)
    }

    fn ensure_can_withdraw(who: &AccountId, amount: Self::Balance) -> DispatchResult {
        let new_balance = Self::free_balance(who)
            .checked_sub(&amount)
            .ok_or(Error::<T>::BalanceTooLow)?;

        Currency::ensure_can_withdraw(who, amount, WithdrawReasons::all(), new_balance)
    }

    fn transfer(from: &AccountId, to: &AccountId, amount: Self::Balance) -> DispatchResult {
        Currency::transfer(from, to, amount, ExistenceRequirement::AllowDeath)
    }

    fn deposit(who: &AccountId, amount: Self::Balance) -> DispatchResult {
        let _ = Currency::deposit_creating(who, amount);
        Ok(())
    }

    fn withdraw(who: &AccountId, amount: Self::Balance) -> DispatchResult {
        Currency::withdraw(
            who,
            amount,
            WithdrawReasons::all(),
            ExistenceRequirement::AllowDeath,
        )
        .map(|_| ())
    }

    fn can_slash(who: &AccountId, amount: Self::Balance) -> bool {
        Currency::can_slash(who, amount)
    }

    fn slash(who: &AccountId, amount: Self::Balance) -> Self::Balance {
        let (_, gap) = Currency::slash(who, amount);
        gap
    }
}

// Adapt `frame_support::traits::Currency`
impl<T, AccountId, Currency, Amount, Moment> BasicCurrencyExtended<AccountId>
    for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
    Amount: Signed
        + TryInto<PalletBalanceOf<AccountId, Currency>>
        + TryFrom<PalletBalanceOf<AccountId, Currency>>
        + SimpleArithmetic
        + Codec
        + Copy
        + MaybeSerializeDeserialize
        + MaxEncodedLen
        + Debug
        + Default,
    Currency: PalletCurrency<AccountId>,
    T: Config,
{
    type Amount = Amount;

    fn update_balance(who: &AccountId, by_amount: Self::Amount) -> DispatchResult {
        let by_balance = by_amount
            .abs()
            .try_into()
            .map_err(|_| Error::<T>::AmountIntoBalanceFailed)?;
        if by_amount.is_positive() {
            Self::deposit(who, by_balance)
        } else {
            Self::withdraw(who, by_balance)
        }
    }
}

// Adapt `frame_support::traits::LockableCurrency`
impl<T, AccountId, Currency, Amount, Moment> BasicLockableCurrency<AccountId>
    for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
    Currency: PalletLockableCurrency<AccountId>,
    T: Config,
{
    type Moment = Moment;

    fn set_lock(lock_id: LockIdentifier, who: &AccountId, amount: Self::Balance) -> DispatchResult {
        Currency::set_lock(lock_id, who, amount, WithdrawReasons::all());
        Ok(())
    }

    fn extend_lock(
        lock_id: LockIdentifier,
        who: &AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        Currency::extend_lock(lock_id, who, amount, WithdrawReasons::all());
        Ok(())
    }

    fn remove_lock(lock_id: LockIdentifier, who: &AccountId) -> DispatchResult {
        Currency::remove_lock(lock_id, who);
        Ok(())
    }
}

// Adapt `frame_support::traits::ReservableCurrency`
impl<T, AccountId, Currency, Amount, Moment> BasicReservableCurrency<AccountId>
    for BasicCurrencyAdapter<T, Currency, Amount, Moment>
where
    Currency: PalletReservableCurrency<AccountId>,
    T: Config,
{
    fn can_reserve(who: &AccountId, value: Self::Balance) -> bool {
        Currency::can_reserve(who, value)
    }

    fn slash_reserved(who: &AccountId, value: Self::Balance) -> Self::Balance {
        let (_, gap) = Currency::slash_reserved(who, value);
        gap
    }

    fn reserved_balance(who: &AccountId) -> Self::Balance {
        Currency::reserved_balance(who)
    }

    fn reserve(who: &AccountId, value: Self::Balance) -> DispatchResult {
        Currency::reserve(who, value)
    }

    fn unreserve(who: &AccountId, value: Self::Balance) -> Self::Balance {
        Currency::unreserve(who, value)
    }

    fn repatriate_reserved(
        slashed: &AccountId,
        beneficiary: &AccountId,
        value: Self::Balance,
        status: BalanceStatus,
    ) -> result::Result<Self::Balance, DispatchError> {
        Currency::repatriate_reserved(slashed, beneficiary, value, status)
    }
}
