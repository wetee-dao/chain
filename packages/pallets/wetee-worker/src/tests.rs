#![allow(unused_imports)]
#![cfg(test)]

use super::*;
use crate as wetee_worker;
use crate::mock::{RuntimeCall, *};
use frame_support::{assert_noop, assert_ok, debug};

pub fn create_cluster() {
    DepositPrices::<Test>::insert(
        1,
        DepositPrice {
            cpu_per: 10,
            memory_per: 10,
            disk_per: 10,
        },
    );
    Pallet::<Test>::cluster_register(
        OriginFor::<Test>::signed(ALICE),
        "test".as_bytes().to_vec(),
        vec![Ip {
            ipv4: Some(2130706433),
            ipv6: None,
        }],
        8080,
        1,
    )
    .unwrap();
}

pub fn create_work() {
    wetee_app::Prices::<Test>::insert(
        1,
        wetee_app::Price {
            cpu_per: 1,
            memory_per: 1,
            disk_per: 1,
        },
    );
    wetee_app::Pallet::<Test>::create(
        OriginFor::<Test>::signed(ALICE),
        "test".as_bytes().to_vec(),
        "test".as_bytes().to_vec(),
        vec![1, 2, 3],
        1,
        1,
        1,
        1,
        1000,
    )
    .unwrap();
}

pub fn mortgage() {
    // 质押
    assert!(Pallet::<Test>::cluster_mortgage(
        OriginFor::<Test>::signed(ALICE),
        0,
        10,
        10,
        10,
        1000
    )
    .is_ok());
}

pub fn start() {
    let work_id = WorkId { t: 1, id: 0 };
    Pallet::<Test>::match_app_deploy(ALICE.clone(), work_id.clone(), None).unwrap();
    frame_system::Pallet::<Test>::set_block_number(631);
    let res = Pallet::<Test>::work_proof_upload(
        OriginFor::<Test>::signed(ALICE),
        work_id,
        ProofOfWork {
            log_hash: "test".as_bytes().to_vec(),
            cr: Cr {
                cpu: 1,
                memory: 1,
                disk: 1,
            },
            cr_hash: "test".as_bytes().to_vec(),
            public_key: "test".as_bytes().to_vec(),
        },
    );
    assert!(res.is_ok());
}

#[test]
pub fn cluster_register() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(ALICE),
            "test".as_bytes().to_vec(),
            vec![Ip {
                ipv4: Some(2130706433),
                ipv6: None,
            }],
            8080,
            1,
        )
        .is_ok());
    });
}

// 没有ip
#[test]
pub fn cluster_register_should_fail() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![],
            8080,
            1,
        )
        .is_err());
    });
}

// ip格式错误
#[test]
pub fn cluster_register_should_fail2() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![Ip {
                ipv4: Some(2130706433),
                ipv6: None,
            }],
            8080,
            1,
        )
        .is_err());
    });
}

// 没有端口
#[test]
pub fn cluster_register_should_fail3() {
    new_test_run().execute_with(|| {
        assert!(Pallet::<Test>::cluster_register(
            OriginFor::<Test>::signed(BOB),
            "test".as_bytes().to_vec(),
            vec![Ip {
                ipv4: Some(2130706433),
                ipv6: None,
            }],
            0,
            1,
        )
        .is_err());
    });
}

#[test]
pub fn cluster_mortgage() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
    });
}

// 非集群所有者
#[test]
pub fn cluster_mortgage_should_fail() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(
            Pallet::<Test>::cluster_mortgage(OriginFor::<Test>::signed(BOB), 0, 1, 1, 1, 100)
                .is_err()
        );
    });
}

// 余额不够
#[test]
pub fn cluster_mortgage_should_fail2() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(
            Pallet::<Test>::cluster_mortgage(OriginFor::<Test>::signed(ALICE), 0, 1, 1, 1, 0)
                .is_err()
        );
    });
}

