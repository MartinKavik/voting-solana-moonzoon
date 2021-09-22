use moon::tokio::task;
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    message::Message,
    signature::Signer,
};
use voting_program::{self, state::VotingState, instruction as voting_instruction};
use borsh::BorshDeserialize;
use crate::solana_helpers;

async fn request_voting_state(voting_state_pubkey: Pubkey) -> Option<VotingState> {
    let voting_state_account = task::spawn_blocking(move || {
        solana_helpers::client().get_account(&voting_state_pubkey)
    }).await.expect("get_acount VotingState task failed");

    if let Ok(account) = voting_state_account {
        let voting_state_data = VotingState::try_from_slice(&account.data)
            .expect("failed to deserialize VotingState account data");

        println!("voting_state_account {:?}", account);
        println!("voting_state_account data {:?}", voting_state_data);

        return Some(voting_state_data);
    }
    None
}

pub async fn init_voting_state() -> VotingState { 
    let voting_owner_pubkey = solana_helpers::voting_owner_keypair().pubkey();

    let voting_state_pubkey_seed = "voting_state";
    let voting_state_pubkey = Pubkey::create_with_seed(
        &voting_owner_pubkey,
        voting_state_pubkey_seed,
        &voting_program::id(),
    ).expect("failed to create voting_state_pubkey");
    println!("voting_state_pubkey: {}", voting_state_pubkey);

    if let Some(voting_state) = request_voting_state(voting_state_pubkey).await {
        return voting_state;
    }

    let voting_state_size = VotingState::serialized_size();
    // @TODO_QUESTION is `system_instruction::create_account_with_seed` ok 
    // or is it better to create an account in a program with `Pubkey::find_program_address` + invoke_signed?
    let create_voting_state_account_ix = system_instruction::create_account_with_seed(
        &voting_owner_pubkey, 
        &voting_state_pubkey, 
        &voting_owner_pubkey, 
        voting_state_pubkey_seed, 
        solana_helpers::request_minimum_balance_for_rent_exemption(voting_state_size).await as u64, 
        voting_state_size as u64, 
        &voting_program::id(),
    );

    let init_voting_ix = voting_instruction::init_voting(
        &voting_owner_pubkey, 
        &voting_state_pubkey
    );
    
    let recent_blockhash = solana_helpers::request_recent_blockhash().await;
    println!("recent_blockhash: {}", recent_blockhash);

    let message = Message::new(
        &[create_voting_state_account_ix, init_voting_ix], 
        None
    );
    let transaction = Transaction::new(
        &[solana_helpers::voting_owner_keypair()], 
        message, 
        recent_blockhash
    );

    println!("Waiting for init_voting transaction...");
    task::spawn_blocking(move || {
        solana_helpers::client().send_and_confirm_transaction(&transaction).expect("init_voting transaction failed");
    }).await.expect("init_voting transaction task failed");

    println!("VotingState initialized.");

    request_voting_state(voting_state_pubkey).await.expect("request VotingState failed")
}
