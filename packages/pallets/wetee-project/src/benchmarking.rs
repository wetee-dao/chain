use super::*;

use frame_benchmarking::v2::*;
use frame_system::{RawOrigin, System};
use wetee_primitives::types::DaoAssetId;

fn get_alice<T: Config>() -> T::AccountId {
    account("alice", 1, 1)
}

fn creat_dao<T: Config>() -> (DaoAssetId, DaoAssetId) {
    let alice = get_alice::<T>();
    let dao_id = 5000;
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

    let _v10: BlockNumberFor<T> = 10u32.into();
    System::<T>::set_block_number(_v10);

    let value: BalanceOf<T> = 10000u32.into();
    let value2: BalanceOf<T> = 100u32.into();
    wetee_assets::Pallet::<T>::create_asset(
        RawOrigin::Signed(alice.clone()).into(),
        dao_id,
        wetee_assets::DaoAssetMeta {
            name: "TESTA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        value,
        value2,
    )
    .unwrap();

    (dao_id, second_id)
}

// #[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
// mod benchmarks {
// 	use super::*;

// }
