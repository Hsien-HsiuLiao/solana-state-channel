

use {
    solana_sdk::{
        instruction::{AccountMeta, Instruction as SolanaInstruction},
        pubkey::Pubkey,
        system_instruction,
        transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        },
     //   borsh::{BorshSerialize, BorshDeserialize},

    },
    std::collections::HashSet,
};

#[derive(Debug, Clone, Copy,  PartialEq)]
pub enum ParkingSpaceStatus {
    Available,
    Reserved,
    Occupied,
    UnAvailable,
    SensorTriggered,
}

pub struct StateChannelTransaction {
    pub program_id: Option<Pubkey>,
    pub parking_space_pda: Option<Pubkey>,
    pub sensor_data: Option<u8>,
    pub parking_space_status: Option<ParkingSpaceStatus>,
    pub from: Option<Pubkey>,
    pub to: Option<Pubkey>,
    pub amount: Option<u64>,
    pub reservation_duration: Option<u64>,
    pub reserved_by: Option<Pubkey>,
}
/* 
pub struct TransactionBuilder {
    transactions: Vec<StateChannelTransaction>,
  //  rpc_client: RpcClient,
}

impl TransactionBuilder {
    pub fn new(/* rpc_client: RpcClient */) -> Self {
        Self {
            transactions: Vec::new(),
         //   rpc_client,
        }
    }
    
    pub fn add_svm_transaction(&mut self, transaction: StateChannelTransaction) {
        self.transactions.push(transaction);
    }
    
    pub fn build(self) -> Vec<StateChannelTransaction> {
        self.transactions
    }
    
 /*    pub fn process(self, state_channel: &StateChannel) {
        state_channel.process_state_channel_transfers(&self.transactions);
    } */ 
    
} */
 
/* // Usage:
let mut builder = PayTubeTransactionBuilder::new(rpc_client);
builder.add_svm_transaction(tx1);
builder.add_svm_transaction(tx2);
builder.process(&channel); */


impl From<&StateChannelTransaction> for SolanaInstruction {
    fn from(value: &StateChannelTransaction) -> Self {
        let StateChannelTransaction {
            sensor_data,
            parking_space_status,
            from,
            to,
            amount,
            reservation_duration,
            reserved_by,
            program_id,
            parking_space_pda,
        } = value;

        // Handle parking space status updates
        if let Some(parking_space_status) = parking_space_status {
            // TODO: Replace with actual program ID and derive listing PDA
            let program_id = program_id.expect("Program ID is required");
            let listing_pda = parking_space_pda.expect("Parking space PDA is required");
            
            let payer = reserved_by.as_ref().unwrap();
               
            
            // Create instruction data
            // Format: [instruction_discriminator: u8, status: u8, reserved_by: 32 bytes, reservation_duration: 8 bytes (optional)]
            let mut data = vec![1u8]; // instruction discriminator for "update_parking_status"
            data.push(*parking_space_status as u8); // status enum as u8
            data.extend_from_slice(payer.as_ref()); // reserved_by pubkey (32 bytes)
            if let Some(duration) = reservation_duration {
                data.extend_from_slice(&duration.to_le_bytes()); // reservation_duration (8 bytes)
            } else {
                data.extend_from_slice(&0u64.to_le_bytes()); // default duration
            }
            
            // Create the instruction with required accounts
            SolanaInstruction::new_with_bincode(
                program_id,
                &data,
                vec![
                    AccountMeta::new(listing_pda, false), // listing PDA (mutable)
                    AccountMeta::new(*payer, true), // payer (signer, writable)
                ],
            )
        } else if let (Some(from), Some(to), Some(amount)) = (from, to, amount) {
            // Handle transfer transactions
            system_instruction::transfer(from, to, *amount)
        } else {
            // If it's not a transfer or parking update, create a no-op instruction
            // This should be replaced with actual sensor data instructions
            system_instruction::transfer(
                &Pubkey::default(),
                &Pubkey::default(),
                0,
            )
        }

      /*   if let Some(sensor_data) = sensor_data {
            //create instruction to update sensor data
//            return sensor_data;
                Instruction::new_with_bincode(
                    program_id: Pubkey,
                    data: &T,
                    accounts: Vec<AccountMeta>
                    )
        } */
      /*   if let Some(parking_space_status) = parking_space_status {
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
            
                pub bump: u8, 
    
                pub rental_rate: u32, //per hour
            
                pub reserved_by: Option<Pubkey>, 
                pub reservation_start: Option<i64>,
                pub reservation_end: Option<i64>,
                pub parking_space_status:ParkingSpaceStatus, 
            }


            */
            
            return parking_space_status;
          //  system_instruction:: create manual instruction for parking space status update
        } */
      /*   if let (Some(from), Some(to), Some(amount)) = (from, to, amount) {
             //https://docs.rs/solana-program/2.0.0/src/solana_program/system_instruction.rs.html#885
        /*
            Instruction::new_with_bincode(
            program_id: Pubkey,
            data: &T,
            accounts: Vec<AccountMeta>
            )
        */
            system_instruction::transfer(from, to, *amount)

        } */
    }
}

impl From<&StateChannelTransaction> for SolanaTransaction {
    fn from(value: &StateChannelTransaction) -> Self {
        // For parking space status updates, use reserved_by as payer (or from as fallback)
        // For transfers, use from as payer
        let default_payer = Pubkey::default();
        let payer = if value.parking_space_status.is_some() {
            // Parking status update: reserved_by takes precedence, then from
            value.reserved_by
                .as_ref()
                .or(value.from.as_ref())
                .unwrap_or(&default_payer)
        } else {
            // Transfer or other: use from
            value.from.as_ref().unwrap_or(&default_payer)
        };
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(payer))
    }
}

impl From<&StateChannelTransaction> for SolanaSanitizedTransaction {
    fn from(value: &StateChannelTransaction) -> Self {
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
    state_channel_transactions
        .iter()
        .map(SolanaSanitizedTransaction::from)
        .collect()
}
