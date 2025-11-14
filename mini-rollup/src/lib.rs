mod state_channel_loader;
mod processor;

/*  mod loader;
mod settler; */
pub mod transaction;

use {
    crate::{
        state_channel_loader::StateChannelAccountLoader,/* loader::PayTubeAccountLoader, settler::PayTubeSettler, */ transaction::StateChannelTransaction,
    },
    processor::{create_transaction_batch_processor, get_transaction_check_results},

    solana_client::rpc_client::RpcClient,
    solana_compute_budget::compute_budget::ComputeBudget,
    solana_sdk::{
        feature_set::FeatureSet, fee::FeeStructure, hash::Hash, rent_collector::RentCollector,
        signature::Keypair,
    },
    solana_svm::transaction_processor::{
        TransactionProcessingConfig, TransactionProcessingEnvironment,
    },
   // transaction::create_svm_transactions,

    
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
       // PayTube loader/callback implementation.
      // let account_loader = PayTubeAccountLoader::new(&self.rpc_client);

         // Solana SVM transaction batch processor.
         let processor =
         create_transaction_batch_processor(&account_loader, &feature_set, &compute_budget);
/* 
     // The PayTube transaction processing runtime environment.
     let processing_environment = TransactionProcessingEnvironment {
         blockhash: Hash::default(),
         epoch_total_stake: None,
         epoch_vote_accounts: None,
         feature_set: Arc::new(feature_set),
         fee_structure: Some(&fee_structure),
         lamports_per_signature,
         rent_collector: Some(&rent_collector),
     };

     // The PayTube transaction processing config for Solana SVM.
     let processing_config = TransactionProcessingConfig {
         compute_budget: Some(compute_budget),
         ..Default::default()
     };

     // 1. Convert to an SVM transaction batch.
     let svm_transactions = create_svm_transactions(transactions);

     // 2. Process transactions with the SVM API.
     let results = processor.load_and_execute_sanitized_transactions(
         &account_loader,
         &svm_transactions,
         get_transaction_check_results(svm_transactions.len(), lamports_per_signature),
         &processing_environment,
         &processing_config,
     );

     // 3. Convert results into a final ledger using a `PayTubeSettler`.
     let settler = PayTubeSettler::new(&self.rpc_client);

     // 4. Submit to the Solana base chain.
     settler.process_settle(transactions, results, &self.keys); */

    }
}