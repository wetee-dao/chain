
use super::*;
use crate::{Pallet, Call};
use frame_benchmarking::v2::*;
use wetee_primitives::{
    types::{DaoAssetId},
};
use frame_system::RawOrigin;

fn get_alice<T: Config>() -> T::AccountId {
	account("alice", 1, 1)
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
		vec![1; 4],).is_ok()
	);
	(dao_id, second_id)
}

#[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
	use super::*;

    #[benchmark]
    fn sudo(){
      let (_dao_id, _second_id) = creat_dao::<T>();
      let alice = get_alice::<T>();
      let proposal: <T as wetee_org::Config>::RuntimeCall  = wetee_org::Call::org_integrate_app {
          dao_id: 5000,
          app_id: 0,
      }.into();
      let _ = wetee_org::Pallet::<T>::create_app(
        RawOrigin::Signed(alice.clone()).into(),
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
        vec![1; 4],
      );

      #[extrinsic_call]
		  _(RawOrigin::Signed(alice.clone()),5000, Box::new(proposal.clone()));
    }

    #[benchmark]
    fn set_sudo_account(){
      let (_dao_id, _second_id) = creat_dao::<T>();
      let alice = get_alice::<T>();

      #[extrinsic_call]
		  _(RawOrigin::Signed(alice.clone()),5000, alice.clone());
    }

    #[benchmark]
    fn close_sudo(){
      let (_dao_id, _second_id) = creat_dao::<T>();
      let alice = get_alice::<T>();

      #[extrinsic_call]
		  _(RawOrigin::Signed(alice.clone()),5000);
    }
}