#[test]
pub fn cluster_unmortgage() {
    new_test_run().execute_with(|| {
        create_cluster();
        frame_system::Pallet::<Test>::set_block_number(30);
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
        frame_system::Pallet::<Test>::set_block_number(31);
        assert!(
            Pallet::<Test>::cluster_unmortgage(OriginFor::<Test>::signed(ALICE), 0, 30).is_ok()
        );
    });
}

// 非集群所有者
#[test]
pub fn cluster_unmortgage_should_fail() {
    new_test_run().execute_with(|| {
        create_cluster();
        frame_system::Pallet::<Test>::set_block_number(30);
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
        frame_system::Pallet::<Test>::set_block_number(31);
        assert!(Pallet::<Test>::cluster_unmortgage(OriginFor::<Test>::signed(BOB), 0, 30).is_err());
    });
}

// id 错误
#[test]
pub fn cluster_unmortgage_should_fail2() {
    new_test_run().execute_with(|| {
        create_cluster();
        frame_system::Pallet::<Test>::set_block_number(30);
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
        frame_system::Pallet::<Test>::set_block_number(31);
        assert!(
            Pallet::<Test>::cluster_unmortgage(OriginFor::<Test>::signed(ALICE), 1, 30).is_err()
        );
    });
}

// 块高度错误
#[test]
pub fn cluster_unmortgage_should_fail3() {
    new_test_run().execute_with(|| {
        create_cluster();
        frame_system::Pallet::<Test>::set_block_number(30);
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());
        frame_system::Pallet::<Test>::set_block_number(31);
        assert!(
            Pallet::<Test>::cluster_unmortgage(OriginFor::<Test>::signed(ALICE), 0, 10).is_err()
        );
    });
}

#[test]
pub fn cluster_proof_upload() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_mortgage(
            OriginFor::<Test>::signed(ALICE),
            0,
            1,
            1,
            1,
            100
        )
        .is_ok());

        assert!(Pallet::<Test>::cluster_proof_upload(
            OriginFor::<Test>::signed(ALICE),
            0,
            ProofOfCluster {
                public_key: "test".as_bytes().to_vec()
            },
        )
        .is_ok());
    });
}

#[test]
pub fn cluster_proof_upload_should_fail() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_proof_upload(
            OriginFor::<Test>::signed(BOB),
            0,
            ProofOfCluster {
                public_key: "test".as_bytes().to_vec()
            }
        )
        .is_err());
    });
}

#[test]
pub fn cluster_proof_upload_should_fail2() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_proof_upload(
            OriginFor::<Test>::signed(ALICE),
            1,
            ProofOfCluster {
                public_key: "test".as_bytes().to_vec()
            }
        )
        .is_err());
    });
}

#[test]
pub fn cluster_stop() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_stop(OriginFor::<Test>::signed(ALICE), 0).is_ok());
    });
}

#[test]
pub fn cluster_stop_should_fail() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_stop(OriginFor::<Test>::signed(BOB), 0).is_err());
    });
}

#[test]
pub fn cluster_stop_should_fail2() {
    new_test_run().execute_with(|| {
        create_cluster();
        assert!(Pallet::<Test>::cluster_stop(OriginFor::<Test>::signed(ALICE), 1).is_err());
    });
}

#[test]
pub fn work_proof_upload() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        let work_id = WorkId { t: 1, id: 0 };
        Pallet::<Test>::match_app_deploy(ALICE.clone(), work_id.clone(), None).unwrap();
        frame_system::Pallet::<Test>::set_block_number(631);
        let res = Pallet::<Test>::work_proof_upload(
            OriginFor::<Test>::signed(ALICE),
            work_id,
            ProofOfWork {
                log_hash: "test".as_bytes().to_vec(),
                cr: Cr {
                    cpu: 1,
                    memory: 1,
                    disk: 1,
                },
                cr_hash: "test".as_bytes().to_vec(),
                public_key: "test".as_bytes().to_vec(),
            },
        );
        assert!(res.is_ok());
    });
}

