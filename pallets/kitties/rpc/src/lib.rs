use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use std::sync::Arc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT},
};
use pallet_kitties_rpc_runtime_api::KittiesRuntimeApi;

#[rpc]
pub trait KittiesRpc<BlockHash> {
	#[rpc(name = "get_kitties_count")]
	fn get_kitties_count(&self, at: Option<BlockHash>) -> Result<u64>;
}

pub struct PalletKittiesRpc<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> PalletKittiesRpc<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

pub enum Error {
	DecodeError,
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> KittiesRpc<<Block as BlockT>::Hash>
	for PalletKittiesRpc<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: KittiesRuntimeApi<Block>,
{
	fn get_kitties_count(
		&self,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let result = api.get_kitties_count(&at);
		result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to get kitties count.".into(),
			data: Some(e.to_string().into()),
		})
	}
}