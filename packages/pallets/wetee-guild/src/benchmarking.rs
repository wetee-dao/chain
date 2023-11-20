use super::*;
use crate::{Call, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use wetee_primitives::types::DaoAssetId;

fn get_alice<T: Config>() -> T::AccountId {
    account("alice", 1, 1)
}

fn get_bob<T: Config>() -> T::AccountId {
    account("bob", 1, 1)
}

fn creat_dao<T: Config>() -> (DaoAssetId, DaoAssetId) {
    let alice = get_alice::<T>();
    let dao_id = DaoAssetId::default();
    let second_id: DaoAssetId = Default::default();
    assert!(wetee_org::Pallet::<T>::create_dao(
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

#[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn guild_join() {
        let (_dao_id, _second_id) = creat_dao::<T>();
        let bob = get_bob::<T>();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(wetee_org::Pallet::<T>::dao_approve(5000)),
            5000,
            0,
            bob.clone(),
        );
    }

    #[benchmark]
    fn create_guild() {
        let (_dao_id, _second_id) = creat_dao::<T>();
        let alice = get_alice::<T>();

        #[extrinsic_call]
        _(
            RawOrigin::Signed(wetee_org::Pallet::<T>::dao_approve(5000)),
            5000,
            vec![1; 4],
            vec![1; 4],
            vec![1; 4],
            alice.clone(),
        );
    }
}
