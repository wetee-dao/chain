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
    traits::{ExistenceRequirement, Get, LockIdentifier},
};
use frame_system::{ensure_signed, pallet_prelude::*};
use orml_traits::{
    currency::TransferAll, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
    MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency,
    NamedBasicReservableCurrency, NamedMultiReservableCurrency,
};
use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{Convert, StaticLookup},
    RuntimeDebug,
};
use sp_std::{
    convert::{TryFrom, TryInto},
    result,
};

use wetee_primitives::types::{WeAssetId, NATIVE_ASSET_ID};

pub mod ext;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod weights;
pub use weights::WeightInfo;

#[derive(
    Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo, Serialize, Deserialize,
)]
pub struct AssetMeta {
    /// project name
    /// token 名
    pub name: Vec<u8>,
    /// The ticker symbol for this asset.
    /// 通证符号
    pub symbol: Vec<u8>,
    /// The number of decimals this asset uses to represent one unit.
    /// 资产位数 defalut 12
    pub decimals: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct AssetInfo<AccountId, AssetMeta> {
    pub owner: AccountId,
    pub metadata: AssetMeta,
}

#[frame_support::pallet]
pub mod pallet {
    use sp_runtime::SaturatedConversion;

    use super::*;

    pub(crate) type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    pub(crate) type CurrencyIdOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::CurrencyId;

    pub(crate) type AmountOf<T> = <<T as Config>::MultiCurrency as MultiCurrencyExtended<
        <T as frame_system::Config>::AccountId,
    >>::Amount;

    pub(crate) type ReserveIdentifierOf<T> =
        <<T as Config>::MultiCurrency as NamedMultiReservableCurrency<
            <T as frame_system::Config>::AccountId,
        >>::ReserveIdentifier;

    #[derive(frame_support::DefaultNoBound)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub _config: sp_std::marker::PhantomData<T>,
        pub para_id: u32,
        pub init_assets: Vec<(u32, Vec<u8>, AssetMeta)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            ChainID::<T>::put(self.para_id);
            self.init_assets
                .iter()
                .for_each(|(para_id, general_key, meta)| {
                    let _ = Pallet::<T>::parachain_asset_register(
                        frame_system::RawOrigin::Root.into(),
                        para_id.clone(),
                        BoundedVec::try_from(general_key.clone()).unwrap(),
                        meta.clone(),
                    )
                    .unwrap();
                });
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config + wetee_dao::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// multi asset
        /// 组织内部资产
        type MultiCurrency: TransferAll<Self::AccountId>
            + MultiCurrencyExtended<Self::AccountId>
            + MultiLockableCurrency<Self::AccountId>
            + MultiReservableCurrency<Self::AccountId>
            + NamedMultiReservableCurrency<Self::AccountId>;

        type NativeCurrency: BasicCurrencyExtended<
                Self::AccountId,
                Balance = BalanceOf<Self>,
                Amount = AmountOf<Self>,
            > + BasicLockableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
            + BasicReservableCurrency<Self::AccountId, Balance = BalanceOf<Self>>
            + NamedBasicReservableCurrency<
                Self::AccountId,
                ReserveIdentifierOf<Self>,
                Balance = BalanceOf<Self>,
            >;

        /// Maximum assets that can be created
        /// 最多可创建组织数量
        type MaxCreatableId: Get<WeAssetId>;

        #[pallet::constant]
        type GetNativeCurrencyId: Get<CurrencyIdOf<Self>>;

        /// Convert `WeAssetId` to `T::CurrencyId`.
        type CurrencyIdConvert: Convert<WeAssetId, CurrencyIdOf<Self>>
            + Convert<CurrencyIdOf<Self>, WeAssetId>;

