// #![allow(dead_code)]
// #![allow(unused_variables)]

// use crate as wetee_treasury;
// use frame_support::{
//     parameter_types,
//     traits::{ConstU32, ConstU64, Contains},
//     PalletId,
// };
// use frame_system;
// use sp_core::H256;
// use sp_runtime::{
//     traits::{BlakeTwo256, IdentityLookup},
//     BuildStorage,
// };
// use sp_std::result::Result;

// type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
// pub type Block = frame_system::mocking::MockBlock<Test>;

// parameter_types! {
//     pub const DaoPalletId: PalletId = PalletId(*b"weteedao");
// }

// // Configure a mock runtime to test the pallet.
// frame_support::construct_runtime!(
//     pub enum Test
//     {
//         System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},

//         WETEE: wetee_org::{ Pallet, Call, Event<T>, Storage },
//         WeteeTreasury: wetee_treasury::{ Pallet, Call, Event<T>, Storage },
//     }
// );

// pub struct BlockEverything;
// impl Contains<RuntimeCall> for BlockEverything {
// 	fn contains(_: &RuntimeCall) -> bool {
// 		false
// 	}
// }

// impl frame_system::Config for Test {
// 	type BaseCallFilter = BlockEverything;
// 	type BlockWeights = ();
// 	type BlockLength = ();
// 	type DbWeight = ();
// 	type RuntimeOrigin = RuntimeOrigin;
// 	type RuntimeCall = RuntimeCall;
// 	type Nonce = u64;
// 	type Hash = H256;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = u64;
// 	type Lookup = IdentityLookup<Self::AccountId>;
// 	type Block = Block;
// 	type RuntimeEvent = RuntimeEvent;
// 	type BlockHashCount = ConstU64<250>;
// 	type Version = ();
// 	type PalletInfo = PalletInfo;
// 	type AccountData = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount = ();
// 	type SystemWeightInfo = ();
// 	type SS58Prefix = ();
// 	type OnSetCode = ();
// 	type MaxConsumers = ConstU32<16>;
// }

// impl TryFrom<RuntimeCall> for u64 {
//     type Error = ();
//     fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
//         match call {
//             _ => Ok(0u64),
//         }
//     }
// }

// impl wetee_org::Config for Test {
//     type RuntimeEvent = RuntimeEvent;
//     type RuntimeCall = RuntimeCall;
//     type CallId = u64;
//     type PalletId = DaoPalletId;
//     type UHook = ();
//     type WeightInfo = ();
//     type MaxMembers = ConstU32<1000000>;
// }

// impl wetee_worker::Config for Test {
//     type RuntimeEvent = RuntimeEvent;
//     type WeightInfo = ();
// }

// pub fn new_test_run() -> sp_io::TestExternalities {
//     let t = frame_system::GenesisConfig::<Test>::default()
//     .build_storage()
//     .unwrap();
//     t.into()
// }
