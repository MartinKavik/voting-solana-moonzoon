use zoon::{*, println, eprintln};

use solana_sdk::{
    pubkey::Pubkey,
    transaction::Transaction,
    message::Message,
    signer::{Signer, keypair::Keypair},
};
use voting_program::{instruction as voting_instruction, state::VoterVotes};
use shared::UpMsg;
use crate::{connection::{connection, wait_for_cor_id}, app};
use borsh::BorshDeserialize;

pub fn create_and_send_transaction(voter_keypair: Keypair, party_pubkey: Pubkey, positive: bool) {
    println!("vote transaction wip");


    // let voting_owner_pubkey = voting_owner_keypair.pubkey();

    // let seeds = &[b"voter_votes", voter_pubkey.as_ref(), voting_owner_pubkey.as_ref()];
    // let voter_votes_pubkey = Pubkey::find_program_address(seeds, &voting_program::id()).0;
    // println!("voter_votes_pubkey: {}", voter_votes_pubkey);

    // let add_voter_ix = voting_instruction::add_voter(
    //     &voting_owner_pubkey, 
    //     &voter_votes_pubkey,
    //     &voter_pubkey
    // );

    // super::set_status("Adding the voter...");

    // Task::start(async move {
    //     let up_msg = UpMsg::GetAccount {
    //         account_pubkey: voter_votes_pubkey,
    //     };
    //     if let Ok(cor_id) = connection().send_up_msg(up_msg).await {
    //         wait_for_cor_id(cor_id).await;
    //         if let Some(Ok(account)) = app::account().lock_ref().as_ref() {
    //             let voter_votes_data = VoterVotes::try_from_slice(&account.data)
    //                 .expect("failed to deserialize VoterVotes account data");

    //             println!("voter_votes_account: {:#?}", account);
    //             println!("voter_votes_account data: {:#?}", voter_votes_data);

    //             return super::set_status("The voter is already registered.");
    //         }
    //     }

    //     let recent_blockhash = match connection().send_up_msg(UpMsg::GetRecentBlockhash).await {
    //         Ok(cor_id) => {
    //             wait_for_cor_id(cor_id).await;
    //             let recent_blockhash = app::recent_blockhash().get().unwrap_throw();
    //             println!("recent_blockhash: {:#?}", recent_blockhash);
    //             recent_blockhash
    //         },
    //         Err(error) => {
    //             let error = error.to_string();
    //             eprintln!("recent_blockhash request failed: {}", error);
    //             return super::set_status(error);
    //         }
    //     };

    //     let message = Message::new(
    //         &[add_voter_ix], 
    //         None
    //     );
    //     let transaction = Transaction::new(
    //         &[&voting_owner_keypair], 
    //         message, 
    //         recent_blockhash
    //     );

    //     let up_msg = UpMsg::AddVoter {
    //         pubkey: voter_pubkey,
    //         transaction,
    //     };
    //     if let Err(error) = connection().send_up_msg(up_msg).await {
    //         let error = error.to_string();
    //         eprintln!("add_voter request failed: {}", error);
    //         super::set_status(error);
    //     }
    
    //     println!("add_voter transaction sent.");
    // });
}
