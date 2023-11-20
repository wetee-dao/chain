#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_guild;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug, log::debug};
use wetee_gov::MemberData;
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
        assert!(wetee_guild::Pallet::<Test>::guild_join(
            RuntimeOrigin::signed(BOB),
            dao_id,
            0,
            BOB,
        )
        .is_err());

        // 创建提案
        let proposal = RuntimeCall::WETEEGuild(wetee_guild::Call::guild_join {
            dao_id,
            guild_id: 0,
            who: BOB,
        });

        // 未加入社区尝试提案
        assert!(wetee_gov::Pallet::<Test>::submit_proposal(
            RuntimeOrigin::signed(BOB),
            dao_id,
            MemberData::GLOBAL,
            Box::new(proposal.clone()),
            0
        )
        .is_err());

        // 加入社区
        wetee_assets::Pallet::<Test>::join(RuntimeOrigin::signed(BOB), dao_id, 100, 100).unwrap();

        // 加入社区后尝试提案
        assert_ok!(wetee_gov::Pallet::<Test>::submit_proposal(
            RuntimeOrigin::signed(BOB),
            dao_id,
            MemberData::GLOBAL,
            Box::new(proposal),
            0
        ));

        // 开始投票
        frame_system::Pallet::<Test>::set_block_number(10000);
        assert_ok!(wetee_gov::Pallet::<Test>::deposit_proposal(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32,
            100,
        ));

        // 投票
        assert_ok!(wetee_gov::Pallet::<Test>::vote_for_prop(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32,
            Vote(100000),
            wetee_gov::Opinion::YES,
        ));

        // 运行提案
        assert!(wetee_gov::Pallet::<Test>::run_proposal(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32
        )
        .is_err());

        frame_system::Pallet::<Test>::set_block_number(20000);

        // 运行代码
        assert!(wetee_gov::Pallet::<Test>::run_proposal(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            0u32
        )
        .is_ok());

        let ms = wetee_org::GuildMembers::<Test>::get(dao_id, 0);
        assert!(ms.len() == 2);
    });
}
