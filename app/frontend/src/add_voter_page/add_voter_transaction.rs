use zoon::{*, println, eprintln};

use solana_sdk::{
    pubkey::Pubkey,
    transaction::Transaction,
    message::Message,
    signer::{Signer, keypair::Keypair},
};
use voting_program::instruction as voting_instruction;
use shared::UpMsg;
use crate::{connection::{connection, wait_for_cor_id}, app};

pub fn create_and_send_transaction(voting_owner_keypair: Keypair, voter_pubkey: Pubkey) {
    let voting_owner_pubkey = voting_owner_keypair.pubkey();

    let seeds = &[b"voter_votes", voter_pubkey.as_ref(), voting_owner_pubkey.as_ref()];
    let voter_votes_pubkey = Pubkey::find_program_address(seeds, &voting_program::id()).0;
    println!("voter_votes_pubkey: {}", voter_votes_pubkey);

    // @TODO Check if the account voter_votes already exists. Then set an error status and return.

    let add_voter_ix = voting_instruction::add_voter(
        &voting_owner_pubkey, 
        &voter_votes_pubkey,
        &voter_pubkey
    );

    Task::start(async move {
        let recent_blockhash = match connection().send_up_msg(UpMsg::RecentBlockhash).await {
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
            &[add_voter_ix], 
            None
        );
        let transaction = Transaction::new(
            &[&voting_owner_keypair], 
            message, 
            recent_blockhash
        );

        let up_msg = UpMsg::AddVoter {
            voter_pubkey,
            transaction,
        };
        if let Err(error) = connection().send_up_msg(up_msg).await {
            let error = error.to_string();
            eprintln!("add_voter request failed: {}", error);
            super::set_status(error);
        }
    
        println!("add_voter transaction sent.");
    });
}
