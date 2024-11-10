use super::*;
use crate::{Call, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use wetee_primitives::types::{Ip, WeAssetId};

#[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn register_node() {
        #[block]
        {
            let _ = Pallet::<T>::register_node(RawOrigin::Root.into(), account("a", 1, 1));
        }
    }

    #[benchmark]
    fn upload_code() {
        #[block]
        {
            let _ = Pallet::<T>::upload_code(RawOrigin::Root.into(), vec![0; 64], vec![0; 64]);
        }
    }

    #[benchmark]
    fn upload_cluster_proof() {
        let (caller_public, caller) = T::Helper::signer();
        let sig = T::Helper::sign(&caller_public, b"xx");
        #[block]
        {
            let _ = Pallet::<T>::upload_cluster_proof(
                RawOrigin::Root.into(),
                1,
                vec![0; 32],
                vec![
                    account("a", 1, 1),
                    account("a", 1, 1),
                    account("a", 1, 1),
                    account("a", 1, 1),
                    account("a", 1, 1),
                ],
                vec![
                    sig.clone(),
                    sig.clone(),
                    sig.clone(),
                    sig.clone(),
                    sig.clone(),
                ],
            );
        }
    }

    #[benchmark]
    fn work_launch() {
        #[block]
        {
            let _ = Pallet::<T>::work_launch(
                RawOrigin::Root.into(),
                WorkId::default(),
                Some(vec![0; 32]),
                account("a", 1, 1),
            );
        }
    }

    #[benchmark]
    fn set_node_pub_server() {
        #[block]
        {
            let _ = Pallet::<T>::set_node_pub_server(
                RawOrigin::Root.into(),
                1,
                P2PAddr {
                    ip: Ip::default(),
                    port: 1,
                    id: account("a", 1, 1),
                },
            );
        }
    }
}
