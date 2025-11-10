/* mod loader;
mod processor;
mod settler; */
pub mod transaction;

use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::signature::Keypair,
  /*   solana_svm::transaction_processor::{
        TransactionProcessingConfig, TransactionProcessingEnvironment,
    }, */
    
};

pub struct StateChannel {
    /// I think you know why this is a bad idea...
    keys: Vec<Keypair>,
    rpc_client: RpcClient,
}

impl StateChannel {
    pub fn new(keys: Vec<Keypair>, rpc_client: RpcClient) -> Self {
        Self { keys, rpc_client }
    }
}