use crate::{Runtime, WeTEEBridge};
use codec::Encode;
use frame_support::traits::Randomness;
use log::{error, trace};
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{AccountId32, DispatchError};
use wetee_primitives::types::{WorkId, WorkType};

/// Contract extension for `Ink`
#[derive(Default)]
pub struct TeeExtension;

impl ChainExtension<Runtime> for TeeExtension {
    fn call<E: Ext>(&mut self, mut env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
        E: Ext<T = Runtime>,
    {
        let func_id = env.func_id();
        let origin = env.ext().address().clone();
        match func_id {
            1001 => {
                let mut env = env.buf_in_buf_out();
                let arg: [u8; 32] = env.read_as()?;

                WeTEEBridge::call_from_ink(
                    origin,
                    WorkId {
                        wtype: WorkType::APP,
                        id: 1,
                    },
                    1,
                    arg.to_vec(),
                )
                .unwrap();

                let random_seed = crate::RandomnessCollectiveFlip::random(&arg).0;
                let random_slice = random_seed.encode();
                trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&random_slice, false, None)
                    .map_err(|_| DispatchError::Other("ChainExtension failed to call random"))?;
            }

            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
