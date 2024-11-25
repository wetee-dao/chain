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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

use frame_support::{
    dispatch::DispatchResult,
    ensure,
    pallet_prelude::*,
    sp_runtime::SaturatedConversion,
    traits::{
        Currency as PalletCurrency, ExistenceRequirement, Get,
        LockableCurrency as PalletLockableCurrency, ReservableCurrency as PalletReservableCurrency,
        WithdrawReasons,
    },
};
use frame_system::{ensure_signed, pallet_prelude::*};
use orml_traits::{
    arithmetic::{Signed, SimpleArithmetic},
    BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency,
    BasicReservableCurrency, LockIdentifier, MultiCurrency, MultiCurrencyExtended,
    MultiLockableCurrency, MultiReservableCurrency,
};
use parity_scale_codec::{Codec, Decode, Encode, MaxEncodedLen};
use scale_info::prelude::vec::Vec;
use scale_info::TypeInfo;

use sp_runtime::{
    traits::{CheckedSub, MaybeSerializeDeserialize, StaticLookup, Zero},
    RuntimeDebug,
};
use sp_std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    marker, result,
};
use wetee_primitives::types::WeAssetId;

pub mod asset_adaper_in_pallet;
mod asset_in_pallet;
mod impl_currency_handler;
mod impl_multi_currency;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;

mod traits;
use traits::CurrenciesHandler;

