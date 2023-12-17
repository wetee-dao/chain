#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_guild;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug};
use sp_runtime::BoundedVec;
use wetee_gov::{DefaultPeriods, MemberData, Period};
use wetee_primitives::types::DaoAssetId;

pub fn create_asset() -> DaoAssetId {
    let dao_id = wetee_org::Pallet::<Test>::next_dao_id();

    wetee_org::Pallet::<Test>::create_dao(
        RuntimeOrigin::signed(ALICE),
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
    .unwrap();

    wetee_assets::Pallet::<Test>::create_asset(
        RuntimeOrigin::signed(ALICE),
        dao_id,
        wetee_assets::DaoAssetMeta {
            name: "TestA".as_bytes().to_vec(),
            symbol: "TA".as_bytes().to_vec(),
            decimals: 10,
        },
        10000,
        99,
    )
    .unwrap();

    let proposal = RuntimeCall::WeteeAsset(wetee_assets::Call::set_existenial_deposit {
        dao_id,
        existenial_deposit: 1,
    });

    assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(dao_id).unwrap().creator),
        dao_id,
        Box::new(proposal)
    ));

    dao_id
}

#[test]
pub fn test_guild_join_request() {
    new_test_run().execute_with(|| {
        let dao_id = create_asset();

        // 加入社区
        wetee_assets::Pallet::<Test>::join(RuntimeOrigin::signed(BOB), dao_id, 100, 100000)
            .unwrap();

        // 创建提案
        let proposal = RuntimeCall::WETEEGuild(wetee_guild::Call::guild_join {
            dao_id,
            guild_id: 0,
            who: BOB,
        });

        // 加入社区后尝试提案
        assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            Box::new(proposal)
        ));

        let ms = wetee_org::GuildMembers::<Test>::get(dao_id, 0);
        assert!(ms.len() == 2);
    });
}
