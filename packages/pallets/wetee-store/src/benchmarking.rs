use crate::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks( where <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn register_app() {
        #[block]
        {
            let _ = Pallet::<T>::register_app(
                RawOrigin::Root.into(),
                vec![],
                crate::AppType::Service,
                vec![Default::default()],
                crate::RuntimeType::SGX,
            );
        }
    }

    #[benchmark]
    fn unregister_app() {
        #[block]
        {
            let _ = Pallet::<T>::unregister_app(RawOrigin::Root.into(), 0);
        }
    }

    #[benchmark]
    fn review_app() {
        #[block]
        {
            let _ = Pallet::<T>::review_app(RawOrigin::Root.into(), 0, WeAssetId::new(0, 0, 0, 0));
        }
    }

    #[benchmark]
    fn init_mint() {
        #[block]
        {
            let _ = Pallet::<T>::init_mint(RawOrigin::Root.into());
        }
    }
}
