 mod loader;
/*mod processor;
mod settler; */
pub mod transaction;

use {
    crate::{
        loader::StateChannelAccountLoader, /* settler::PayTubeSettler,*/ transaction::StateChannelTransaction,
    },
    solana_client::rpc_client::RpcClient,
    solana_compute_budget::compute_budget::ComputeBudget,
    solana_sdk::{
        feature_set::FeatureSet, fee::FeeStructure, hash::Hash, rent_collector::RentCollector,
        signature::Keypair,
    },
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

    pub fn process_transactions(&self, transactions: &[StateChannelTransaction]) {
        // PayTube default configs.
        let compute_budget = ComputeBudget::default();
        let feature_set = FeatureSet::all_enabled();
        let fee_structure = FeeStructure::default();
        let lamports_per_signature = fee_structure.lamports_per_signature;
        let rent_collector = RentCollector::default();

        // StateChannel loader/callback implementation.
        let account_loader = StateChannelAccountLoader::new(&self.rpc_client);
    }
}