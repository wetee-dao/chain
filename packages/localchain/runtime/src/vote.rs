// use crate::*;
use codec::{Decode, Encode, MaxEncodedLen};
pub use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};

use crate::{AccountId, Balance, BlockNumber, WeTEEAsset};
use wetee_gov::traits::PledgeTrait;
use wetee_primitives::types::DaoAssetId;

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen)]
pub enum Pledge<Balance> {
    FungToken(Balance),
}

impl Default for Pledge<Balance> {
    fn default() -> Self {
        Pledge::FungToken(0)
    }
}

impl PledgeTrait<Balance, AccountId, DaoAssetId, BlockNumber, DispatchError> for Pledge<Balance> {
    fn try_vote(
        &self,
        who: &AccountId,
        dao_id: &DaoAssetId,
        vote_model: u8,
    ) -> Result<(Balance, BlockNumber), DispatchError> {
        let amount = match self {
            Pledge::FungToken(x) => {
                WeTEEAsset::reserve(*dao_id, who.clone(), *x)?;
                if vote_model == 1 {
                    // 1 account = 1 vote
                    1
                } else {
                    // 1 token = 1 vote
                    *x
                }
            }
        };
        log::info!("try_vote amount {:?}", amount);
        Ok((amount, 100))
    }

    fn vote_end_do(&self, who: &AccountId, dao_id: &DaoAssetId) -> Result<(), DispatchError> {
        match self {
            Pledge::FungToken(x) => {
                WeTEEAsset::unreserve(*dao_id, who.clone(), *x)?;
                Ok(())
            }
        }
    }
}
