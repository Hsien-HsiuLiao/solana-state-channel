# solana-state-channel

- compare costs, benchmarks between doing tx on-chain vs state channel

https://docs.rs/solana-svm/2.0.0/solana_svm/transaction_processor/struct.TransactionProcessingConfig.html

Struct solana_svm::transaction_processor::TransactionProcessingConfig

https://docs.rs/solana-svm/2.0.0/src/solana_svm/transaction_processor.rs.html#103-121

```
pub struct TransactionProcessingConfig<'a> {
    pub account_overrides: Option<&'a AccountOverrides>,
    pub check_program_modification_slot: bool,
    pub compute_budget: Option<ComputeBudget>,
    pub log_messages_bytes_limit: Option<usize>,
    pub limit_to_load_programs: bool,
    pub recording_config: ExecutionRecordingConfig,
    pub transaction_account_lock_limit: Option<usize>,
}
```

Fields

account_overrides: Option<&'a AccountOverrides>
Encapsulates overridden accounts, typically used for transaction simulation.

check_program_modification_slot: bool
Whether or not to check a program’s modification slot when replenishing a program cache instance.

compute_budget: Option<ComputeBudget>
The compute budget to use for transaction execution.

log_messages_bytes_limit: Option<usize>
The maximum number of bytes that log messages can consume.

limit_to_load_programs: bool
Whether to limit the number of programs loaded for the transaction batch.

recording_config: ExecutionRecordingConfig
Recording capabilities for transaction execution.

transaction_account_lock_limit: Option<usize>
The max number of accounts that a transaction may lock.