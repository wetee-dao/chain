use super::*;
use crate::{Call, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use wetee_primitives::types::WeAssetId;

fn get_alice<T: Config>() -> T::AccountId {
    account("alice", 1, 1)
}

fn get_bob<T: Config>() -> T::AccountId {
    account("bob", 1, 1)
}

fn creat_dao<T: Config>() -> (WeAssetId, WeAssetId) {
    let alice = get_alice::<T>();
    let dao_id = WeAssetId::default();
    let second_id: WeAssetId = Default::default();
    assert!(wetee_dao::Pallet::<T>::create_dao(
        RawOrigin::Signed(alice).into(),
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
    )
    .is_ok());
    (dao_id, second_id)
}

#[benchmarks( where <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn spend() {
        let (_dao_id, _second_id) = creat_dao::<T>();
        let bob = get_bob::<T>();
        let value: BalanceOf<T> = 0u32.into();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(5000, 0)),
            5000,
            bob.clone(),
            value,
        );
    }
}
