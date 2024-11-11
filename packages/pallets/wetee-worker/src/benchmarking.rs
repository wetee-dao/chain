use super::*;
use crate::{Ip, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;

#[benchmarks( where <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn cluster_register() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_register(
                RawOrigin::Root.into(),
                vec![1u8; 32],
                vec![
                    Ip {
                        ipv4: Some(0),
                        ipv6: Some(0),
                        domain: Some(vec![0u8; 40]),
                    },
                    Ip {
                        ipv4: Some(0),
                        ipv6: Some(0),
                        domain: Some(vec![0u8; 40]),
                    },
                ],
                1,
                1,
            );
        }
    }

    #[benchmark]
    fn cluster_mortgage() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_mortgage(
                RawOrigin::Root.into(),
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                100u32.into(),
            );
        }
    }

    #[benchmark]
    fn cluster_unmortgage() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_unmortgage(RawOrigin::Root.into(), 1, 1u32.into());
        }
    }

    #[benchmark]
    fn work_proof_upload() {
        #[block]
        {
            let _ = Pallet::<T>::work_proof_upload(
                RawOrigin::Root.into(),
                WorkId::default(),
                Some(ProofOfWork::default()),
                Some(vec![1u8; 64]),
            );
        }
    }

    #[benchmark]
    fn cluster_withdrawal() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_withdrawal(
                RawOrigin::Root.into(),
                WorkId::default(),
                1u32.into(),
            );
        }
    }

    #[benchmark]
    fn cluster_stop() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_stop(RawOrigin::Root.into(), 1);
        }
    }

    #[benchmark]
    fn cluster_report() {
        #[block]
        {
            let _ = Pallet::<T>::cluster_report(
                RawOrigin::Root.into(),
                1,
                WorkId::default(),
                vec![1u8; 64],
            );
        }
    }

    #[benchmark]
    fn report_close() {
        #[block]
        {
            let _ = Pallet::<T>::report_close(RawOrigin::Root.into(), 1, WorkId::default());
        }
    }

    #[benchmark]
    fn set_boot_peers() {
        #[block]
        {
            let _ = Pallet::<T>::set_boot_peers(
                RawOrigin::Root.into(),
                vec![
                    P2PAddr {
                        ip: Ip::default(),
                        port: 1,
                        id: account("a", 1, 1),
                    },
                    P2PAddr {
                        ip: Ip::default(),
                        port: 1,
                        id: account("a", 1, 1),
                    },
                ],
            );
        }
    }

    #[benchmark]
    fn set_stage() {
        #[block]
        {
            let _ = Pallet::<T>::set_stage(RawOrigin::Root.into(), 1);
        }
    }

    #[benchmark]
    fn upload_code() {
        #[block]
        {
            let _ = Pallet::<T>::upload_code(RawOrigin::Root.into(), vec![1u8; 64], vec![1u8; 64]);
        }
    }

    #[benchmark]
    fn work_stop() {
        #[block]
        {
            let _ = Pallet::<T>::work_stop(RawOrigin::Root.into(), WorkId::default());
        }
    }
}
