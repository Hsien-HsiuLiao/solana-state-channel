
use {
    solana_sdk::{
        instruction::{AccountMeta, Instruction as SolanaInstruction},
        pubkey::Pubkey,
        system_instruction,
        transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        },
    },
    std::collections::HashSet,
    borsh::{BorshSerialize, BorshDeserialize},
};

#[derive(Debug, Clone, Copy,  PartialEq, BorshSerialize, BorshDeserialize)]
pub enum ParkingSpaceStatus {
    Available,
    Reserved,
    Occupied,
    UnAvailable,
    SensorTriggered,
}
#[derive(Debug)]
pub struct StateChannelTransaction {
    pub program_id: Option<Pubkey>,
    pub parking_space_pda: Option<Pubkey>,
    pub parking_space_status: Option<ParkingSpaceStatus>,
    pub homeowner: Option<Pubkey>,
    pub rental_rate_per_hour: Option<u64>,
    pub reservation_duration: Option<u64>,
    pub reserved_by_driver: Option<Pubkey>,
    pub rental_amount_due: Option<u64>,
}

impl From<&StateChannelTransaction> for SolanaInstruction {
    fn from(value: &StateChannelTransaction) -> Self {
        let StateChannelTransaction {
            parking_space_status,
            homeowner,
            rental_amount_due,
            reservation_duration,
            reserved_by_driver,
            program_id,
            parking_space_pda,
            rental_rate_per_hour: _,
        } = value;

        // Handle parking space status updates
        if let Some(parking_space_status) = parking_space_status {
            println!("Updating parking space status: {:?}", parking_space_status);
            let program_id = program_id.expect("Program ID is required for parking status update");
            let listing_pda = parking_space_pda.expect("Parking space PDA is required for parking status update");
            
            // Determine payer based on status type
            let payer = match parking_space_status {
                ParkingSpaceStatus::SensorTriggered => {
                    println!("Sensor triggered");
                    // Sensor triggered: homeowner or reserved_by_driver can update
                    homeowner.as_ref()
                        .or(reserved_by_driver.as_ref())
                        .expect("Either 'homeowner' or 'reserved_by_driver' is required for SensorTriggered status")
                }
                ParkingSpaceStatus::Occupied => {
                    println!("Occupied");
                    // Driver occupies the space: reserved_by_driver is payer
                    reserved_by_driver.as_ref()
                        .expect("reserved_by_driver is required for Occupied status")
                }
                ParkingSpaceStatus::Available => {
                    println!("Available");
                    // Space becomes available: homeowner or reserved_by_driver can update
                    homeowner.as_ref()
                        .or(reserved_by_driver.as_ref())
                        .expect("Either 'homeowner' or 'reserved_by_driver' is required for Available status")
                }
                ParkingSpaceStatus::Reserved => {
                    println!("Reserved");
                    // Driver reserves: reserved_by_driver is the payer
                    reserved_by_driver.as_ref()
                        .expect("reserved_by_driver is required for Reserved status")
                }
                ParkingSpaceStatus::UnAvailable => {
                    println!("UnAvailable");
                    // Space unavailable: homeowner typically sets this
                    homeowner.as_ref()
                        .or(reserved_by_driver.as_ref())
                        .expect("Either 'homeowner' or 'reserved_by_driver' is required for UnAvailable status")
                }
            };
            
            // Create instruction data
            // Format: [instruction_discriminator: u8, status: u8, payer: 32 bytes, reservation_duration: 8 bytes (optional)]
            let mut data = vec![1u8]; // instruction discriminator for "update_parking_status"
            data.push(*parking_space_status as u8); // status enum as u8
            data.extend_from_slice(payer.as_ref()); // payer pubkey (32 bytes) - varies by status type
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
        } else if let (Some(reserved_by_driver), Some(homeowner), Some(rental_amount_due)) = (reserved_by_driver, homeowner, rental_amount_due) {
        //https://docs.rs/solana-program/2.0.0/src/solana_program/system_instruction.rs.html#885

            println!("Transferring rental amount: {:?} from driver {:?} to homeowner {:?}", rental_amount_due, reserved_by_driver, homeowner);
            // Handle payment transfer transactions
            system_instruction::transfer(reserved_by_driver, homeowner, *rental_amount_due)
        } else {
            panic!("StateChannelTransaction must be either a parking status update or a payment transfer");
        }
      
    }
}

impl From<&StateChannelTransaction> for SolanaTransaction {
    fn from(value: &StateChannelTransaction) -> Self {
        // For parking space status updates, use reserved_by_driver as payer
        // For transfers, use reserved_by_driver as payer
        let default_payer = Pubkey::default();
        let payer = if value.parking_space_status.is_some() {
            // Parking status update: reserved_by_driver is the payer
            value.reserved_by_driver
                .as_ref()
                .unwrap_or(&default_payer)
        } else {
            // Transfer: use reserved_by_driver (the driver who reserved)
            value.reserved_by_driver.as_ref().unwrap_or(&default_payer)
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
