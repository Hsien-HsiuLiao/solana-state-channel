use mini_rollup::transaction::{StateChannelTransaction, ParkingSpaceStatus};
use solana_sdk::pubkey::Pubkey;

pub struct TransactionBuilder {
    transactions: Vec<StateChannelTransaction>,
    program_id: Option<Pubkey>,
    parking_space_pda: Option<Pubkey>,
    driver_pubkey: Option<Pubkey>,
    homeowner_pubkey: Option<Pubkey>,
    reservation_duration: Option<u64>,
    rental_rate_per_hour: Option<u64>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self { 
            transactions: Vec::new(),
            program_id: None,
            parking_space_pda: None,
            driver_pubkey: None,
            homeowner_pubkey: None,
            reservation_duration: None,
            rental_rate_per_hour: None,
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

    pub fn set_reservation_duration(&mut self, duration: u64) {
        self.reservation_duration = Some(duration);
    }

    pub fn set_rental_rate_per_hour(&mut self, rate: u64) {
        self.rental_rate_per_hour = Some(rate);
    }

    pub fn add_parking_space_status_update(&mut self, status: ParkingSpaceStatus) {
        self.transactions.push(StateChannelTransaction {
            program_id: self.program_id,
            parking_space_pda: self.parking_space_pda,
            parking_space_status: Some(status),
            reserved_by_driver: self.driver_pubkey,
            homeowner: self.homeowner_pubkey,
            rental_amount_due: None,
            reservation_duration: self.reservation_duration,
            rental_rate_per_hour: self.rental_rate_per_hour,
        });
        println!("Parking space status update transaction added:\n {:?}\n", self.transactions.last().unwrap());
    }

    pub fn add_payment_transaction(&mut self) {
        self.transactions.push(StateChannelTransaction {
            program_id: self.program_id,
            parking_space_pda: self.parking_space_pda,
            reserved_by_driver: self.driver_pubkey,
            homeowner: self.homeowner_pubkey,
            rental_amount_due: Some(self.rental_rate_per_hour.unwrap() * self.reservation_duration.unwrap() / 3600),
            parking_space_status: None,
            reservation_duration: self.reservation_duration,
            rental_rate_per_hour: self.rental_rate_per_hour,
        });
        println!("Payment transaction added:\n {:?}\n", self.transactions.last().unwrap());
    }

    pub fn build(self) -> Vec<StateChannelTransaction> {
        self.transactions
    }
}