        /// Weight information for extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {
        AmountIntoBalanceFailed,
        BalanceTooLow,
        AssetIdOverflow,
        AssetAlreadyExists,
        ParaAssetAlreadyExists,
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
        /// Deposit result is not expected
        DepositFailed,
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
    #[pallet::getter(fn chain_id)]
    pub type ChainID<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_infos)]
    pub type AssetInfos<T: Config> =
        StorageMap<_, Blake2_128Concat, CurrencyIdOf<T>, AssetInfo<T::AccountId, AssetMeta>>;

    #[pallet::storage]
    #[pallet::getter(fn symbols)]
    pub type AssetSymbols<T: Config> =
        StorageMap<_, Identity, Vec<u8>, CurrencyIdOf<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn para_maps)]
    pub type ParaAssetMaps<T: Config> =
        StorageDoubleMap<_, Identity, u32, Identity, Vec<u8>, CurrencyIdOf<T>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_para_ids)]
    pub type AssetParaIds<T: Config> = StorageMap<_, Identity, CurrencyIdOf<T>, u32, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn location_symbols)]
    pub type LocationToSymbols<T: Config> = StorageMap<_, Identity, Vec<u8>, Vec<u8>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn symbol_locations)]
    pub type SymbolToLocations<T: Config> = StorageMap<_, Identity, Vec<u8>, Vec<u8>, OptionQuery>;

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

            ensure!(
                metadata.name.len() > 2
                    && metadata.symbol.len() > 1
                    && metadata.decimals > 0u8
                    && metadata.decimals < 19,
                Error::<T>::MetadataErr
            );

            let asset_id = wetee_dao::NextDaoId::<T>::get();

            // 记录下一个 DAO id
            let next_id = asset_id.checked_add(1).ok_or(Error::<T>::AssetIdOverflow)?;
            wetee_dao::NextDaoId::<T>::put(next_id);

            // 创建资产
            Self::try_create(who.clone(), asset_id, metadata, init_amount)?;

            Ok(().into())
        }

        #[pallet::call_index(010)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::delete_asset())]
        pub fn delete_asset(
            origin: OriginFor<T>,
            asset_id: WeAssetId,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin.clone())?;

            // 获取资产信息
            let currency_id = Self::get_local_asset(asset_id);
            // 确认是否是 ParaAsset,如果资产是 ParaAsset,则不允许删除
            let asset = AssetParaIds::<T>::get(currency_id);
            if asset.is_some() {
                return Err(Error::<T>::ParaAssetAlreadyExists.into());
            }

            let asset_info = AssetInfos::<T>::get(currency_id).ok_or(Error::<T>::AssetNotExists)?;
            let token_symbol = asset_info.metadata.symbol.clone();

            // 删除资产信息
            AssetInfos::<T>::remove(currency_id);
            AssetSymbols::<T>::remove(token_symbol);

            Ok(().into())
        }

        // /// You should have created the asset first.
        // /// 设置资产元数据
        // #[pallet::call_index(004)]
        // #[pallet::weight(<T as pallet::Config>::WeightInfo::set_metadata())]
        // pub fn set_metadata(
        //     origin: OriginFor<T>,
        //     asset_id: WeAssetId,
        //     metadata: AssetMeta,
        // ) -> DispatchResultWithPostInfo {
        //     let who = ensure_signed(origin)?;

        //     ensure!(
        //         metadata.name.len() > 2 && metadata.symbol.len() > 1,
        //         Error::<T>::MetadataErr
        //     );

        //     let mut asset_info = AssetInfos::<T>::get(Self::get_local_asset(asset_id))
        //         .ok_or(Error::<T>::AssetNotExists)?;

        //     ensure!(
        //         asset_info.metadata != metadata,
        //         Error::<T>::MetadataNotChange
        //     );

        //     // 确保 decimal 不变
        //     ensure!(
        //         asset_info.metadata.decimals == metadata.decimals,
        //         Error::<T>::ShouldNotChangeDecimals
        //     );

        //     // 确认用户是否是创建者
        //     ensure!(who == asset_info.owner, Error::<T>::ShouldNotChangeDecimals);

        //     asset_info.metadata = metadata.clone();

        //     AssetInfos::<T>::insert(Self::get_local_asset(asset_id), asset_info);
        //     Self::deposit_event(Event::SetMetadata(who, asset_id, metadata));

        //     Ok(().into())
        // }

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

            <Self as MultiCurrency<T::AccountId>>::withdraw(
                Self::get_local_asset(asset_id),
                &user,
                amount,
                ExistenceRequirement::AllowDeath,
            )?;
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

            <Self as MultiCurrency<T::AccountId>>::transfer(
                Self::get_local_asset(asset_id),
                &from,
                &to,
                amount,
                ExistenceRequirement::AllowDeath,
            )?;
            Ok(().into())
        }

        #[pallet::call_index(007)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::parachain_asset_register())]
        pub fn parachain_asset_register(
            origin: OriginFor<T>,
            para_id: u32,
            general_key: BoundedVec<u8, ConstU32<32>>,
            metadata: AssetMeta,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin.clone())?;

            let who = wetee_dao::Pallet::<T>::asset_root();
            ensure!(
                metadata.name.len() > 2
                    && metadata.symbol.len() > 1
                    && metadata.decimals > 0u8
                    && metadata.decimals < 19,
                Error::<T>::MetadataErr
            );

            // 确保 symbol 不重复
            let token_symbol = metadata.symbol.clone();
            ensure!(
                !ParaAssetMaps::<T>::contains_key(para_id, token_symbol.clone()),
                Error::<T>::AssetAlreadyExists
            );

            // 确认用户是否是组织创建者
            let asset_id = wetee_dao::NextDaoId::<T>::get();

            // 记录下一个 DAO id
            let next_id = asset_id.checked_add(1).ok_or(Error::<T>::AssetIdOverflow)?;
            wetee_dao::NextDaoId::<T>::put(next_id);

            // 创建资产
            Self::try_create(who.clone(), asset_id, metadata.clone(), 0u32.into())?;

            let currency_id = T::CurrencyIdConvert::convert(asset_id);
            ParaAssetMaps::<T>::insert(para_id, token_symbol.clone(), currency_id);
            AssetParaIds::<T>::insert(currency_id, para_id);

            let key = general_key.to_vec();
            SymbolToLocations::<T>::insert(token_symbol.clone(), key.clone());
            LocationToSymbols::<T>::insert(key, token_symbol);

            Ok(().into())
        }

        #[pallet::call_index(008)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_chain_id())]
        pub fn set_chain_id(origin: OriginFor<T>, para_id: u32) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin.clone())?;

            ChainID::<T>::put(para_id);

            Ok(().into())
        }

        #[pallet::call_index(009)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::delete_parachain_for_asset())]
        pub fn delete_parachain_for_asset(
            origin: OriginFor<T>,
            asset_id: WeAssetId,
            para_id: u32,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin.clone())?;

            // 获取资产信息
            let currency_id = Self::get_local_asset(asset_id);
            let asset_info = AssetInfos::<T>::get(currency_id).ok_or(Error::<T>::AssetNotExists)?;
            let token_symbol = asset_info.metadata.symbol.clone();

            ParaAssetMaps::<T>::remove(para_id, token_symbol.clone());
            AssetParaIds::<T>::remove(currency_id);

            let general_key = SymbolToLocations::<T>::get(token_symbol.clone());
            if general_key.clone().is_some() {
                SymbolToLocations::<T>::remove(token_symbol.clone());
                LocationToSymbols::<T>::remove(general_key.unwrap());
            }

            Ok(().into())
        }

        #[pallet::call_index(011)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_parachain_for_asset())]
        pub fn set_parachain_for_asset(
            origin: OriginFor<T>,
            asset_id: WeAssetId,
            para_id: u32,
            general_key: BoundedVec<u8, ConstU32<32>>,
        ) -> DispatchResultWithPostInfo {
            // TODO 更新治理模块后更新
            ensure_root(origin.clone())?;

            let currency_id = T::CurrencyIdConvert::convert(asset_id);

            // 确认是否是 ParaAsset,如果资产是 ParaAsset,则不允许重新创建
            let asset = AssetParaIds::<T>::get(currency_id);
            if asset.is_some() {
                return Err(Error::<T>::ParaAssetAlreadyExists.into());
            }

            let asset_info = AssetInfos::<T>::get(currency_id).ok_or(Error::<T>::AssetNotExists)?;
            let token_symbol = asset_info.metadata.symbol.clone();

            ParaAssetMaps::<T>::insert(para_id, token_symbol.clone(), currency_id);
            AssetParaIds::<T>::insert(currency_id, para_id);

            let key = general_key.to_vec();
            SymbolToLocations::<T>::insert(token_symbol.clone(), key.clone());
            LocationToSymbols::<T>::insert(key, token_symbol);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 总发行量
        pub fn get_total_issuance(asset_id: WeAssetId) -> BalanceOf<T> {
            <Self as MultiCurrency<T::AccountId>>::total_issuance(Self::get_local_asset(asset_id))
        }

        /// 获取账户金额
        pub fn get_balance(
            asset_id: WeAssetId,
            who: T::AccountId,
        ) -> result::Result<BalanceOf<T>, DispatchError> {
            let balance = <Self as MultiCurrency<T::AccountId>>::total_balance(
                Self::get_local_asset(asset_id),
                &who,
            );
            Ok(balance)
        }

        // 获取可用金额
        pub fn get_free_balance(asset_id: WeAssetId, who: T::AccountId) -> BalanceOf<T> {
            let balance = <Self as MultiCurrency<T::AccountId>>::free_balance(
                Self::get_local_asset(asset_id),
                &who.clone(),
            );
            balance
        }

        // 设置账户金额
        pub fn try_set_balance(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::deposit(
                Self::get_local_asset(asset_id),
                &who,
                value,
            )?;
            Ok(())
        }

        /// 为...锁定保证金
        pub fn try_reserve(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::reserve(
                Self::get_local_asset(asset_id),
                &who,
                value,
            )?;
            Ok(())
        }

        /// 解除保证
        pub fn try_unreserve(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiReservableCurrency<T::AccountId>>::unreserve(
                Self::get_local_asset(asset_id),
                &who,
                value,
            );
            Ok(())
        }

        /// 尽可能解除保证
        pub fn try_slash_reserved(
            asset_id: WeAssetId,
            who: T::AccountId,
            value: BalanceOf<T>,
        ) -> BalanceOf<T> {
            <Self as MultiReservableCurrency<T::AccountId>>::slash_reserved(
                Self::get_local_asset(asset_id),
                &who,
                value,
            )
        }

        // 创建代币
        pub fn try_create(
            user: T::AccountId,
            asset_id: WeAssetId,
            metadata: AssetMeta,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            if AssetSymbols::<T>::contains_key(metadata.symbol.clone()) {
                return Ok(());
            }

            ensure!(
                !Self::is_exists(asset_id)
                    && <T as pallet::Config>::MultiCurrency::total_issuance(
                        T::CurrencyIdConvert::convert(asset_id)
                    ) == BalanceOf::<T>::from(0u32),
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

            let cid = Self::get_local_asset(asset_id);
            <T as pallet::Config>::MultiCurrency::deposit(cid.clone(), &user, amount)?;

            AssetInfos::<T>::insert(
                Self::get_local_asset(asset_id),
                AssetInfo {
                    owner: user.clone(),
                    metadata: metadata.clone(),
                },
            );

            AssetSymbols::<T>::insert(metadata.symbol, cid.clone());

            Self::deposit_event(Event::CreateAsset(user, asset_id, amount));

            Ok(())
        }

        /// 转帐
        pub fn try_transfer(
            asset_id: WeAssetId,
            from: T::AccountId,
            to: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::transfer(
                Self::get_local_asset(asset_id),
                &from,
                &to,
                value,
                ExistenceRequirement::AllowDeath,
            )?;
            Ok(())
        }

        /// 销毁资产
        pub fn try_burn(
            asset_id: WeAssetId,
            from: T::AccountId,
            value: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            <Self as MultiCurrency<T::AccountId>>::withdraw(
                Self::get_local_asset(asset_id),
                &from,
                value,
                ExistenceRequirement::AllowDeath,
            )?;
            Ok(())
        }

        /// 转帐
        pub fn burn_with_number(
            asset_id: WeAssetId,
            from: T::AccountId,
            value: u128,
        ) -> result::Result<(), DispatchError> {
            let amount: BalanceOf<T> = value.saturated_into::<BalanceOf<T>>();

            <Self as MultiCurrency<T::AccountId>>::withdraw(
                Self::get_local_asset(asset_id),
                &from,
                amount,
                ExistenceRequirement::AllowDeath,
            )?;
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
            <Self as MultiCurrency<T::AccountId>>::deposit(
                Self::get_local_asset(asset_id),
                &dest,
                value,
            )?;

            Ok(().into())
        }

        // 锁定资产
        pub fn try_extend_lock(
            lock_id: LockIdentifier,
            asset_id: WeAssetId,
            who: T::AccountId,
            amount: BalanceOf<T>,
        ) -> result::Result<(), DispatchError> {
            // 确认组织是否存在
            ensure!(Self::is_exists(asset_id), Error::<T>::AssetNotExists);

            T::MultiCurrency::extend_lock(lock_id, Self::get_local_asset(asset_id), &who, amount)
        }

        // 解除资产锁定
        pub fn try_remove_lock(
            lock_id: LockIdentifier,
            asset_id: WeAssetId,
            who: T::AccountId,
        ) -> result::Result<(), DispatchError> {
            // 确认组织是否存在
            ensure!(Self::is_exists(asset_id), Error::<T>::AssetNotExists);

            T::MultiCurrency::remove_lock(lock_id, Self::get_local_asset(asset_id), &who)
        }

        /// 判断资产是否存在
        pub fn is_exists(asset_id: WeAssetId) -> bool {
            if asset_id == NATIVE_ASSET_ID {
                return true;
            }
            if AssetInfos::<T>::get(Self::get_local_asset(asset_id)).is_some() {
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

        /// 获取跨链资产ID
        pub fn para_asset_id(para_id: u32, general_key: Vec<u8>) -> Option<WeAssetId> {
            let symbol = LocationToSymbols::<T>::get(general_key.clone());
            if symbol.is_none() {
                return None;
            }

            let id = ParaAssetMaps::<T>::get(para_id, symbol.unwrap());
            match id {
                Some(cid) => Some(T::CurrencyIdConvert::convert(cid)),
                None => None,
            }
        }

        /// 获取跨链资产ID
        pub fn para_id(cid: WeAssetId) -> Option<u32> {
            let id = AssetParaIds::<T>::get(Self::get_local_asset(cid));
            match id {
                Some(cid) => Some(cid),
                None => None,
            }
        }

        /// 获取本地资产ID
        pub fn get_local_asset(asset_id: WeAssetId) -> CurrencyIdOf<T> {
            T::CurrencyIdConvert::convert(asset_id)
        }

        /// 获取跨链资产ID localtion
        pub fn para_asset_localtion(id: WeAssetId) -> Option<Vec<u8>> {
            let id = AssetInfos::<T>::get(T::CurrencyIdConvert::convert(id));
            match id {
                Some(info) => {
                    let symbol: Vec<u8> = info.metadata.symbol.clone();
                    SymbolToLocations::<T>::get(symbol)
                }
                None => None,
            }
        }
    }
}
