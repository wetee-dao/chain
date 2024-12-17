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

use wetee_primitives::types::WeAssetId;

#[derive(
    Encode,
    Decode,
    MaxEncodedLen,
    Eq,
    PartialEq,
    Copy,
    Clone,
    RuntimeDebug,
    PartialOrd,
    Ord,
    TypeInfo,
    Serialize,
    Deserialize,
)]
pub struct CurrencyId {
    // para id relay==0 local==1 other parachain
    pub para_id: u32,
    pub currency_id: WeAssetId,
}

pub struct CurrencyIdConvert<Runtime>(PhantomData<Runtime>);

impl<Runtime: wetee_assets::Config> Convert<CurrencyId, Option<Location>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(id: CurrencyId) -> Option<Location> {
        match id.para_id {
            // relay chain
            0 => Some(Parent.into()),
            // local chain
            1 => {
                let chain_id = wetee_assets::Pallet::<Runtime>::chain_id();
                let name: Vec<u8> =
                    wetee_assets::Pallet::<Runtime>::para_asset_name(chain_id, id.currency_id)
                        .unwrap();
                Some(
                    (
                        Parent,
                        Parachain(id.para_id),
                        Junction::from(BoundedVec::try_from(name).unwrap()),
                    )
                        .into(),
                )
            }
            // other parachain
            _ => {
                let name: Vec<u8> =
                    wetee_assets::Pallet::<Runtime>::para_asset_name(id.para_id, id.currency_id)
                        .unwrap();
                Some(
                    (
                        Parent,
                        Parachain(id.para_id),
                        Junction::from(BoundedVec::try_from(name).unwrap()),
                    )
                        .into(),
                )
            }
        }
    }
}

impl<Runtime: wetee_assets::Config> Convert<Location, Option<CurrencyId>>
    for CurrencyIdConvert<Runtime>
{
    fn convert(l: Location) -> Option<CurrencyId> {
        if l == Location::parent() {
            return Some(CurrencyId {
                para_id: 0,
                currency_id: 1,
            });
        }

        let (parents, interior) = l.unpack();
        log::info!(
            "parents interior ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++{:?} = {:?}",
            parents,interior
        );
        match interior {
            [Parachain(para), GeneralKey { data, .. }] => {
                let asset_id =
                    wetee_assets::Pallet::<Runtime>::para_asset_id(para.clone(), data.clone());
                if asset_id.is_some() {
                    Some(CurrencyId {
                        para_id: para.clone(),
                        currency_id: asset_id.unwrap(),
                    })
                } else {
                    log::error!(
                        "currency not found ---------------------------------------------------------------------++++++++++++++++++++++++++++++++++++++++++{:?} {:?}",
                        para,data
                    );
                    None
                }
            }
            _ => None,
        }
    }
}

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

impl<Runtime: wetee_assets::Config> Convert<(u32, WeAssetId), CurrencyId>
    for CurrencyIdConvert<Runtime>
{
    fn convert(id: (u32, WeAssetId)) -> CurrencyId {
        return CurrencyId {
            para_id: id.0,
            currency_id: id.1,
        };
    }
}

impl<Runtime: wetee_assets::Config> Convert<CurrencyId, WeAssetId> for CurrencyIdConvert<Runtime> {
    fn convert(id: CurrencyId) -> WeAssetId {
        return id.currency_id;
    }
}
