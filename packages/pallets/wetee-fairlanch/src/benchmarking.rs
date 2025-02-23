use super::*;
use crate::{Call, Pallet};
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;

#[benchmarks( where <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn v_staking() {
        #[block]
        {
            let t: BalanceOf<T> = 1000u32.into();
            let _ = Pallet::<T>::v_staking(RawOrigin::Root.into(), 5000, t);
        }
    }

    #[benchmark]
    fn v_unstaking() {
        #[block]
        {
            let t: BalanceOf<T> = 1000u32.into();
            let _ = Pallet::<T>::v_unstaking(RawOrigin::Root.into(), 5000, t);
        }
    }

    #[benchmark]
    fn set_economics() {
        #[block]
        {
            let _ = Pallet::<T>::set_economics(RawOrigin::Root.into(), 5000, 10, 0u32.into());
        }
    }

    #[benchmark]
    fn register_vtoken() {
        #[block]
        {
            let _ = Pallet::<T>::register_vtoken(RawOrigin::Root.into(), 5000, 50001, 10, 12);
        }
    }

    #[benchmark]
    fn set_vtoken_rate() {
        #[block]
        {
            let _ = Pallet::<T>::set_vtoken_rate(RawOrigin::Root.into(), 5000, 50001, 10, 12);
        }
    }

    #[benchmark]
    fn delete_economics() {
        #[block]
        {
            let _ = Pallet::<T>::delete_economics(RawOrigin::Root.into(), 5000);
        }
    }

    #[benchmark]
    fn v_staking_cancel() {
        #[block]
        {
            let t: BalanceOf<T> = 1000u32.into();
            let _ = Pallet::<T>::v_staking_cancel(RawOrigin::Root.into(), 5000, t);
        }
    }

    #[benchmark]
    fn set_epoch() {
        #[block]
        {
            let _ = Pallet::<T>::set_epoch(RawOrigin::Root.into());
        }
    }
}
