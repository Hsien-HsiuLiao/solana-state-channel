#![allow(unused)]

use {
    solana_sdk::{
        account::{Account, AccountSharedData, ReadableAccount},
        epoch_schedule::EpochSchedule,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::Keypair,
        system_program,
    },
    solana_test_validator::{TestValidator, TestValidatorGenesis},
};

const SLOTS_PER_EPOCH: u64 = 50;

pub struct TestValidatorContext {
    pub test_validator: TestValidator,
    pub payer: Keypair,
}

impl TestValidatorContext {
    pub fn start_with_accounts(accounts: Vec<(Pubkey, AccountSharedData)>) -> Self {
        #[rustfmt::skip]
        solana_logger::setup_with_default(
            "solana_rbpf::vm=debug,\
             solana_runtime::message_processor=debug,\
             solana_runtime::system_instruction_processor=trace",
        );

        let epoch_schedule = EpochSchedule::custom(SLOTS_PER_EPOCH, SLOTS_PER_EPOCH, false);

        let (test_validator, payer) = TestValidatorGenesis::default()
            .epoch_schedule(epoch_schedule)
            .add_accounts(accounts)
            .start(); //https://docs.rs/solana-test-validator/2.0.2/src/solana_test_validator/lib.rs.html#622-624

        Self {
            test_validator,
            payer,
        }
    }
}



pub fn system_account(lamports: u64) -> AccountSharedData {
    AccountSharedData::new(lamports, 0, &system_program::id())
}


