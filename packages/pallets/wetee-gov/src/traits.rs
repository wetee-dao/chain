use super::*;

pub trait PledgeTrait<VoteWeight, AccountId, DaoId, BlockNumber, DispatchError> {
    fn try_vote(
        &self,
        who: &AccountId,
        dao_id: &DaoId,
        vote_model: u8,
    ) -> result::Result<(VoteWeight, BlockNumber), DispatchError>;
    fn vote_end_do(&self, who: &AccountId, dao_id: &DaoId) -> result::Result<(), DispatchError>;
}

pub trait ConvertInto<A> {
    fn convert_into(&self) -> A;
}

impl<A: Default> ConvertInto<A> for () {
    fn convert_into(&self) -> A {
        Default::default()
    }
}
