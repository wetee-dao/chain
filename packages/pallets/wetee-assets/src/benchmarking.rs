use super::*;
use crate::{ Config, AssetMeta, Pallet as Asset};
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use wetee_primitives::types::WeAssetId;

fn creat_dao<T: Config>() -> (WeAssetId, WeAssetId) {
    let caller = whitelisted_caller();
    let dao_id = WeAssetId::default();
    let second_id: WeAssetId = Default::default();
    assert!(wetee_org::Pallet::<T>::create_dao(
        RawOrigin::Signed(caller).into(),
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
    .is_ok());
    (dao_id, second_id)
}

#[benchmarks( where 
    <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>,
    T: pallet_balances::Config
)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_asset() {
        let caller = whitelisted_caller();
        let (_dao_id, _second_id) = creat_dao::<T>();
        let amount: BalanceOf<T> = 10u32.into();
        let init_dao_asset: BalanceOf<T> = 9u32.into();

        #[block]
		{
            Asset::<T>::create_asset(
                RawOrigin::Signed(caller).into(),
                5000,
                AssetMeta {
                    name: "TestA".as_bytes().to_vec(),
                    symbol: "TA".as_bytes().to_vec(),
                    decimals: 10,
                },
                amount,
                init_dao_asset,
            );
        }
    }

    #[benchmark]
    fn set_existenial_deposit() {
        let caller = whitelisted_caller();
        let caller1 = whitelisted_caller();
        let (_dao_id, _second_id) = creat_dao::<T>();
        let amount: BalanceOf<T> = 10u32.into();
        let init_dao_asset: BalanceOf<T> = 9u32.into();

        Asset::<T>::create_asset(
            RawOrigin::Signed(caller).into(),
            5000,
            AssetMeta {
                name: "TestA".as_bytes().to_vec(),
                symbol: "TA".as_bytes().to_vec(),
                decimals: 10,
            },
            amount,
            init_dao_asset,
        );

        #[block]
		{

            Asset::<T>::set_existenial_deposit(
                RawOrigin::Signed(caller1).into(),
                5000,
                amount,
            );
        }
    }

    #[benchmark]
    fn set_metadata() {
        let caller = whitelisted_caller();
        let caller1 = whitelisted_caller();
        let (_dao_id, _second_id) = creat_dao::<T>();
        let amount: BalanceOf<T> = 10u32.into();
        let init_dao_asset: BalanceOf<T> = 9u32.into();

        Asset::<T>::create_asset(
            RawOrigin::Signed(caller).into(),
            5000,
            AssetMeta {
                name: "TestA".as_bytes().to_vec(),
                symbol: "TA".as_bytes().to_vec(),
                decimals: 10,
            },
            amount,
            init_dao_asset,
        );

        #[block]
        {
            Asset::<T>::set_metadata(
                RawOrigin::Signed(caller1).into(),
                5000,
                AssetMeta {
                    name: "TestA".as_bytes().to_vec(),
                    symbol: "TA".as_bytes().to_vec(),
                    decimals: 10,
                },
            );
        }   
    }
    
    #[benchmark]
    fn burn(){
        let caller = whitelisted_caller();
        let caller1 = whitelisted_caller();
        let (_dao_id, _second_id) = creat_dao::<T>();
        let amount: BalanceOf<T> = 10u32.into();
        let init_dao_asset: BalanceOf<T> = 9u32.into();

        Asset::<T>::create_asset(
            RawOrigin::Signed(caller).into(),
            5000,
            AssetMeta {
                name: "TestA".as_bytes().to_vec(),
                symbol: "TA".as_bytes().to_vec(),
                decimals: 10,
            },
            amount,
            init_dao_asset,
        );
        
        let t: BalanceOf<T> = 1u32.into();
        #[block]
        {
            Asset::<T>::burn(
                RawOrigin::Signed(caller1).into(),
                5000,
                t,
            );
        }
    }

    #[benchmark]
    fn transfer() {
        let caller = whitelisted_caller();
        let caller1 = whitelisted_caller();
        let (_dao_id, _second_id) = creat_dao::<T>();
        let amount: BalanceOf<T> = 10u32.into();
        let init_dao_asset: BalanceOf<T> = 9u32.into();

        Asset::<T>::create_asset(
            RawOrigin::Signed(caller).into(),
            5000,
            AssetMeta {
                name: "TestA".as_bytes().to_vec(),
                symbol: "TA".as_bytes().to_vec(),
                decimals: 10,
            },
            amount,
            init_dao_asset,
        );

        let dist = account("dist",2,2);
        let t: BalanceOf<T> = 1u32.into();
        let dist_lookup = T::Lookup::unlookup(dist);
        #[block]
		{
            Asset::<T>::transfer(
                RawOrigin::Signed(caller1).into(),
                dist_lookup,
                5000,
                t,
            );
        }
    }

}