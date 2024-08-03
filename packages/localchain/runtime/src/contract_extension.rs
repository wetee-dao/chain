use crate::{Runtime, WeTEEBridge};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{ConstU32, Randomness};
use log::{error, trace};
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig,
};
use scale_info::prelude::vec::Vec;
use sp_core::crypto::UncheckedFrom;
use sp_runtime::{AccountId32, BoundedVec, DispatchError};

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
        // get func id
        let func_id = env.func_id();
        // get contract address
        let origin = env.ext().address().clone();
        // get contract sender
        let binding = env.ext().caller().clone();
        let sender = binding.account_id()?;
        match func_id {
            // call tee from ink
            1001 => {
                let mut env = env.buf_in_buf_out();

                // read input
                let input: TEECallInput = env.read_as()?;

                // call tee bridge
                let id = WeTEEBridge::call_from_ink(
                    origin,
                    sender.clone(),
                    input.tee,
                    input.method,
                    input.callback_method,
                    input.args.into(),
                )
                .unwrap();

                // return call id
                env.write(&id.encode(), false, None)
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

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
struct TEECallInput {
    pub tee: WorkId,
    pub method: u16,
    pub callback_method: [u8; 4],
    pub args: [u8; 256],
}
