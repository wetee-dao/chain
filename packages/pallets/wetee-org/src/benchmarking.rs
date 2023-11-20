use super::*;
use crate::{Config, Pallet as Org};
use frame_benchmarking::{
    account, benchmarks, benchmarks_instance, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin as SystemOrigin;
use wetee_primitives::{
    traits::AfterCreate,
    types::{DaoAssetId, GuildId, ProjectId, TaskId},
};

fn get_alice<T: Config>() -> T::AccountId {
    account("alice", 1, 1)
}

fn get_dao_account<T: Config>(second_id: DaoAssetId) -> T::AccountId {
    Org::<T>::try_get_dao_account_id(second_id).unwrap()
}

fn creat_dao<T: Config>() -> (DaoAssetId, DaoAssetId) {
    let alice = get_alice::<T>();
    let dao_id = DaoAssetId::default();
    let second_id: DaoAssetId = Default::default();
    assert!(Org::<T>::create_dao(
        SystemOrigin::Signed(alice).into(),
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

benchmarks! {
    create_dao {
    }:{
        let (dao_id, second_id) = creat_dao::<T>();
    }

    update_dao {
        let (dao_id, second_id) = creat_dao::<T>();
    }:{
        assert!(Org::<T>::daos(5000).is_some());
        let alice = get_alice::<T>();
        Org::<T>::update_dao(
            SystemOrigin::Signed(alice).into(),
            5000,
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(vec![1; 4]),
            Some(crate::Status::Active)
        );
    }

    create_roadmap_task {
        let (dao_id, second_id) = creat_dao::<T>();
    }:{
        assert!(Org::<T>::daos(5000).is_some());
        let alice = get_alice::<T>();
        Org::<T>::create_roadmap_task(
            SystemOrigin::Signed(alice).into(),
            5000,
            202301,
            vec![1; 4],
            1,
            vec![1].into(),
        );
    }

    update_roadmap_task{
        let alice = get_alice::<T>();
        let (dao_id, second_id) = creat_dao::<T>();
        Org::<T>::create_roadmap_task(
            SystemOrigin::Signed(alice).into(),
            5000,
            202301,
            vec![1; 4], // name
            1, // priority
            vec![1].into(), // tags
        );
    Org::<T>::get_task(5000, 202301, 0).unwrap();
    }:{
        assert!(Org::<T>::daos(5000).is_some());
        let alice = get_alice::<T>();

        Org::<T>::update_roadmap_task(
            SystemOrigin::Signed(alice).into(),
            5000,
            202301,
            0, // task_id
            0, // priority
            1, // status
            Some(vec![1].into()), // tags
        );
    }

  create_app{
    let (dao_id, second_id) = creat_dao::<T>();
  }:{
    assert!(Org::<T>::daos(5000).is_some());
    let alice = get_alice::<T>();
    Org::<T>::create_app(
      SystemOrigin::Signed(alice).into(),
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
    );
  }

  update_app_status{
    let (dao_id, second_id) = creat_dao::<T>();
    let alice = get_alice::<T>();
    Org::<T>::create_app(
      SystemOrigin::Signed(alice).into(),
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
    );
  }:{
    assert!(Org::<T>::daos(5000).is_some());
    let alice2 = get_alice::<T>();
    Org::<T>::update_app_status(
      SystemOrigin::Signed(alice2).into(),
      0,
      Status::InActive,
    )
  }

  org_integrate_app{
    let (dao_id, second_id) = creat_dao::<T>();
    let alice = get_alice::<T>();
    Org::<T>::create_app(
      SystemOrigin::Signed(alice).into(),
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
    );
  }:{
    assert!(Org::<T>::daos(5000).is_some());
    let alice2 = get_alice::<T>();
    Org::<T>::org_integrate_app(
      SystemOrigin::Signed(alice2).into(),
      5000,
      0,
    )
  }

  update_org_app_status{
    let (dao_id, second_id) = creat_dao::<T>();
    let alice = get_alice::<T>();
    Org::<T>::create_app(
      SystemOrigin::Signed(alice).into(),
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
      vec![1; 4],
    );
    let alice2 = get_alice::<T>();
    Org::<T>::org_integrate_app(
      SystemOrigin::Signed(alice2).into(),
      5000,
      0,
    );
  }:{
    assert!(Org::<T>::daos(5000).is_some());
    let alice3 = get_alice::<T>();
    Org::<T>::update_org_app_status(
      SystemOrigin::Signed(alice3).into(),
      5000,
      0,
      Status::InActive,
    );
  }
}
