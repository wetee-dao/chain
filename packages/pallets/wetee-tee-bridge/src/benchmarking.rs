use super::*;
use crate::Pallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use wetee_primitives::types::{WorkId, WorkType};

#[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_tee_api() {
        #[block]
        {
            let _ = Pallet::<T>::set_tee_api(
                RawOrigin::Root.into(),
                WorkId::default(),
                ApiMeta {
                    port: 1,
                    apis: vec![
                        wetee_primitives::types::Api {
                            url: vec![0; 64],
                            method: 1,
                        },
                        wetee_primitives::types::Api {
                            url: vec![0; 64],
                            method: 1,
                        },
                    ],
                },
            );
        }
    }

    #[benchmark]
    fn ink_callback() {
        #[block]
        {
            let _ = Pallet::<T>::ink_callback(
                RawOrigin::Root.into(),
                1,
                1,
                vec![InkArg::Bool(true)],
                10u32.into(),
                None,
            );
        }
    }

    #[benchmark]
    fn call_ink() {
        #[block]
        {
            let _ = Pallet::<T>::call_ink(
                RawOrigin::Root.into(),
                WorkId {
                    wtype: WorkType::APP,
                    id: 0,
                },
                account("a", 1, 1),
                [0, 0, 0, 42],
                vec![InkArg::Bool(true)],
                10u32.into(),
            );
        }
    }

    #[benchmark]
    fn delete_call() {
        #[block]
        {
            let _ = Pallet::<T>::delete_call(RawOrigin::Root.into(), 1, 1);
        }
    }
}
