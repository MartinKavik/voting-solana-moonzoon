use zoon::{*, println, eprintln};

use solana_sdk::{
    pubkey::Pubkey,
    transaction::Transaction,
    message::Message,
    signer::keypair::Keypair,
};
use voting_program::{instruction as voting_instruction, state::{VotingState}};
use shared::UpMsg;
use crate::{connection::{connection, wait_for_cor_id}, app};
use borsh::BorshDeserialize;
use std::str::FromStr;

#[static_ref]
fn voting_owner_pubkey() -> &'static Pubkey {
    let voting_owner_pubkey = include_str!("../../../../program/keypairs/voting-owner-pubkey");
    Pubkey::from_str(voting_owner_pubkey.trim()).expect_throw("cannot parse voting-owner-pubkey")
}

#[static_ref]
fn voting_state_pubkey() -> &'static Pubkey {
    Pubkey::create_with_seed(
        voting_owner_pubkey(),
        "voting_state",
        &voting_program::id(),
    ).expect("failed to create voting_state_pubkey")
}

pub fn create_and_send_transaction(fee_payer_keypair: Keypair, party_name: String) {
    super::set_status("Adding the party...");

    Task::start(async move {
        let up_msg = UpMsg::GetAccount {
            account_pubkey: *voting_state_pubkey(),
        };
        match connection().send_up_msg(up_msg).await {
            Err(error) => return super::set_status(error.to_string()),
            Ok(cor_id) => wait_for_cor_id(cor_id).await,
        };
        let new_party_index_bytes = match app::account().lock_ref().as_ref().unwrap_throw() {
            Err(error) => return super::set_status(error.to_owned()),
            Ok(account) => {
                let voting_state_data = VotingState::try_from_slice(&account.data)
                .expect("failed to deserialize VotingState account data");

                println!("voting_state_account: {:#?}", account);
                println!("voting_state_account data: {:#?}", voting_state_data);
                voting_state_data.party_count.to_le_bytes()
            }
        };

        // @TODO_QUESTION: How to represent a `Vec<Party>` on chain?  
        // @TODO_QUESTION: Can I create a Party account without the need 
        // to define the new index outside of transaction to prevent conflicts?  
        let seeds = &[b"party", new_party_index_bytes.as_ref(), voting_state_pubkey().as_ref()];
        let party_pubkey = Pubkey::find_program_address(seeds, &voting_program::id()).0;
        println!("party_pubkey: {}", party_pubkey);

        let add_party_ix = voting_instruction::add_party(
            &party_pubkey, 
            &party_name,
            &voting_state_pubkey()
        );

        let recent_blockhash = match connection().send_up_msg(UpMsg::GetRecentBlockhash).await {
            Ok(cor_id) => {
                wait_for_cor_id(cor_id).await;
                let recent_blockhash = app::recent_blockhash().get().unwrap_throw();
                println!("recent_blockhash: {:#?}", recent_blockhash);
                recent_blockhash
            },
            Err(error) => {
                let error = error.to_string();
                eprintln!("recent_blockhash request failed: {}", error);
                return super::set_status(error);
            }
        };

        let message = Message::new(
            &[add_party_ix], 
            None
        );
        let transaction = Transaction::new(
            &[&fee_payer_keypair], 
            message, 
            recent_blockhash
        );

        let up_msg = UpMsg::AddParty {
            name: party_name,
            pubkey: party_pubkey,
            transaction,
        };
        if let Err(error) = connection().send_up_msg(up_msg).await {
            let error = error.to_string();
            eprintln!("add_party request failed: {}", error);
            super::set_status(error);
        }
    
        println!("add_party transaction sent.");
    });
}
