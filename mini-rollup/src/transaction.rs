

use {
    solana_sdk::{
        instruction::Instruction as SolanaInstruction,
        pubkey::Pubkey,
        system_instruction,
        transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        },
    },
    std::collections::HashSet,
};


pub struct StateChannelTransaction {
    pub sensor_data: Option<u8>,
    pub parking_space_status: Option<ParkingSpaceStatus>,
    pub from: Option<Pubkey>,
    pub to: Option<Pubkey>,
    pub amount: Option<u64>,
}

impl From<&StateChannelTransaction> for SolanaInstruction {
    fn from(value: &StateChannelTransaction) -> Self {
        let StateChannelTransaction {
            sensor_data,
            parking_space_status,
            from,
            to,
            amount,
        } = value;
        if let Some(sensor_data) = sensor_data {
            //create instruction to update sensor data
            return sensor_data;
        }
        if let Some(parking_space_status) = parking_space_status {
            //create instruction to update parking space status
            /*
            #[account(
        mut,
        seeds = [marketplace.key().as_ref(), 
        maker.key().as_ref()
        ], 
        bump, 
      /
    )]
    pub listing: Account<'info, Listing>,

    pub struct Listing {
    pub maker: Pubkey,
    #[max_len(32)] 
    pub email: String,
    #[max_len(8)] 
    pub phone: String,
    pub bump: u8, 
    #[max_len(32)] 
    pub address: String,
    pub latitude:f64, 
    pub longitude:f64,
    pub rental_rate: u32, //per hour
    pub availabilty_start: i64, //unix time stamp
    pub availabilty_end: i64,
    #[max_len(8)] 
    pub sensor_id: String, 
    pub reserved_by: Option<Pubkey>, 
    pub reservation_start: Option<i64>,
    pub reservation_end: Option<i64>,
    pub parking_space_status:ParkingSpaceStatus, 
    
    #[max_len(32)]
    pub additional_info: Option<String>,
    //date/times avail
    pub feed: Option<Pubkey>
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum ParkingSpaceStatus {
    Available,
    Reserved,
    Occupied,
    UnAvailable
}
            */
            
            return parking_space_status;
            system_instruction::
        }
        if let Some(from), Some(to), Some(amount) = (from, to, amount) {
             //https://docs.rs/solana-program/2.0.0/src/solana_program/system_instruction.rs.html#885
        /*
        Instruction::new_with_bincode(
         program_id: Pubkey,
    data: &T,
    accounts: Vec<AccountMeta>
    )
        */
            system_instruction::transfer(from, to, *amount)

        }
    }
}

impl From<&PayTubeTransaction> for SolanaTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.from))
    }
}

impl From<&PayTubeTransaction> for SolanaSanitizedTransaction {
    fn from(value: &PayTubeTransaction) -> Self {
        SolanaSanitizedTransaction::try_from_legacy_transaction(
            SolanaTransaction::from(value),
            &HashSet::new(),
        )
        .unwrap()
    }
}

/// Create a batch of Solana transactions, for the Solana SVM's transaction
/// processor, from a batch of PayTube instructions.
pub fn create_svm_transactions(
    state_channel_transactions: &[StateChannelTransaction],
) -> Vec<SolanaSanitizedTransaction> {
   
}
