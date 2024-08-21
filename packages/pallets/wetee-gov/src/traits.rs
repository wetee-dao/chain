use super::*;

pub trait PledgeTrait<VoteWeight, AccountId, DaoId, BlockNumber, DispatchError> {
    fn try_vote(
        who: &AccountId,
        dao_id: &DaoId,
        vote_model: u8,
        amount: VoteWeight,
    ) -> result::Result<(VoteWeight, BlockNumber), DispatchError>;
    fn vote_end_do(
        who: &AccountId,
        dao_id: &DaoId,
        amount: VoteWeight,
    ) -> result::Result<(), DispatchError>;
}

pub trait ConvertInto<A> {
    fn convert_into(&self) -> A;
}

impl<A: Default> ConvertInto<A> for () {
    fn convert_into(&self) -> A {
        Default::default()
    }
}
