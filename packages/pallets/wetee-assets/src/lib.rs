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

use codec::{Codec, Decode, Encode, MaxEncodedLen};
use frame_support::{
    dispatch::DispatchResult,
    ensure,
    pallet_prelude::*,
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
use wetee_org::{self as dao};
use wetee_primitives::types::DaoAssetId;

pub mod asset_adaper_in_pallet;
mod asset_in_pallet;
mod impl_currency_handler;
mod impl_multi_currency;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod weights;
pub use weights::WeightInfo;

mod traits;
use traits::CurrenciesHandler;

pub const NATIVE_ASSET_ID: DaoAssetId = 0;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoAssetMeta {
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
pub struct DaoAssetInfo<AccountId, DaoAssetMeta> {
    pub owner: AccountId,
    pub metadata: DaoAssetMeta,
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
    pub trait Config: frame_system::Config + dao::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// dao asset
        /// 组织内部资产
        type MultiAsset: MultiCurrency<Self::AccountId, CurrencyId = DaoAssetId>
            + MultiCurrencyExtended<Self::AccountId>
            + MultiLockableCurrency<Self::AccountId>
            + MultiReservableCurrency<Self::AccountId>;

        /// dao naive token
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
        type MaxCreatableId: Get<DaoAssetId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        AmountIntoBalanceFailed,
        BalanceTooLow,
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
        DaoExists,
        CexTransferClosed,
        AssetIdExisted,
        DepositTooLow,
        DepositNotZero,
        DepositRateError,
        BadDaoOrigin,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Currency transfer success. [dao_id, from, to, amount]
        Transferred(DaoAssetId, T::AccountId, T::AccountId, BalanceOf<T>),
        /// Update balance success. [dao_id, who, amount]
        BalanceUpdated(DaoAssetId, T::AccountId, AmountOf<T>),
        /// Deposit success. [dao_id, who, amount]
        Deposited(DaoAssetId, T::AccountId, BalanceOf<T>),
        /// Withdraw success. [dao_id, who, amount]
        Withdrawn(DaoAssetId, T::AccountId, BalanceOf<T>),
        /// Create asset success. [dao_id, metadata]
        CreateAsset(T::AccountId, DaoAssetId, BalanceOf<T>),
        /// Update metadata success. [dao_id, metadata]
        SetMetadata(T::AccountId, DaoAssetId, DaoAssetMeta),
        /// Burn success. [dao_id, who, amount]
        Burn(T::AccountId, DaoAssetId, BalanceOf<T>),
        /// Set weight rate success. [dao_id, multiple]
        SetWeightRateMultiple { dao_id: DaoAssetId, multiple: u128 },
        /// Set existenial deposit success. [dao_id, existenial_deposit]
        SetExistenialDepposit {
            dao_id: DaoAssetId,
            existenial_deposit: BalanceOf<T>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn asset_info)]
    pub type DaoAssetsInfo<T: Config> =
        StorageMap<_, Blake2_128Concat, DaoAssetId, DaoAssetInfo<T::AccountId, DaoAssetMeta>>;

    #[pallet::storage]
    #[pallet::getter(fn users_number)]
    pub type UsersNumber<T: Config> = StorageMap<_, Identity, DaoAssetId, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn existenial_deposits)]
    pub type ExistentDeposits<T: Config> =
        StorageMap<_, Identity, DaoAssetId, BalanceOf<T>, ValueQuery>;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create dao asset.
        /// 创建 WETEE 资产
        #[pallet::call_index(001)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_asset())]
        pub fn create_asset(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            metadata: DaoAssetMeta,
            amount: BalanceOf<T>,
            init_dao_asset: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                wetee_org::Daos::<T>::contains_key(dao_id),
                Error::<T>::AssetNotExists
            );

            let user = ensure_signed(origin)?;
            wetee_org::Pallet::<T>::ensrue_dao_creator(user.clone(), dao_id)?;

            Self::do_create(user.clone(), dao_id, metadata, init_dao_asset, false)?;

            // 将资金转入资金池B池
            <Self as MultiCurrency<T::AccountId>>::transfer(
                NATIVE_ASSET_ID,
                &user,
                &wetee_org::Pallet::<T>::dao_account(dao_id),
                amount,
            )?;

            // 初始化账户基本资产
            <Self as MultiCurrency<T::AccountId>>::deposit(
                dao_id,
                &wetee_org::Pallet::<T>::dao_account(dao_id),
                init_dao_asset,
            )?;

            Ok(().into())
        }

        /// 设置加入WETEE所需要的最小抵押                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           
        #[pallet::call_index(003)]
        #[pallet::weight(T::DbWeight::get().reads_writes(4, 1) + Weight::from_all(40_000))]
        pub fn set_existenial_deposit(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            existenial_deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(who)?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            ExistentDeposits::<T>::insert(dao_id, existenial_deposit);
            Self::deposit_event(Event::SetExistenialDepposit {
                dao_id,
                existenial_deposit,
            });

            Ok(().into())
        }

        /// You should have created the asset first.
        /// 设置资产元数据
        #[pallet::call_index(004)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_metadata())]
        pub fn set_metadata(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            metadata: DaoAssetMeta,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(who.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            ensure!(
                wetee_org::Daos::<T>::contains_key(dao_id),
                Error::<T>::AssetNotExists
            );

            ensure!(
                metadata.name.len() > 2
                    && metadata.symbol.len() > 1
                    && metadata.decimals > 0u8
                    && metadata.decimals < 19,
                Error::<T>::MetadataErr
            );

            let mut asset_info =
                DaoAssetsInfo::<T>::get(dao_id).ok_or(Error::<T>::AssetNotExists)?;

            ensure!(
                asset_info.metadata != metadata,
                Error::<T>::MetadataNotChange
            );
            ensure!(
                asset_info.metadata.decimals == metadata.decimals,
                Error::<T>::ShouldNotChangeDecimals
            );

            asset_info.metadata = metadata.clone();

            DaoAssetsInfo::<T>::insert(dao_id, asset_info);
            Self::deposit_event(Event::SetMetadata(who, dao_id, metadata));

            Ok(().into())
        }

        /// Users destroy their own assets.
        /// 销毁资产
        #[pallet::call_index(005)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                wetee_org::Daos::<T>::contains_key(dao_id),
                Error::<T>::AssetNotExists
            );
            let user = ensure_signed(origin)?;
            // wetee_org::Pallet::<T>::ensrue_gov_approve_account(user.clone(), dao_id)?;

            ensure!(
                Self::is_exists_metadata(dao_id),
                Error::<T>::MetadataNotExists
            );

            <Self as MultiCurrency<T::AccountId>>::withdraw(dao_id, &user, amount)?;
            Self::deposit_event(Event::Burn(user, dao_id, amount));
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
            dao_id: DaoAssetId,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(dest)?;
            ensure!(
                wetee_org::Daos::<T>::contains_key(dao_id),
                Error::<T>::AssetNotExists
            );

            // 从WETEE转出手续费 TODO
            // match wetee_org::Daos::<T>::get(dao_id) {
            //     Some(dao) => {
            //         let _dao_account = dao.dao_account_id;
            //     }
            // };

            if let Some(dao) = wetee_org::Daos::<T>::get(dao_id) {
                let _dao_account = dao.dao_account_id;
            }

            ensure!(
                Self::is_exists_metadata(dao_id),
                Error::<T>::MetadataNotExists
            );

            <Self as MultiCurrency<T::AccountId>>::transfer(dao_id, &from, &to, amount)?;
            Ok(().into())
        }

        /// 成为会员
        #[pallet::call_index(007)]
        #[pallet::weight(T::DbWeight::get().reads_writes(4, 4) + Weight::from_all(40_000))]
        pub fn join(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            share_expect: u32,
            #[pallet::compact] existenial_deposit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let daogov = wetee_org::Pallet::<T>::ensrue_gov_approve_account(who.clone())?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoOrigin);

            // 最低押金必须大于0
            ensure!(
                existenial_deposit >= 0u32.into(),
                Error::<T>::DepositNotZero
            );

            // 获取链上资金池
            let treasury = wetee_org::Pallet::<T>::dao_treasury(dao_id);
            let treasury_total =
                <Self as MultiCurrency<T::AccountId>>::total_balance(NATIVE_ASSET_ID, &treasury);
            ensure!(treasury_total > 0u32.into(), Error::<T>::DepositTooLow);

            // 判断用户期望share是否符合当前汇率
            let share_expect_b: BalanceOf<T> = share_expect.into();
            ensure!(
                <Self as MultiCurrency<T::AccountId>>::total_issuance(dao_id) / treasury_total
                    >= share_expect_b / existenial_deposit,
                Error::<T>::DepositRateError
            );

            // 将资金转入资金池B池
            <Self as MultiCurrency<T::AccountId>>::transfer(
                NATIVE_ASSET_ID,
                &who,
                &treasury,
                existenial_deposit,
            )?;

            // 设置为会员，并且为用户添加 share
            wetee_org::Pallet::<T>::try_add_member(dao_id, who.clone())?;
            <Self as MultiCurrency<T::AccountId>>::deposit(dao_id, &who, share_expect.into())?;

            // 往国库产生 mint
            <Self as MultiCurrency<T::AccountId>>::deposit(dao_id, &treasury, share_expect.into())?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取账户金额
        pub fn get_balance(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<BalanceOf<T>, DispatchError> {
            let balance = <Self as MultiCurrency<T::AccountId>>::total_balance(dao_id, &who);
            Ok(balance)
        }

        /// 为...锁定保证金
        pub fn reserve(
            dao_id: DaoAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::reserve(dao_id, &who, value)?;
            Ok(())
        }

        /// 解除保证
        pub fn unreserve(
            dao_id: DaoAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::unreserve(dao_id, &who, value);
            Ok(())
        }

        /// 尽可能解除保证
        pub fn slash_reserved(
            dao_id: DaoAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> BalanceOf<T> {
            <Self as MultiReservableCurrency<T::AccountId>>::slash_reserved(dao_id, &who, value)
        }

        /// 总发行量
        pub fn total_issuance(dao_id: DaoAssetId) -> BalanceOf<T> {
            <Self as MultiCurrency<T::AccountId>>::total_issuance(dao_id)
        }

        /// 转帐
        pub fn try_transfer(
            dao_id: DaoAssetId,
            from: T::AccountId,
            to: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::transfer(dao_id, &from, &to, value)?;
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// 判断资产是否存在
    fn is_exists_metadata(dao_id: DaoAssetId) -> bool {
        if dao_id == NATIVE_ASSET_ID {
            return true;
        }
        if DaoAssetsInfo::<T>::get(dao_id).is_some() {
            return true;
        }
        false
    }

    /// 判断资产ID是否太大
    fn is_asset_id_too_large(dao_id: DaoAssetId) -> bool {
        if dao_id >= T::MaxCreatableId::get() {
            return true;
        }
        false
    }
}
