#![cfg_attr(not(feature = "std"), no_std)]
use super::*;
use frame_benchmarking::v2::*;
use wetee_primitives::{
    traits::AfterCreate,
    types::{DaoAssetId, GuildId, ProjectId, TaskId},
};
use frame_support::codec::{Decode, Encode};
use frame_support::traits::UnfilteredDispatchable;
use scale_info::TypeInfo;
use frame_system::{RawOrigin,Pallet as System};
use sp_runtime::RuntimeDebug;
use scale_info::prelude::boxed::Box;
use wetee_org::{self};

fn get_alice<T: Config>() -> T::AccountId {
	account("alice", 1, 1)
}

fn creat_dao<T: Config>(init:bool) -> (DaoAssetId, DaoAssetId) {
	let alice = get_alice::<T>();
	let dao_id = 5000;
	wetee_org::Pallet::<T>::create_dao(
		RawOrigin::Signed(alice.clone()).into(),
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
		vec![1; 4],
  ).unwrap();

  // let value: BalanceOf<T> = 10000u32.into();
  // let value2: BalanceOf<T> = 100u32.into();
  // wetee_assets::Pallet::<T>::create_asset(
  //   RawOrigin::Signed(alice.clone()).into(),
  //   dao_id,
  //   wetee_assets::DaoAssetMeta {
  //       name: "TESTA".as_bytes().to_vec(),
  //       symbol: "TA".as_bytes().to_vec(),
  //       decimals: 10,
  //   },
  //   value,
  //   value2,
  // )
  // .unwrap();

  let _v1: BalanceOf<T> = 1u32.into();
  let _v10: BlockNumberFor<T> = 10u32.into();
  let _v100: BlockNumberFor<T> = 100u32.into();
  Pallet::<T>::set_periods(
    RawOrigin::Signed(wetee_org::Pallet::<T>::dao_approve(dao_id)).into(),
    dao_id,
    vec![Period{ 
      name: "gov".into(), 
      pallet_index: 4, 
      decision_deposit: _v1, 
      prepare_period: _v10.clone(), 
      max_deciding: _v100, 
      confirm_period: _v10.clone(), 
      decision_period: _v10.clone(), 
      min_enactment_period: _v10.clone(), 
      min_approval: 1, 
      min_support: 1,
    }]
  );
  
  if init_proposal {
    let proposal: <T as wetee_org::Config>::RuntimeCall  = wetee_org::Call::create_roadmap_task {
      dao_id,
      roadmap_id: 202301,
      name: vec![1; 4],
      priority: 1,
      tags: Some(vec![1].into()),
    }.into();

    Pallet::<T>::submit_proposal(RawOrigin::Signed(alice.clone()).into(),5000,MemberData::GLOBAL,Box::new(proposal.clone()),0);
  }

	(dao_id, 0)
}

#[benchmarks( where 
  <T as wetee_org::Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

  #[benchmark]
  fn submit_proposal(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    let proposal: <T as wetee_org::Config>::RuntimeCall  = wetee_org::Call::create_roadmap_task {
      dao_id,
      roadmap_id: 202301,
      name: vec![1; 4],
      priority: 1,
      tags: Some(vec![1].into()),
    }.into();
  
    #[extrinsic_call]
    _(RawOrigin::Signed(alice.clone()),5000,MemberData::GLOBAL,Box::new(proposal.clone()),0);
  }

  // #[benchmark]
  // fn deposit_proposal(){
  //   let (dao_id, _) = creat_dao::<T>(true);
  //   let alice = get_alice::<T>();
  //   let _v1: BalanceOf<T> = 100u32.into();

  //   let _v10: BlockNumberFor<T> = 10u32.into();
  //   System::<T>::set_block_number(_v10);
  //   #[extrinsic_call]
  //   _(
  //     RawOrigin::Signed(alice),
  //     5000,
  //     0,
  //     _v1
  //   )
  // }
}