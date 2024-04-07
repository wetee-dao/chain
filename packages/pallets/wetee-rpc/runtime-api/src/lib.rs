#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::Codec;
use sp_runtime::traits::MaybeDisplay;
use wetee_primitives::types::DaoAssetId;

sp_api::decl_runtime_apis! {
    pub trait WeteeAssetRuntimeApi<AccountId,Balance>
    where
        AccountId: Codec,
        Balance: Codec + MaybeDisplay,
    {
        fn get_asset_balance(dao_id: DaoAssetId,who: AccountId) -> Balance;
    }
}
