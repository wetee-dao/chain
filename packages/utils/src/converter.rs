use sp_runtime::{traits::Convert, BoundedVec};
use sp_std::marker::PhantomData;
use xcm::{
    latest::Asset,
    prelude::{Fungible, GeneralKey, Parachain},
    v4::prelude::*,
    v4::{AssetId, Location, Parent},
};

use wetee_primitives::{types::WeAssetId, u8_32_vec, values::PARENT};

pub type CurrencyId = WeAssetId;

/// CurrencyIdConvert
pub struct CurrencyIdConvert<Runtime>(PhantomData<Runtime>);

/// Convert CurrencyId to Location
impl<Runtime: wetee_assets::Config> Convert<CurrencyId, Option<Location>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(id: CurrencyId) -> Option<Location> {
        let general_key = wetee_assets::Pallet::<Runtime>::para_asset_localtion(id);
        if general_key.is_none() {
            log::info!(
                "to relay general_key ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++ NONE"
            );
            return None;
        }

        // if parent
        if general_key.clone().unwrap() == PARENT.to_vec() {
            log::info!(
                "to relay general_key ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++ PARENT"
            );
            return Some(Parent.into());
        }

        let para_id = wetee_assets::Pallet::<Runtime>::para_id(id).unwrap();
        log::info!(
            "to para_id - general_key ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++ {:?} {:?}",
            para_id,general_key
        );
        Some(
            (
                Parent,
                Parachain(para_id),
                Junction::from(BoundedVec::try_from(general_key.unwrap()).unwrap()),
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
            let general_key = PARENT.to_vec();
            let asset_id = wetee_assets::Pallet::<Runtime>::para_asset_id(0, general_key);
            log::info!(
                "relay ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++ {:?}",
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
            [Parachain(para), GeneralKey { data, length }] => {
                let symbol = u8_32_vec(data.clone(), length.clone());
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

/// Convert WeAssetId to CurrencyId
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
