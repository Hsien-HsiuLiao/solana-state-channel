

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

pub enum ParkingSpaceStatus {
    Available,
    Reserved,
    Occupied,
    UnAvailable
}

pub struct StateChannelTransaction {
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
        } = value;

        system_instruction::transfer(&from.expect("From is required"), &to.expect("To is required"), amount.expect("Amount is required"))

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
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.from.expect("From is required")))
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
