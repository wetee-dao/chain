#![cfg_attr(not(feature = "std"), no_std)]
use super::*;
use frame_benchmarking::v2::*;
use wetee_primitives::{
    traits::UHook,
    types::{WeAssetId, GuildId, ProjectId, TaskId},
};
use parity_scale_codec::{Decode, Encode};
use frame_support::traits::UnfilteredDispatchable;
use scale_info::TypeInfo;
use frame_system::{RawOrigin,Pallet as System};
use sp_runtime::RuntimeDebug;
use scale_info::prelude::boxed::Box;
use scale_info::prelude::vec;
use wetee_dao::{self};

fn get_alice<T: Config>() -> T::AccountId {
	account("alice", 1, 1)
}

fn creat_dao<T: Config>(init:bool) -> (WeAssetId, WeAssetId) {
	let alice = get_alice::<T>();
	let dao_id = 5000;
	wetee_dao::Pallet::<T>::create_dao(
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

  let value: BalanceOf<T> = 10000u32.into();
  let value2: BalanceOf<T> = 100u32.into();
  wetee_assets::Pallet::<T>::create_asset(
    RawOrigin::Signed(alice.clone()).into(),
    dao_id,
    wetee_assets::DaoAssetMeta {
        name: "TESTA".as_bytes().to_vec(),
        symbol: "TA".as_bytes().to_vec(),
        decimals: 10,
    },
    value,
    value2,
  );

  let _v1: BalanceOf<T> = 1u32.into();
  let _v10: BlockNumberFor<T> = 10u32.into();
  let _v100: BlockNumberFor<T> = 100u32.into();
  let max_balance: BalanceOf<T> = 10000u32.into();
  Pallet::<T>::set_periods(
    RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
    dao_id,
    vec![Period{ 
      name: "gov".into(), 
      pallet_index: 1, 
      decision_deposit: _v1, 
      prepare_period: _v10.clone(), 
      max_deciding: _v100, 
      confirm_period: _v10.clone(), 
      decision_period: _v10.clone(), 
      min_enactment_period: _v10.clone(), 
      min_approval: 1, 
      min_support: 1,
      max_balance: max_balance,
    }]
  );
  
  
  let proposal: <T as wetee_dao::Config>::RuntimeCall  = wetee_dao::Call::create_roadmap_task {
    dao_id,
    roadmap_id: 202301,
    name: vec![1; 4],
    priority: 1,
    tags: Some(vec![1].into()),
  }.into();

  Pallet::<T>::submit_proposal(RawOrigin::Signed(alice.clone()).into(),5000,MemberData::GLOBAL,Box::new(proposal.clone()),0);

	(dao_id, 0)
}

#[benchmarks( where 
  <T as wetee_dao::Config>::RuntimeCall: From<frame_system::Call<T>>,
)]
mod benchmarks {
	use super::*;

  #[benchmark]
  fn submit_proposal(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    let proposal: <T as wetee_dao::Config>::RuntimeCall  = wetee_dao::Call::create_roadmap_task {
      dao_id,
      roadmap_id: 202301,
      name: vec![1; 4],
      priority: 1,
      tags: Some(vec![1].into()),
    }.into();
  
    #[extrinsic_call]
    _(RawOrigin::Signed(alice.clone()),5000,MemberData::GLOBAL,Box::new(proposal.clone()),0);
  }

  #[benchmark]
  fn set_periods(){
    let (dao_id, _) = creat_dao::<T>(false);
    let _v1: BalanceOf<T> = 1u32.into();
    let _v10: BlockNumberFor<T> = 10u32.into();
    let _v100: BlockNumberFor<T> = 100u32.into();
    let max_balance: BalanceOf<T> = 10000u32.into();

    #[block]{
      Pallet::<T>::set_periods(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
        vec![Period{ 
          name: "gov".into(), 
          pallet_index: 1, 
          decision_deposit: _v1, 
          prepare_period: _v10.clone(), 
          max_deciding: _v100, 
          confirm_period: _v10.clone(), 
          decision_period: _v10.clone(), 
          min_enactment_period: _v10.clone(), 
          min_approval: 1, 
          min_support: 1,
          max_balance: max_balance,
        }]
      );
    }
  }

  #[benchmark]
  fn update_vote_model(){
    let (dao_id, _) = creat_dao::<T>(false);
    
    #[block]{
      Pallet::<T>::update_vote_model(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
        1,
      );
    }
  }

  #[benchmark]
  fn set_max_pre_props(){
    let (dao_id, _) = creat_dao::<T>(false);

    #[block]{
      Pallet::<T>::set_max_pre_props(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
        100,
      );
    }
  }

  #[benchmark]
  fn deposit_proposal(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    frame_system::Pallet::<T>::set_block_number(80u32.into());

    let deposit: BalanceOf<T> = 10000u32.into();

    #[block]{
      Pallet::<T>::deposit_proposal(
        RawOrigin::Signed(alice.clone()).into(),
        dao_id,
        0,
        deposit,
      );
    }
  }

  #[benchmark]
  fn vote_for_prop(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    frame_system::Pallet::<T>::set_block_number(80u32.into());

    #[block]{
      Pallet::<T>::vote_for_prop(
        RawOrigin::Signed(alice.into()).into(),
        dao_id,
        0,
        100u32.into(),
        Opinion::YES,
      );
    }
  }

  #[benchmark]
  fn cancel_vote(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    frame_system::Pallet::<T>::set_block_number(80u32.into());

    Pallet::<T>::vote_for_prop(
      RawOrigin::Signed(alice.into()).into(),
      dao_id,
      0,
      100u32.into(),
      Opinion::YES,
    );

    #[block]{
      Pallet::<T>::cancel_vote(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
        0,
      );
    }
  }

  #[benchmark]
  fn run_proposal(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    frame_system::Pallet::<T>::set_block_number(80u32.into());

    Pallet::<T>::vote_for_prop(
      RawOrigin::Signed(alice.into()).into(),
      dao_id,
      0,
      100u32.into(),
      Opinion::YES,
    );

    #[block]{
      Pallet::<T>::run_proposal(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
        0,
      );
    }
  }

  #[benchmark]
  fn unlock(){
    let (dao_id, _) = creat_dao::<T>(false);
    let alice = get_alice::<T>();
    frame_system::Pallet::<T>::set_block_number(80u32.into());

    Pallet::<T>::vote_for_prop(
      RawOrigin::Signed(alice.into()).into(),
      dao_id,
      0,
      100u32.into(),
      Opinion::YES,
    );

    
    Pallet::<T>::run_proposal(
      RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
      dao_id,
      0,
    );
    
    #[block]{
      Pallet::<T>::unlock(
        RawOrigin::Signed(wetee_dao::Pallet::<T>::dao_approve(dao_id,0)).into(),
        dao_id,
      );
    }
  }
}