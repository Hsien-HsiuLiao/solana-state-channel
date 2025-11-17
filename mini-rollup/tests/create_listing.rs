use solana_sdk::{
    pubkey::Pubkey,
    account::{Account, AccountSharedData},
    rent::Rent,
};
use solana_client::rpc_client::RpcClient;
use borsh::{BorshSerialize, BorshDeserialize};
use mini_rollup::transaction::ParkingSpaceStatus;



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


