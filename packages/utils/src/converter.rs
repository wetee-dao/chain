use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Convert, BoundedVec, RuntimeDebug};
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;
use xcm::{
    latest::Asset,
    prelude::{Fungible, GeneralKey, Parachain},
    v4::prelude::*,
    v4::{AssetId, Location, Parent},
};

use wetee_primitives::{types::WeAssetId, u8_32_vec};

// #[derive(
//     Encode,
//     Decode,
//     MaxEncodedLen,
//     Eq,
//     PartialEq,
//     Copy,
//     Clone,
//     RuntimeDebug,
//     PartialOrd,
//     Ord,
//     TypeInfo,
//     Serialize,
//     Deserialize,
// )]
// pub struct CurrencyId {
//     // para id relay==0  other parachain
//     pub para_id: u32,
//     pub currency_id: WeAssetId,
// }

pub type CurrencyId = WeAssetId;

/// CurrencyIdConvert
pub struct CurrencyIdConvert<Runtime>(PhantomData<Runtime>);

/// Convert CurrencyId to Location
impl<Runtime: wetee_assets::Config> Convert<CurrencyId, Option<Location>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(id: CurrencyId) -> Option<Location> {
        let name: Vec<u8> = wetee_assets::Pallet::<Runtime>::para_asset_symbol(id).unwrap();
        let para_id = wetee_assets::Pallet::<Runtime>::para_id(id).unwrap();
        Some(
            (
                Parent,
                Parachain(para_id),
                Junction::from(BoundedVec::try_from(name).unwrap()),
            )
                .into(),
        )
    }
}

/// Convert Location to CurrencyId
impl<Runtime: wetee_assets::Config> Convert<Location, Option<CurrencyId>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(l: Location) -> Option<CurrencyId> {
        if l == Location::parent() {
            let token_symbol = b"DOT".to_vec();
            let asset_id = wetee_assets::Pallet::<Runtime>::para_asset_id(0, token_symbol);
            log::info!(
                "relay ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++{:?}",
                asset_id
            );

            if asset_id.is_none() {
                return None;
            }

            return Some(asset_id.unwrap());
        }

        let (parents, interior) = l.unpack();
        log::info!(
            "parents interior ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++{:?} = {:?}",
            parents,interior
        );
        match interior {
            [Parachain(para), GeneralKey { data, .. }] => {
                let symbol = u8_32_vec(data.clone());
                let asset_id =
                    wetee_assets::Pallet::<Runtime>::para_asset_id(para.clone(), symbol.clone());

                if asset_id.is_some() {
                    Some(asset_id.unwrap())
                } else {
                    log::error!(
                        "currency not found ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++{:?} {:?}",
                        para,symbol
                    );
                    None
                }
            }
            _ => None,
        }
    }
}

/// Convert Asset to CurrencyId
impl<Runtime: wetee_assets::Config> Convert<Asset, Option<CurrencyId>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(asset: Asset) -> Option<CurrencyId> {
        if let Asset {
            fun: Fungible(_),
            id: AssetId(id),
        } = asset
        {
            Self::convert(id)
        } else {
            Option::None
        }
    }
}

/// Convert CurrencyId to (u32, WeAssetId)
impl<Runtime: wetee_assets::Config> Convert<WeAssetId, CurrencyId> for CurrencyIdConvert<Runtime> {
    fn convert(id: WeAssetId) -> CurrencyId {
        return id;
    }
}

// Convert CurrencyId to WeAssetId
// impl<Runtime: wetee_assets::Config> Convert<CurrencyId, WeAssetId> for CurrencyIdConvert<Runtime> {
//     fn convert(id: CurrencyId) -> WeAssetId {
//         return id;
//     }
// }
