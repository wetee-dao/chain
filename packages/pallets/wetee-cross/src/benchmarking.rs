use super::*;
use crate::Pallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks( where <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn cross_transfer_from() {
        #[block]
        {
            let _ = Pallet::<T>::cross_transfer_from(RawOrigin::Root.into());
        }
    }
}