// 未开始的工作
#[test]
pub fn work_proof_upload_should_fail() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        let work_id = WorkId { t: 1, id: 0 };
        // Pallet::<Test>::match_app_deploy(ALICE.clone(), work_id.clone(), None).unwrap();
        frame_system::Pallet::<Test>::set_block_number(631);
        let res = Pallet::<Test>::work_proof_upload(
            OriginFor::<Test>::signed(ALICE),
            work_id,
            ProofOfWork {
                log_hash: "test".as_bytes().to_vec(),
                cr: Cr {
                    cpu: 1,
                    memory: 1,
                    disk: 1,
                },
                cr_hash: "test".as_bytes().to_vec(),
                public_key: "test".as_bytes().to_vec(),
            },
        );
        assert!(res.is_err());
    });
}

// 未到达工作完成时间
#[test]
pub fn work_proof_upload_should_fail2() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        let work_id = WorkId { t: 1, id: 0 };
        Pallet::<Test>::match_app_deploy(ALICE.clone(), work_id.clone(), None).unwrap();
        // frame_system::Pallet::<Test>::set_block_number(631);
        let res = Pallet::<Test>::work_proof_upload(
            OriginFor::<Test>::signed(ALICE),
            work_id,
            ProofOfWork {
                log_hash: "test".as_bytes().to_vec(),
                cr: Cr {
                    cpu: 1,
                    memory: 1,
                    disk: 1,
                },
                cr_hash: "test".as_bytes().to_vec(),
                public_key: "test".as_bytes().to_vec(),
            },
        );
        assert!(res.is_err());
    });
}

#[test]
pub fn cluster_withdrawal() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        start();
        let work_id = WorkId { t: 1, id: 0 };
        let res = Pallet::<Test>::cluster_withdrawal(OriginFor::<Test>::signed(ALICE), work_id, 10);
        assert!(res.is_ok());
    });
}

#[test]
pub fn cluster_report() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        start();
        let work_id = WorkId { t: 1, id: 0 };
        let res = Pallet::<Test>::cluster_report(
            OriginFor::<Test>::signed(ALICE),
            0,
            work_id,
            "test".as_bytes().to_vec(),
        );
        assert!(res.is_ok());
    });
}

#[test]
pub fn cluster_report_should_fail() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        start();
        let work_id = WorkId { t: 1, id: 0 };
        let mut vec: Vec<u8> = Vec::with_capacity(256);
        vec.resize(256, 0);
        let res = Pallet::<Test>::cluster_report(OriginFor::<Test>::signed(ALICE), 0, work_id, vec);
        assert!(res.is_err());
    });
}

#[test]
pub fn cluster_report_close() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        start();

        let work_id = WorkId { t: 1, id: 0 };
        let res = Pallet::<Test>::cluster_report(
            OriginFor::<Test>::signed(ALICE),
            0,
            work_id.clone(),
            "test".as_bytes().to_vec(),
        );
        assert!(res.is_ok());

        assert!(Pallet::<Test>::report_close(OriginFor::<Test>::signed(ALICE), 0, work_id).is_ok());
    });
}

// 非举报人员
#[test]
pub fn cluster_report_close_should_fail() {
    new_test_run().execute_with(|| {
        frame_system::Pallet::<Test>::set_block_number(1);
        create_cluster();
        create_work();
        mortgage();
        start();

        let work_id = WorkId { t: 1, id: 0 };
        let res = Pallet::<Test>::cluster_report(
            OriginFor::<Test>::signed(ALICE),
            0,
            work_id.clone(),
            "test".as_bytes().to_vec(),
        );
        assert!(res.is_ok());

        assert!(Pallet::<Test>::report_close(OriginFor::<Test>::signed(BOB), 0, work_id).is_err());
    });
}
