mod setup;
mod listing_helpers;

use {
    mini_rollup::{transaction::{StateChannelTransaction, ParkingSpaceStatus}, StateChannel},
    setup::{system_account, TestValidatorContext},
    listing_helpers::{create_parking_space_listing, get_rental_rate_from_pda, reserve_parking_space_listing},
    solana_sdk::{
        pubkey::Pubkey, 
        signature::Keypair, 
        signer::Signer,
    },
    solana_client::rpc_client::RpcClient,
};

#[test]
fn test_parking_tx() {
    let homeowner = Keypair::new();
    let driver = Keypair::new();
    
    let homeowner_pubkey = homeowner.pubkey();
    let driver_pubkey = driver.pubkey();

    let program_id = Pubkey::new_unique();

    //homeowner creates a parking space listing PDA with  rental rate
    let rental_rate_usdc = 10_000_000; // 10 USDC
    let (parking_space_pda, parking_space_account) = create_parking_space_listing(
        &homeowner_pubkey,
        rental_rate_usdc,
        &program_id,
        None, // reserved_by
        None, // reservation_duration
        ParkingSpaceStatus::Available, // parking_space_status
    ).expect("Failed to create parking space listing");

    let accounts = vec![
        (homeowner_pubkey, system_account(10_000_000)),
        (driver_pubkey, system_account(10_000_000)),
        (parking_space_pda, parking_space_account.clone()), 
    ];

   
    let context = TestValidatorContext::start_with_accounts(accounts);
    let test_validator = &context.test_validator;
    let payer = context.payer.insecure_clone();

    let rpc_client = test_validator.get_rpc_client();

    let pda_rental_rate = get_rental_rate_from_pda(&rpc_client, &parking_space_pda)
    .expect("PDA account should exist");
    
    assert_eq!(pda_rental_rate, rental_rate_usdc, 
    "Expected rental rate {} but got {}", rental_rate_usdc, pda_rental_rate);

    //driver reserves a parking space
    //listingPda parking space status changes to reserved, reservation length is updated, reserved_by is updated
 /*    StateChannelTransaction {
        parking_space_status: Some(ParkingSpaceStatus::Reserved),
        reservation_duration: Some(1_000_000),
        reserved_by: Some(driver_pubkey), //payer
        from: None,
        to: None,
        amount: None,
        sensor_data: None,
    } */

    // Driver reserves the parking space by sending a transaction
    let reservation_duration = 1_000_000;
    let parking_space_pda = reserve_parking_space_listing(
        &rpc_client, 
        &homeowner_pubkey, 
        &driver, 
        reservation_duration, 
        &program_id
    ).expect("Failed to reserve parking space");
    
    //after driver reservation confirmed, channel opens
    let state_channel = StateChannel::new(vec![payer, homeowner, driver], rpc_client);


    /*
    let tx_list_fixed_size_maybe4 = [StateChannelTransaction; 4];
    let mut builder = TransactionBuilder::new();
    builder.add_parking_space_status_update(ParkingSpaceStatus::Reserved, tx_list_fixed_size_maybe4[0]);
    builder.add_parking_space_status_update(ParkingSpaceStatus::SensorTriggered, tx_list_fixed_size_maybe4[1]);
    builder.add_parking_space_status_update(ParkingSpaceStatus::Occupied, tx_list_fixed_size_maybe4[2]);
    builder.add_parking_space_status_update(ParkingSpaceStatus::Available, tx_list_fixed_size_maybe4[3]);
    builder.add_payment_transaction(rental_fee_to_owner, tx_list_fixed_size_maybe4[4]);
    */


    /* 
    driver reserves a parking space
        parking space status changes to reserved

    driver arrives at parking space, triggers sensor
        sensor triggers transaction, parking space status step 1

    driver confirms parking and arrival
        transaction, confirm step 2, parking status occupied

    driver leaves parking space, triggers sensor
        parking space status update, payment send to parking space owner

    process end, transactions processed and settled

    
    struct ParkingSpaceUpdate {
        previous_status: ParkingSpaceStatus,
        new_status: ParkingSpaceStatus,
    }

    another struct for payment transaction or optional field in ParkingSpaceUpdate

    
    */

//after driver reserves, next step is driver arrives at parking space, triggers sensor
    state_channel.process_transactions(&[
               StateChannelTransaction {
            parking_space_status: Some(ParkingSpaceStatus::SensorTriggered),
            reservation_duration: None,
            reserved_by: Some(driver_pubkey), //payer
            from: None,
            to: None,
            amount: None,
            sensor_data: None,
            program_id: Some(program_id),
            parking_space_pda: Some(parking_space_pda),
        },
    ]);

   
    let rpc_client = test_validator.get_rpc_client();
//    assert_eq!(get_rental_rate_from_pda(&rpc_client, &parking_space_pda).unwrap(), rental_rate_usdc);
 //assert parking space listing updates
}
