use solana_sdk::{
    pubkey::Pubkey,
    account::{Account, AccountSharedData},
};

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


