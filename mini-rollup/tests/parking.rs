mod setup;
mod create_listing;


use {
    mini_rollup::{transaction::StateChannelTransaction, StateChannel},
    setup::{system_account, TestValidatorContext},
    create_listing::create_parking_space_listing,
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
        (parking_space_pda, parking_space_account), 
    ];

   
    let context = TestValidatorContext::start_with_accounts(accounts);
    let test_validator = &context.test_validator;
    let payer = context.payer.insecure_clone();

    let rpc_client = test_validator.get_rpc_client();

    //homeowner creates a parking space listing PDA with  rental rate
   // let parking_space = ParkingSpace::new(homeowner_pubkey, "123 Main St", 100, 100);

    //opens a channel
    let state_channel = StateChannel::new(vec![payer, homeowner, driver], rpc_client);

   /*  let mut builder = TransactionBuilder::new(rpc_client);
    builder.add_svm_transaction(tx1);
    builder.add_svm_transaction(tx2);
    builder.process(&channel); */
/*

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


    paytube_channel.process_paytube_transfers(&[
               PayTubeTransaction {
            from: will_pubkey,
            to: alice_pubkey,
            amount: 1_000_000,
            mint: None,
        },
    ]);

    // Ledger:
    // Alice:   10_000_000 - 2_000_000 - 2_000_000 + 1_000_000  = 7_000_000
    // Bob:     10_000_000 + 2_000_000 - 5_000_000 + 2_000_000  = 9_000_000
    // Will:    10_000_000 + 5_000_000 - 1_000_000              = 14_000_000
    let rpc_client = test_validator.get_rpc_client();
    assert_eq!(rpc_client.get_balance(&alice_pubkey).unwrap(), 7_000_000);
    assert_eq!(rpc_client.get_balance(&bob_pubkey).unwrap(), 9_000_000);
    assert_eq!(rpc_client.get_balance(&will_pubkey).unwrap(), 14_000_000); */
}