pub const NATIVE_ASSET_ID: WeAssetId = 0;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct AssetMeta {
    /// project name
    /// token 名
    pub name: Vec<u8>,
    /// The ticker symbol for this asset.
    /// 通证符号
    pub symbol: Vec<u8>,
    /// The number of decimals this asset uses to represent one unit.
    /// 资产小数点位数
    pub decimals: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct AssetInfo<AccountId, AssetMeta> {
    pub owner: AccountId,
    pub metadata: AssetMeta,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    pub(crate) type BalanceOf<T> = <<T as Config>::MultiAsset as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    pub(crate) type AmountOf<T> = <<T as Config>::MultiAsset as MultiCurrencyExtended<
        <T as frame_system::Config>::AccountId,
    >>::Amount;

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_dao::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// we asset
        /// 组织内部资产
        type MultiAsset: MultiCurrency<Self::AccountId, CurrencyId = WeAssetId>
            + MultiCurrencyExtended<Self::AccountId>
            + MultiLockableCurrency<Self::AccountId>
            + MultiReservableCurrency<Self::AccountId>;

        /// we naive token
        /// 链上原生通证
        type NativeAsset: BasicCurrencyExtended<
                Self::AccountId,
                Balance = BalanceOf<Self>,
                Amount = AmountOf<Self>,
            > + BasicLockableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
            + BasicReservableCurrency<Self::AccountId, Balance = BalanceOf<Self>>;

        /// Weight information for extrinsics in this pallet.
        /// 链上 weight
        type WeightInfo: WeightInfo;

        /// Maximum assets that can be created
        /// 最多可创建组织数量
        type MaxCreatableId: Get<WeAssetId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        AmountIntoBalanceFailed,
        BalanceTooLow,
        AssetIdOverflow,
        AssetAlreadyExists,
        AssetNotExists,
        MetadataNotChange,
        MetadataErr,
        NotOwner,
        ShouldNotChangeDecimals,
        MetadataNotExists,
        NativeCurrency,
        CurrencyIdTooLarge,
        CurrencyIdTooLow,
        CexTransferClosed,
        AssetIdExisted,
        DepositTooLow,
        DepositNotZero,
        DepositRateError,
        BadWeOrigin,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Currency transfer success. [asset_id, from, to, amount]
        Transferred(WeAssetId, T::AccountId, T::AccountId, BalanceOf<T>),
        /// Update balance success. [asset_id, who, amount]
        BalanceUpdated(WeAssetId, T::AccountId, AmountOf<T>),
        /// Deposit success. [asset_id, who, amount]
        Deposited(WeAssetId, T::AccountId, BalanceOf<T>),
        /// Withdraw success. [asset_id, who, amount]
        Withdrawn(WeAssetId, T::AccountId, BalanceOf<T>),
        /// Create asset success. [asset_id, metadata]
        CreateAsset(T::AccountId, WeAssetId, BalanceOf<T>),
        /// Update metadata success. [asset_id, metadata]
        SetMetadata(T::AccountId, WeAssetId, AssetMeta),
        /// Burn success. [asset_id, who, amount]
        Burn(T::AccountId, WeAssetId, BalanceOf<T>),
        /// Set weight rate success. [asset_id, multiple]
        SetWeightRateMultiple { asset_id: WeAssetId, multiple: u128 },
        /// Set existenial deposit success. [asset_id, existenial_deposit]
        SetExistenialDepposit {
            asset_id: WeAssetId,
            existenial_deposit: BalanceOf<T>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn asset_info)]
    pub type AssetsInfo<T: Config> =
        StorageMap<_, Blake2_128Concat, WeAssetId, AssetInfo<T::AccountId, AssetMeta>>;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create we asset.
        /// 创建 WETEE 资产
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_asset())]
        pub fn create_asset(
            origin: OriginFor<T>,
            metadata: AssetMeta,
            init_amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            // 确认用户是否是组织创建者
            let who = ensure_signed(origin)?;
            let asset_id = wetee_dao::NextDaoId::<T>::get();

            // 记录下一个 DAO id
            let next_id = asset_id.checked_add(1).ok_or(Error::<T>::AssetIdOverflow)?;
            wetee_dao::NextDaoId::<T>::put(next_id);

            // 创建资产
            Self::try_create(who.clone(), asset_id, metadata, init_amount)?;

            Ok(().into())
        }

        /// You should have created the asset first.
        /// 设置资产元数据
        #[pallet::call_index(004)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_metadata())]
        pub fn set_metadata(
            origin: OriginFor<T>,
            asset_id: WeAssetId,
            metadata: AssetMeta,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(
                metadata.name.len() > 2
                    && metadata.symbol.len() > 1
                    && metadata.decimals > 0u8
                    && metadata.decimals < 19,
                Error::<T>::MetadataErr
            );

            let mut asset_info =
                AssetsInfo::<T>::get(asset_id).ok_or(Error::<T>::AssetNotExists)?;

            ensure!(
                asset_info.metadata != metadata,
                Error::<T>::MetadataNotChange
            );
            ensure!(
                asset_info.metadata.decimals == metadata.decimals,
                Error::<T>::ShouldNotChangeDecimals
            );

            // 确认用户是否是创建者
            ensure!(who == asset_info.owner, Error::<T>::ShouldNotChangeDecimals);

            asset_info.metadata = metadata.clone();

            AssetsInfo::<T>::insert(asset_id, asset_info);
            Self::deposit_event(Event::SetMetadata(who, asset_id, metadata));

            Ok(().into())
        }

        /// Users destroy their own assets.
        /// 销毁资产
        #[pallet::call_index(005)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            asset_id: WeAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(Self::is_exists(asset_id), Error::<T>::AssetNotExists);
            let user = ensure_signed(origin)?;

            <Self as MultiCurrency<T::AccountId>>::withdraw(asset_id, &user, amount)?;
            Self::deposit_event(Event::Burn(user, asset_id, amount));
            Ok(().into())
        }

        /// This function transfers the given amount from the source to the destination.
        ///
        /// # Arguments
        ///
        /// * `amount` - The amount to transfer
        /// * `source` - The source account
        /// * `destination` - The destination account
        /// 转移资产
        #[pallet::call_index(006)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            asset_id: WeAssetId,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(dest)?;
            ensure!(Self::is_exists(asset_id), Error::<T>::AssetNotExists);

            <Self as MultiCurrency<T::AccountId>>::transfer(asset_id, &from, &to, amount)?;
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取账户金额
        pub fn get_balance(
            asset_id: WeAssetId,
            who: T::AccountId,
        ) -> result::Result<BalanceOf<T>, DispatchError> {
            let balance = <Self as MultiCurrency<T::AccountId>>::total_balance(asset_id, &who);
            Ok(balance)
        }

        // 设置账户金额
        pub fn set_balance(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::deposit(asset_id, &who, value)?;
            Ok(())
        }

        /// 为...锁定保证金
        pub fn reserve(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::reserve(asset_id, &who, value)?;
            Ok(())
        }

        /// 解除保证
        pub fn unreserve(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::unreserve(asset_id, &who, value);
            Ok(())
        }

        /// 尽可能解除保证
        pub fn slash_reserved(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> BalanceOf<T> {
            <Self as MultiReservableCurrency<T::AccountId>>::slash_reserved(asset_id, &who, value)
        }

        /// 总发行量
        pub fn total_issuance(asset_id: WeAssetId) -> BalanceOf<T> {
            <Self as MultiCurrency<T::AccountId>>::total_issuance(asset_id)
        }

        pub fn try_create(
            user: T::AccountId,
            asset_id: WeAssetId,
            metadata: AssetMeta,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            Self::do_create(user, asset_id, metadata, amount)
        }

        /// 转帐
        pub fn try_transfer(
            asset_id: WeAssetId,
            from: T::AccountId,
            to: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::transfer(asset_id, &from, &to, value)?;
            Ok(())
        }

        /// 销毁资产
        pub fn try_burn(
            asset_id: WeAssetId,
            from: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::withdraw(asset_id, &from, value)?;
            Ok(())
        }

        /// 转帐
        pub fn burn_with_number(
            asset_id: WeAssetId,
            from: T::AccountId,
            value: u128,
        ) -> result::Result<(), DispatchError> {
            let amount: BalanceOf<T> = value.saturated_into::<BalanceOf<T>>();

            <Self as MultiCurrency<T::AccountId>>::withdraw(asset_id, &from, amount)?;
            Ok(())
        }

        // 产生 TOKEN
        pub fn try_deposit(
            asset_id: WeAssetId,
            dest: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            // 确认组织是否存在
            ensure!(Self::is_exists(asset_id), Error::<T>::AssetNotExists);

            // 产生 Token
            <Self as MultiCurrency<T::AccountId>>::deposit(asset_id, &dest, value)?;

            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// 判断资产是否存在
    pub fn is_exists(asset_id: WeAssetId) -> bool {
        if asset_id == NATIVE_ASSET_ID {
            return true;
        }
        if AssetsInfo::<T>::get(asset_id).is_some() {
            return true;
        }
        false
    }

    /// 判断资产ID是否太大
    pub fn is_asset_id_too_large(asset_id: WeAssetId) -> bool {
        if asset_id >= T::MaxCreatableId::get() {
            return true;
        }
        false
    }
}
