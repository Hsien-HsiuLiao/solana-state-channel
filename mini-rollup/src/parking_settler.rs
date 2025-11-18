//! Parking space state channel settler component.
//!
//! When parking space transactions are finished in the state channel, this
//! component settles the final state to the Solana blockchain:
//! 1. Parking space status updates (sent as instructions to update the PDA)
//! 2. Payment transaction (single payment from driver to homeowner)

use {
    crate::transaction::StateChannelTransaction,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::Instruction as SolanaInstruction,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::Transaction as SolanaTransaction,
    },
    solana_svm::{
        transaction_processor::LoadAndExecuteSanitizedTransactionsOutput,
        transaction_results::TransactionExecutionResult,
    },
};

/// Parking space state channel settler.
pub struct StateChannelSettler<'a> {
    rpc_client: &'a RpcClient,
}

impl<'a> StateChannelSettler<'a> {
    pub fn new(rpc_client: &'a RpcClient) -> Self {
        Self { rpc_client }
    }

    /// Settle parking space state channel results to the Solana blockchain.
    ///
    /// This processes two types of transactions:
    /// 1. Parking space status updates - sent as instructions to update the PDA
    /// 2. Payment transaction - single payment from driver to homeowner
    ///
    /// # Arguments
    /// * `state_channel_transactions` - All transactions processed in the state channel
    /// * `svm_output` - Execution results from the SVM
    /// * `keys` - Keypairs for signing settlement transactions
    pub fn process_settle(
        &self,
        state_channel_transactions: &[StateChannelTransaction],
        svm_output: LoadAndExecuteSanitizedTransactionsOutput,
        keys: &[Keypair],
    ) {
        // Collect parking space status update instructions and find payment transaction
        let mut parking_status_instructions = Vec::new();
        let mut payment_instruction: Option<SolanaInstruction> = None;
        
        // Extract execution_results (this moves it out of svm_output)
        let execution_results = svm_output.execution_results;
        
        // Process transactions to collect parking status updates and payment
        state_channel_transactions
            .iter()
            .zip(execution_results.iter())
            .for_each(|(transaction, result)| {
                if result.was_executed_successfully() {
                    if transaction.parking_space_status.is_some() {
                        // Collect parking space status updates
                        parking_status_instructions.push(SolanaInstruction::from(transaction));
                    } else if let (Some(reserved_by_driver), Some(homeowner), Some(rental_amount_due)) = 
                        (transaction.reserved_by_driver, transaction.homeowner, transaction.rental_amount_due) {
                        // Single payment transaction: driver -> homeowner
                        payment_instruction = Some(system_instruction::transfer(
                            &reserved_by_driver,
                            &homeowner,
                            rental_amount_due,
                        ));
                    }
                }
            });
        
        // Build all instructions: parking status updates + payment
        let mut all_instructions = parking_status_instructions;
        if let Some(payment) = payment_instruction {
            all_instructions.push(payment);
        }
        
        // Send all instructions to the Solana blockchain in chunks
        let recent_blockhash = self.rpc_client.get_latest_blockhash().unwrap();
        all_instructions.chunks(10).for_each(|chunk| {
            let transaction = SolanaTransaction::new_signed_with_payer(
                chunk,
                Some(&keys[0].pubkey()),
                keys,
                recent_blockhash,
            );
            self.rpc_client
                .send_and_confirm_transaction(&transaction)
                .unwrap();
        });
    }
}

