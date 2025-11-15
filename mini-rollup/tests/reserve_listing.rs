#![allow(unexpected_cfgs)]


use solana_sdk::{
    pubkey::Pubkey,
    account::{Account, AccountSharedData},
};
use solana_client::rpc_client::RpcClient;


pub fn reserve_parking_space_listing(
    driver: &Pubkey,
    rental_duration: u64,
    program_id: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    
    //update parking space listing pda parking space status to reserved
    //update reservation duration
   Ok(())
}
