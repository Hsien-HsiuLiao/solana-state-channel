mod setup;
mod create_listing;


use {
    mini_rollup::{transaction::{StateChannelTransaction, ParkingSpaceStatus}, StateChannel},
    setup::{system_account, TestValidatorContext},
    create_listing::{create_parking_space_listing, get_rental_rate_from_pda},
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},

};

#[test]
fn test_parking_tx() {
    let homeowner = Keypair::new();
    let driver = Keypair::new();
    
    let homeowner_pubkey = homeowner.pubkey();
    let driver_pubkey = driver.pubkey();

    let program_id = Pubkey::new_unique();

    // Create parking space listing PDA account with rental rate stored
    let rental_rate_usdc = 10_000_000; // 10 USDC
    let (parking_space_pda, parking_space_account) = create_parking_space_listing(
        &homeowner_pubkey,
        rental_rate_usdc,
        &program_id,
        1_000_000, // lamports for rent exemption
    );

    let accounts = vec![
        (homeowner_pubkey, system_account(10_000_000)),
        (driver_pubkey, system_account(10_000_000)),
        (parking_space_pda, parking_space_account.clone()), 
    ];

   
    let context = TestValidatorContext::start_with_accounts(accounts);
    let test_validator = &context.test_validator;
    let payer = context.payer.insecure_clone();

    let rpc_client = test_validator.get_rpc_client();

    //homeowner creates a parking space listing PDA with  rental rate
   // let parking_space = ParkingSpace::new(homeowner_pubkey, "123 Main St", 100, 100);
   let actual_rental_rate = get_rental_rate_from_pda(&rpc_client, &parking_space_pda)
   .expect("PDA account should exist");
   
    assert_eq!(actual_rental_rate, rental_rate_usdc, 
    "Expected rental rate {} but got {}", rental_rate_usdc, actual_rental_rate);
    //driver reserves a parking space
    //listingPda parking space status changes to reserved, reservation length is updated, reserved_by is updated

    //opens a channel
    let state_channel = StateChannel::new(vec![payer, homeowner, driver], rpc_client);

   /*  let mut builder = TransactionBuilder::new(rpc_client);
   //tx1 - driver arrives at parking space, triggers sensor
   //tx2 - driver confirms parking and arrival
   //tx3 - driver leaves parking space, triggers sensor
   //tx4 - homeowner receives payment, channel closed and txns processed and settled
    builder.add_svm_transaction(tx1);
    builder.add_svm_transaction(tx2);
    builder.add_svm_transaction(tx3);
    builder.add_svm_transaction(tx4);
    builder.process(&channel); */


    /* 
    driver reserves a parking space
        parking space status changes to reserved

    driver arrives at parking space, triggers sensor
        sensor triggers transaction, parking space status step 1

    driver confirms parking and arrival
        transaction, confirm step 2, parking status occupied

    driver leaves parking space, triggers sensor
        parking space status update, payment send to parking space owner

    process end, mini-rollup closed, how?

    
    struct ParkingSpaceUpdate {
        previous_status: ParkingSpaceStatus,
        new_status: ParkingSpaceStatus,
    }

    another struct for payment transaction or optional field in ParkingSpaceUpdate

    
    */


    state_channel.process_transactions(&[
               StateChannelTransaction {
            parking_space_status: Some(ParkingSpaceStatus::Reserved),
            reservation_duration: Some(1_000_000),
            reserved_by: Some(driver_pubkey),
            from: None,
            to: None,
            amount: None,
            sensor_data: None,
        },
    ]);

   
    let rpc_client = test_validator.get_rpc_client();
//    assert_eq!(get_rental_rate_from_pda(&rpc_client, &parking_space_pda).unwrap(), rental_rate_usdc);
 //assert parking space listing updates
}
