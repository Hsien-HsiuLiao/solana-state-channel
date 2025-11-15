use solana_sdk::{
    pubkey::Pubkey,
    account::{Account, AccountSharedData},
};
use solana_client::rpc_client::RpcClient;

/// Create a parking space listing PDA account with rental rate stored in account data
/// 
/// Returns a tuple of (PDA address, AccountSharedData) that can be added to test validator accounts.
/// The rental rate is stored in the account data.
/// 
/// # Arguments
/// * `homeowner` - The public key of the homeowner
/// * `rental_rate_usdc` - The rental rate in USDC (as u64, representing the smallest unit)
/// * `program_id` - The program ID that owns the PDA
/// * `lamports` - Initial lamports for the account (for rent exemption)
pub fn create_parking_space_listing(
    homeowner: &Pubkey,
    rental_rate_usdc: u64,
    program_id: &Pubkey,
    lamports: u64,
) -> (Pubkey, AccountSharedData) {
    // Derive the PDA
    let (pda, _bump) = Pubkey::find_program_address(
        &[
            b"parking_space",
            homeowner.as_ref(),
        ],
        program_id,
    );
    
    // Create account data with rental rate stored
    // Data layout: [rental_rate_usdc: u64 (8 bytes)]
    let mut data = vec![0u8; 8];
    data[0..8].copy_from_slice(&rental_rate_usdc.to_le_bytes());
    
    // Create the account with the rental rate stored in data
    let account = Account {
        lamports,
        data,
        owner: *program_id,
        executable: false,
        rent_epoch: 0,
    };
    
    (pda, AccountSharedData::from(account))
}

/// Read the rental rate from a parking space PDA account
/// 
/// Fetches the account data from the RPC client and deserializes the rental rate
/// that was stored in the first 8 bytes of the account data.
/// 
/// # Arguments
/// * `rpc_client` - The RPC client to query the account
/// * `pda` - The PDA address of the parking space listing
/// 
/// # Returns
/// * `Ok(u64)` - The rental rate in USDC (smallest unit)
/// * `Err` - If the account doesn't exist or data can't be read
pub fn get_rental_rate_from_pda(
    rpc_client: &RpcClient,
    pda: &Pubkey,
) -> Result<u64, Box<dyn std::error::Error>> {
    let account = rpc_client.get_account(pda)?;
    
    // The rental rate is stored in the first 8 bytes as a little-endian u64
    if account.data.len() < 8 {
        return Err("Account data too short".into());
    }
    
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&account.data[0..8]);
    let rental_rate = u64::from_le_bytes(bytes);
    
    Ok(rental_rate)
}


