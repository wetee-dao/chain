#![allow(unused_imports)]
#![cfg(test)]

use crate as wetee_project;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok, debug};
use wetee_gov::MemberData;
use wetee_primitives::types::DaoAssetId;
use wetee_primitives::types::ProjectId;

pub const PROJECT_INDEX: ProjectId = 1;
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

    let proposal2 = RuntimeCall::WETEEProject(wetee_project::Call::create_project {
        dao_id,
        name: "TestP".as_bytes().to_vec(),
        description: "TestPd".as_bytes().to_vec(),
        creator: ALICE,
    });

    assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(dao_id).unwrap().creator),
        dao_id,
        Box::new(proposal2)
    ));

    dao_id
}

pub fn project_join_reques() -> DaoAssetId {
    let dao_id = create_asset();
    assert!(wetee_project::Pallet::<Test>::project_join_request(
        RuntimeOrigin::signed(BOB),
        dao_id,
        PROJECT_INDEX,
        BOB,
    )
    .is_err());

    // 创建提案
    let proposal = RuntimeCall::WETEEProject(wetee_project::Call::project_join_request {
        dao_id,
        project_id: PROJECT_INDEX,
        who: BOB,
    });

    // 加入社区
    wetee_assets::Pallet::<Test>::join(RuntimeOrigin::signed(BOB), dao_id, 100, 10000).unwrap();

    // 加入社区后尝试提案
    assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
        RuntimeOrigin::signed(ALICE),
        dao_id,
        Box::new(proposal)
    ));

    let ms = wetee_org::ProjectMembers::<Test>::get(dao_id, PROJECT_INDEX);
    println!("项目成员 => {:?}", ms);
    assert!(ms.len() == 2);

    dao_id
}

#[test]
pub fn test_project_join_request() {
    new_test_run().execute_with(|| {
        project_join_reques();
    });
}

#[test]
pub fn test_task() {
    new_test_run().execute_with(|| {
        let dao_id = project_join_reques();

        // 为项目申请资金
        let proposal = RuntimeCall::WETEEProject(wetee_project::Call::apply_project_funds {
            dao_id,
            project_id: PROJECT_INDEX,
            amount: 19,
        });

        assert_ok!(wetee_sudo::Pallet::<Test>::sudo(
            RuntimeOrigin::signed(wetee_org::Daos::<Test>::get(dao_id).unwrap().creator),
            dao_id,
            Box::new(proposal)
        ));

        print_account(dao_id);

        // 创建任务
        wetee_project::Pallet::<Test>::create_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            "TestA".as_bytes().to_vec(),
            "TestA".as_bytes().to_vec(),
            10,
            1,
            Some(1),
            Some(vec![1]),
            Some(vec![]),
            Some(vec![]),
            10,
        )
        .unwrap();

        // 加入任务
        wetee_project::Pallet::<Test>::join_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 成为任务审核员
        wetee_project::Pallet::<Test>::join_task_review(
            RuntimeOrigin::signed(BOB),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 开始任务
        wetee_project::Pallet::<Test>::start_task(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 请求审查
        wetee_project::Pallet::<Test>::request_review(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        // 请求审查
        wetee_project::Pallet::<Test>::make_review(
            RuntimeOrigin::signed(BOB),
            dao_id,
            PROJECT_INDEX,
            1,
            wetee_project::ReviewOpinion::YES,
            "通过".as_bytes().to_vec(),
        )
        .unwrap();

        // 请求完成
        wetee_project::Pallet::<Test>::task_done(
            RuntimeOrigin::signed(ALICE),
            dao_id,
            PROJECT_INDEX,
            1,
        )
        .unwrap();

        print_account(dao_id);
    });
}

pub fn print_account(dao_id: DaoAssetId) {
    let project_dao = wetee_assets::Pallet::<Test>::get_balance(
        dao_id,
        wetee_org::Pallet::<Test>::dao_project(dao_id, PROJECT_INDEX),
    )
    .unwrap();

    let dao = wetee_assets::Pallet::<Test>::get_balance(
        dao_id,
        wetee_org::Pallet::<Test>::dao_account(dao_id),
    )
    .unwrap();

    // 判断现在项目的
    println!(
        "dao_id => {:?} project_dao => {:?} dao => {:?}",
        dao_id, project_dao, dao
    );

    let alice_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, ALICE).unwrap();
    let bob_dao = wetee_assets::Pallet::<Test>::get_balance(dao_id, BOB).unwrap();
    println!("alice_dao => {:?} ||| bob_dao => {:?} ", alice_dao, bob_dao);
}
