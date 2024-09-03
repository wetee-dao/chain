use super::*;
use crate::{Call, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;

#[benchmarks( where <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create() {
        #[block]
        {
            let _ = Pallet::<T>::create(
                RawOrigin::Root.into(),
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![Service::Tcp(80)],
                Command::SH("".as_bytes().to_vec()),
                vec![EnvInput::default()],
                Some(EnvHash::default()),
                1,
                100,
                vec![Disk::default()],
                1,
                vec![Container::default(), Container::default()],
                1,
                TEEVersion::SGX,
            );
        }
    }

    #[benchmark]
    fn update() {
        #[block]
        {
            let _ = Pallet::<T>::update(
                RawOrigin::Root.into(),
                1,
                Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
                Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
                Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                Some(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                Some(vec![Service::Tcp(80)]),
                Some(Command::SH("".as_bytes().to_vec())),
                vec![EnvInput::default()],
                Some(EnvHash::default()),
                true,
            );
        }
    }

    #[benchmark]
    fn restart() {
        #[block]
        {
            let _ = Pallet::<T>::restart(RawOrigin::Root.into(), 1);
        }
    }

    #[benchmark]
    fn update_price() {
        #[block]
        {
            let _ = Pallet::<T>::update_price(
                RawOrigin::Root.into(),
                1,
                Price {
                    cpu_per: 100,
                    memory_per: 100,
                    disk_per: 100,
                },
            );
        }
    }
}
