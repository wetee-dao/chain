use codec::{self};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_runtime::traits::{Block as BlockT};
use std::{sync::Arc};
use wetee_primitives::types::DaoAssetId;

pub use wetee_runtime_api::WeteeAssetRuntimeApi;

#[rpc(client, server)]
pub trait WeteeAssetApi<Block, AccountId, Balance> {
    #[method(name = "wetee_assetBalance")]
    fn get_asset_balance(&self, dao_id: DaoAssetId, who: AccountId) -> RpcResult<Balance>;
}

pub struct WeteeAsset<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> WeteeAsset<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

// impl<C, Block, AccountId, Balance> WeteeAssetApiServer<<Block as BlockT>::Hash, AccountId, Balance>
//     for WeteeAsset<C, Block>
// where
//     Block: BlockT,
//     AccountId: Clone + Display + Codec + Send + 'static,
//     Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex> + Send + Sync + 'static,
//     C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
//     C::Api: WeteeAssetRuntimeApi<Block, AccountId, Balance>,
// {
//     fn get_asset_balance(&self, dao_id: DaoAssetId, who: AccountId) -> RpcResult<Balance> {
//         let api = self.client.runtime_api();
//         let best = self.client.info().best_hash;

//         let amount = api.get_asset_balance(best, dao_id, who).unwrap();
//         Ok(amount)
//     }
// }
