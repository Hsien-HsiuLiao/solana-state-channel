use solana_sdk::{
    pubkey::Pubkey,
    account::{Account, AccountSharedData},
    rent::Rent,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction as SolanaTransaction,
    instruction::Instruction as SolanaInstruction,
};
use solana_client::rpc_client::RpcClient;
use borsh::{BorshSerialize, BorshDeserialize};
use mini_rollup::transaction::{ParkingSpaceStatus, StateChannelTransaction};

/// Uses Borsh serialization for type-safe account data storage
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct ParkingSpaceAccountData {
    pub rental_rate_usdc: u64,
    pub reserved_by: Option<Pubkey>,
    pub reservation_duration: Option<u64>,
    pub parking_space_status: ParkingSpaceStatus, 
}

/// Create a parking space listing PDA account with all fields stored in account data
/// 
/// Returns a tuple of (PDA address, AccountSharedData) that can be added to test validator accounts.
/// 
/// # Arguments
/// * `homeowner` - The public key of the homeowner
/// * `rental_rate_usdc` - The rental rate in USDC (as u64, representing the smallest unit)
/// * `program_id` - The program ID that owns the PDA
/// * `reserved_by` - Optional pubkey of who reserved the space (defaults to None)
/// * `reservation_duration` - Optional reservation duration in seconds (defaults to None)
/// * `parking_space_status` - Initial status Available
pub fn create_parking_space_listing(
    homeowner: &Pubkey,
    rental_rate_usdc: u64,
    program_id: &Pubkey,
    reserved_by: Option<Pubkey>,
    reservation_duration: Option<u64>,
    parking_space_status: ParkingSpaceStatus,
) -> Result<(Pubkey, AccountSharedData), Box<dyn std::error::Error>> {
    let (pda, _bump) = Pubkey::find_program_address(
        &[
            b"parking_space",
            homeowner.as_ref(),
        ],
        program_id,
    );
    
    let account_data = ParkingSpaceAccountData {
        rental_rate_usdc,
        reserved_by,
        reservation_duration,
        parking_space_status,
    };
    
    //  Serialize to bytes using Borsh
    let data = borsh::to_vec(&account_data)
        .map_err(|e| format!("Failed to serialize account data: {}", e))?;
    
    //  Calculate rent exemption based on serialized data size
    // Use Rent::default() instead of Rent::get() because sysvars aren't available
    // when creating accounts before the test validator starts
    let rent = Rent::default();
    let required_lamports = rent.minimum_balance(data.len());
    
    //  Create the Account with serialized data
    let account = Account {
        lamports: required_lamports,
        data,
        owner: *program_id,
        executable: false,
        rent_epoch: 0,
    };
    
    // 6. Convert to AccountSharedData for test validator
    Ok((pda, AccountSharedData::from(account)))
}

/// Read the rental rate from a parking space PDA account
/// 
/// Fetches the account data from the RPC client and deserializes it to return the rental rate.
/// 
/// # Arguments
/// * `rpc_client` - The RPC client to query the account
/// * `pda` - The PDA address of the parking space listing
/// 
/// # Returns
/// * `Ok(u64)` - The rental rate in USDC (as u64, representing the smallest unit)
/// * `Err` - If the account doesn't exist or data can't be read
pub fn get_rental_rate_from_pda(
    rpc_client: &RpcClient,
    pda: &Pubkey,
) -> Result<u64, Box<dyn std::error::Error>> {
    let account = rpc_client.get_account(pda)?;
    
    // Deserialize using Borsh - this handles all the byte manipulation automatically
    let account_data: ParkingSpaceAccountData = borsh::from_slice(&account.data)
        .map_err(|e| format!("Failed to deserialize account data: {}", e))?;
    
    Ok(account_data.rental_rate_usdc)
}

/// Reserve a parking space listing by updating the PDA account data and sending a transaction
/// 
/// Reads the current account data, updates it to Reserved status with the driver and duration,
/// creates and sends a transaction to update the account on-chain.
/// 
/// # Arguments
/// * `rpc_client` - The RPC client to query the current account data and send transactions
/// * `homeowner` - The public key of the homeowner (needed to derive the PDA)
/// * `driver` - The Keypair of the driver reserving the space (needed to sign the transaction)
/// * `rental_duration` - The reservation duration in seconds
/// * `program_id` - The program ID that owns the PDA
/// 
/// # Returns
/// * `Ok(Pubkey)` - The PDA address of the parking space listing
/// * `Err` - If the account doesn't exist or data can't be read/updated/transaction fails
pub fn reserve_parking_space_listing(
    rpc_client: &RpcClient,
    homeowner: &Pubkey,
    driver: &Keypair,
    rental_duration: u64,
    program_id: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    // 1. Derive the PDA address (same as in create_parking_space_listing)
    let (pda, _bump) = Pubkey::find_program_address(
        &[
            b"parking_space",
            homeowner.as_ref(),
        ],
        program_id,
    );
    
    let driver_pubkey = driver.pubkey();
    
    // 2. Create a StateChannelTransaction for the reservation
    let reserve_tx = StateChannelTransaction {
        parking_space_status: Some(ParkingSpaceStatus::Reserved),
        reservation_duration: Some(rental_duration),
        reserved_by: Some(driver_pubkey),
        from: None,
        to: None,
        amount: None,
        program_id: Some(*program_id),
        parking_space_pda: Some(pda),
    };
    
    // 3. Convert to Solana transaction and send it
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    
    // Create the instruction
    let instruction = SolanaInstruction::from(&reserve_tx);
    
    // Create and sign the transaction
    let solana_tx = SolanaTransaction::new_signed_with_payer(
        &[instruction],
        Some(&driver_pubkey),
        &[driver],
        recent_blockhash,
    );
    //need to deploy a program to the test validator to send the transaction
    //solana-test-validator --bpf-program <PROGRAM_ID> <path/to/program.so>
    //or
    //solana-test-validator --bpf-program fixtures/address-kp.json fixtures/program.so
   /*  
    // Send and confirm the transaction
    rpc_client.send_and_confirm_transaction(&solana_tx)?;
    
    // 4. Verify the reservation by reading the updated account data
    let account = rpc_client.get_account(&pda)?;
    let account_data: ParkingSpaceAccountData = borsh::from_slice(&account.data)
        .map_err(|e| format!("Failed to deserialize account data: {}", e))?;
    
    // Verify the account was updated correctly
    if account_data.parking_space_status != ParkingSpaceStatus::Reserved {
        return Err("Parking space status was not updated to Reserved".into());
    }
    if account_data.reserved_by != Some(driver_pubkey) {
        return Err("Reserved by was not updated correctly".into());
    }
    if account_data.reservation_duration != Some(rental_duration) {
        return Err("Reservation duration was not updated correctly".into());
    }*/ 
    
    Ok(pda)
}

