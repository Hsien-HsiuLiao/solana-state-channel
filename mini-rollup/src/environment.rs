use {
    solana_sdk::{
        feature_set::FeatureSet, fee::FeeStructure, hash::Hash, rent_collector::RentCollector,
    },
    solana_svm::transaction_processor::TransactionProcessingEnvironment,
    std::sync::Arc,
};

pub fn get_environment<'a>(
    fee_structure: &'a FeeStructure,
rent_collector: &'a RentCollector,
) -> TransactionProcessingEnvironment<'a> {
    let feature_set = FeatureSet::all_enabled();
   // let fee_structure = FeeStructure::default();
    let lamports_per_signature = fee_structure.lamports_per_signature;
   // let rent_collector = RentCollector::default();

    let tp_environment =     TransactionProcessingEnvironment {
        blockhash: Hash::default(),
        epoch_total_stake: None,
        epoch_vote_accounts: None,
        feature_set: Arc::new(feature_set),
        fee_structure: Some(&fee_structure),
        lamports_per_signature,
        rent_collector: Some(&rent_collector),
    };

    tp_environment
}

