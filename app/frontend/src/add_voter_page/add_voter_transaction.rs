use zoon::{eprintln, println, *};

use crate::{
    app,
    connection::{connection, wait_for_cor_id},
    solana_helpers,
};
use borsh::BorshDeserialize;
use shared::UpMsg;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use voting_program::{instruction as voting_instruction, state::VoterVotes};

pub fn create_and_send_transaction(voting_owner_keypair: Keypair, voter_pubkey: Pubkey) {
    let (add_voter_ix, voter_votes_pubkey) =
        voting_instruction::add_voter(&voting_owner_keypair.pubkey(), &voter_pubkey);
    println!("voter_votes_pubkey: {}", voter_votes_pubkey);

    super::set_status("Adding the voter...");

    Task::start(async move {
        let up_msg = UpMsg::GetAccount {
            account_pubkey: voter_votes_pubkey,
        };
        if let Ok(cor_id) = connection().send_up_msg(up_msg).await {
            wait_for_cor_id(cor_id).await;
            if let Some(Ok(account)) = app::account().lock_ref().as_ref() {
                let voter_votes_data = VoterVotes::try_from_slice(&account.data)
                    .expect("failed to deserialize VoterVotes account data");

                println!("voter_votes_account: {:#?}", account);
                println!("voter_votes_account data: {:#?}", voter_votes_data);

                return super::set_status("The voter is already registered.");
            }
        }

        let recent_blockhash = match solana_helpers::recent_blockhash().await {
            Ok(blockhash) => blockhash,
            Err(error) => {
                return super::set_status(error);
            }
        };
        let message = Message::new(&[add_voter_ix], None);
        let transaction = Transaction::new(&[&voting_owner_keypair], message, recent_blockhash);

        let up_msg = UpMsg::AddVoter {
            pubkey: voter_pubkey,
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
