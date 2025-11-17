use mini_rollup::transaction::{StateChannelTransaction, ParkingSpaceStatus};
use solana_sdk::pubkey::Pubkey;

pub struct TransactionBuilder {
    transactions: Vec<StateChannelTransaction>,
    program_id: Option<Pubkey>,
    parking_space_pda: Option<Pubkey>,
    driver_pubkey: Option<Pubkey>,
    homeowner_pubkey: Option<Pubkey>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self { 
            transactions: Vec::new(),
            program_id: None,
            parking_space_pda: None,
            driver_pubkey: None,
            homeowner_pubkey: None,
        }
    }

    pub fn set_program_id(&mut self, program_id: Pubkey) {
        self.program_id = Some(program_id);
    }

    pub fn set_parking_space_pda(&mut self, pda: Pubkey) {
        self.parking_space_pda = Some(pda);
    }

    pub fn set_driver_pubkey(&mut self, driver: Pubkey) {
        self.driver_pubkey = Some(driver);
    }

    pub fn set_homeowner_pubkey(&mut self, homeowner: Pubkey) {
        self.homeowner_pubkey = Some(homeowner);
    }

    pub fn add_parking_space_status_update(&mut self, status: ParkingSpaceStatus) {
        self.transactions.push(StateChannelTransaction {
            program_id: self.program_id,
            parking_space_pda: self.parking_space_pda,
            parking_space_status: Some(status),
            reserved_by: self.driver_pubkey,
            from: None,
            to: None,
            amount: None,
            reservation_duration: None,
        });
    }

    pub fn add_payment_transaction(&mut self, amount: u64) {
        self.transactions.push(StateChannelTransaction {
            program_id: self.program_id,
            parking_space_pda: self.parking_space_pda,
            from: self.driver_pubkey,
            to: self.homeowner_pubkey,
            amount: Some(amount),
            parking_space_status: None,
            reservation_duration: None,
            reserved_by: None,
        });
    }

    pub fn build(self) -> Vec<StateChannelTransaction> {
        self.transactions
    }
}