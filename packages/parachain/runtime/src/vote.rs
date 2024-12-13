// use crate::*;
use codec::{Decode, Encode, MaxEncodedLen};
pub use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};

use crate::{AccountId, Asset, Balance, BlockNumber};
use wetee_gov::traits::PledgeTrait;
use wetee_primitives::types::WeAssetId;

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, Clone, TypeInfo, Copy, MaxEncodedLen)]
pub struct Pledge;

impl Default for Pledge {
    fn default() -> Self {
        Pledge {}
    }
}

impl PledgeTrait<Balance, AccountId, WeAssetId, BlockNumber, DispatchError> for Pledge {
    fn try_vote(
        who: &AccountId,
        dao_id: &WeAssetId,
        vote_model: u8,
        amount: Balance,
    ) -> Result<(Balance, BlockNumber), DispatchError> {
        let amount = {
            Asset::try_reserve(*dao_id, who.clone(), amount)?;
            if vote_model == 1 {
                // 1 account = 1 vote
                1
            } else {
                // 1 token = 1 vote
                amount
            }
        };
        log::info!("try_vote amount {:?}", amount);
        Ok((amount, 100))
    }

    fn vote_end_do(
        who: &AccountId,
        dao_id: &WeAssetId,
        amount: Balance,
    ) -> Result<(), DispatchError> {
        Asset::try_unreserve(*dao_id, who.clone(), amount)?;
        Ok(())
    }
}
