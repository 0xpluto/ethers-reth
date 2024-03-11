// std
use eyre::Result;
use noop::NoopNetwork;
use reth_db::DatabaseEnv;
use reth_interfaces::RethError;
use reth_node_ethereum::EthEvmConfig;
use std::{fmt::Debug, path::Path, sync::Arc};

// ethers
use ethers::providers::{Middleware, MiddlewareError};

//Reth
use reth_blockchain_tree::ShareableBlockchainTree;
use reth_provider::{providers::BlockchainProvider, ProviderError};
use reth_revm::EvmProcessorFactory;
use reth_rpc::{eth::error::EthApiError, DebugApi, EthApi, EthFilter, TraceApi};
use reth_transaction_pool::{
    blobstore::InMemoryBlobStore, CoinbaseTipOrdering, EthPooledTransaction,
    EthTransactionValidator, Pool, TransactionValidationTaskExecutor,
};
//Error
use jsonrpsee::types::ErrorObjectOwned;
use thiserror::Error;

pub mod init;
pub mod middleware;
pub mod noop;
pub mod type_conversions;
use tokio::runtime::Handle;

pub type RethClient = BlockchainProvider<
    Arc<DatabaseEnv>,
    ShareableBlockchainTree<Arc<DatabaseEnv>, EvmProcessorFactory<EthEvmConfig>>,
>;

pub type RethTxPool = Pool<
    TransactionValidationTaskExecutor<EthTransactionValidator<RethClient, EthPooledTransaction>>,
    CoinbaseTipOrdering<EthPooledTransaction>,
    InMemoryBlobStore,
>;

pub type RethApi = EthApi<RethClient, RethTxPool, NoopNetwork, EthEvmConfig>;
pub type RethFilter = EthFilter<RethClient, RethTxPool>;
pub type RethTrace = TraceApi<RethClient, RethApi>;
pub type RethDebug = DebugApi<RethClient, RethApi>;

#[derive(Clone)]
pub struct RethMiddleware<M> {
    inner: M,
    reth_api: RethApi,
    reth_filter: RethFilter,
    reth_trace: RethTrace,
    reth_debug: RethDebug,
}

impl<M: std::fmt::Debug> std::fmt::Debug for RethMiddleware<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RethMiddleware").field("inner", &self.inner).finish_non_exhaustive()
    }
}

#[derive(Error, Debug)]
pub enum RethMiddlewareError<M: Middleware> {
    /// An error occured in one of the middlewares.
    #[error("{0}")]
    MiddlewareError(M::Error),

    /// An error occurred in the Reth API.
    #[error(transparent)]
    RethApiError(#[from] ErrorObjectOwned),

    /// An error occurred in the Eth API.
    #[error(transparent)]
    EthApiError(#[from] EthApiError),

    #[error(transparent)]
    RethError(#[from] reth_interfaces::RethError),

    /// A trace was expected but none was found.
    #[error("Missing trace")]
    MissingTrace,

    #[error("Chain Id unavailable")]
    ChainIdUnavailable,
}

impl<M: Middleware> From<ProviderError> for RethMiddlewareError<M> {
    fn from(e: ProviderError) -> Self {
        RethMiddlewareError::RethError(RethError::Provider(e))
    }
}

impl<M: Middleware> MiddlewareError for RethMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(e: Self::Inner) -> Self {
        RethMiddlewareError::MiddlewareError(e)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            RethMiddlewareError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

impl<M> RethMiddleware<M>
where
    M: Middleware,
{
    pub fn new<P: AsRef<Path>>(
        inner: M,
        db_path: P,
        handle: Handle,
        chain_id: u64,
    ) -> Result<Self> {
        let (reth_api, reth_filter, reth_trace, reth_debug) =
            Self::try_new(db_path.as_ref(), handle, chain_id)?;
        Ok(Self { inner, reth_api, reth_filter, reth_trace, reth_debug })
    }

    pub fn reth_api(&self) -> &RethApi {
        &self.reth_api
    }
}
