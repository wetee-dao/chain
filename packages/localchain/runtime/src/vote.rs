// use crate::*;
pub use scale_info::TypeInfo;
use sp_runtime::DispatchError;

use crate::{AccountId, Asset, Balance, BlockNumber};
use wetee_gov::traits::PledgeTrait;
use wetee_primitives::types::DaoAssetId;

pub struct Pledge;

impl Default for Pledge {
    fn default() -> Self {
        Pledge {}
    }
}

impl PledgeTrait<Balance, AccountId, DaoAssetId, BlockNumber, DispatchError> for Pledge {
    fn try_vote(
        who: &AccountId,
        dao_id: &DaoAssetId,
        vote_model: u8,
        amount: Balance,
    ) -> Result<(Balance, BlockNumber), DispatchError> {
        let amount = {
            #[cfg(not(feature = "runtime-benchmarks"))]
            Asset::reserve(*dao_id, who.clone(), amount)?;
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
        dao_id: &DaoAssetId,
        amount: Balance,
    ) -> Result<(), DispatchError> {
        #[cfg(not(feature = "runtime-benchmarks"))]
        Asset::unreserve(*dao_id, who.clone(), amount)?;
        Ok(())
    }
}
