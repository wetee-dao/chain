use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Convert, BoundedVec, RuntimeDebug};
use sp_std::vec::Vec;
use xcm::{
    latest::Asset,
    prelude::{AccountId32, Ethereum, Fungible, GeneralKey, GlobalConsensus, Parachain},
    v4::{prelude::*, Weight},
    v4::{AssetId, InteriorLocation, Location, NetworkId, Parent},
};

use crate::types::WeAssetId;

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
#[non_exhaustive]
pub enum CurrencyId {
    NativeToken(WeAssetId),
    Relay,
    Bifrost,
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<Location>> for CurrencyIdConvert {
    fn convert(id: CurrencyId) -> Option<Location> {
        match id {
            CurrencyId::NativeToken(id) => Some(
                (
                    Parent,
                    Parachain(1),
                    Junction::from(BoundedVec::try_from(b"Bifrost".to_vec()).unwrap()),
                    // Junction::GeneralKey(id.into()),
                )
                    .into(),
            ),
            CurrencyId::Relay => Some(Parent.into()),
            CurrencyId::Bifrost => Some(
                (
                    Parent,
                    Parachain(1),
                    Junction::from(BoundedVec::try_from(b"Bifrost".to_vec()).unwrap()),
                )
                    .into(),
            ),
        }
    }
}

impl Convert<Location, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(l: Location) -> Option<CurrencyId> {
        let mut a: Vec<u8> = "A".into();
        a.resize(32, 0);
        if l == Location::parent() {
            return Some(CurrencyId::Relay);
        }
        match l.unpack() {
            (parents, interior) if parents == 1 => match interior {
                [Parachain(1), GeneralKey { data, .. }] if data.to_vec() == a => {
                    Some(CurrencyId::Relay)
                }
                _ => None,
            },
            (parents, interior) if parents == 0 => match interior {
                [GeneralKey { data, .. }] if data.to_vec() == a => Some(CurrencyId::Bifrost),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Convert<Asset, Option<CurrencyId>> for CurrencyIdConvert {
    fn convert(a: Asset) -> Option<CurrencyId> {
        if let Asset {
            fun: Fungible(_),
            id: AssetId(id),
        } = a
        {
            Self::convert(id)
        } else {
            Option::None
        }
    }
}

impl Convert<WeAssetId, CurrencyId> for CurrencyIdConvert {
    fn convert(id: WeAssetId) -> CurrencyId {
        return CurrencyId::NativeToken(id);
    }
}
