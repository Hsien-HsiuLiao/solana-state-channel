What do the tests do?
- create keypairs and program id
- create parking space lsting pda
- create testvalidator context with the accounts
    -details https://docs.rs/solana-test-validator/2.0.2/src/solana_test_validator/lib.rs.html#622-624

- what if mollusk was used instead of solana-test-validator

processor.rs
- to create an instance of TransactionBatchProcessor, ForkGraph trait must be implemented
- ForkGraph is a Solana Program Runtime trait that describes the relationship between slots (blocks) in a blockchain. In a validator, it helps determine: Whether two slots are on the same fork
If one slot is an ancestor of another
If slots are on different forks
- can add program(s) to cache for performance
- add_builtin() for registering system_program, bpf_loader