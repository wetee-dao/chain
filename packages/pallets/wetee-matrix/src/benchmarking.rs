use super::*;
use crate::{Config, Pallet as Org};
use frame_benchmarking::{
    account, benchmarks, benchmarks_instance, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin as SystemOrigin;
use wetee_primitives::{
    traits::UHook,
    types::{GuildId, ProjectId, TaskId, WeAssetId},
};
