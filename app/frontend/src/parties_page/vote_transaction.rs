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

pub fn create_and_send_transaction(voter_keypair: Keypair, party_pubkey: Pubkey, positive: bool) {
    let voter_pubkey = voter_keypair.pubkey();

    let (vote_ix, voter_votes_pubkey, voter_voted_pubkey) = voting_instruction::vote(
        &voter_pubkey,
        solana_helpers::voting_state_pubkey(),
        &party_pubkey,
        positive,
    );

    super::set_status("Voting...");

    Task::start(async move {
        let up_msg = UpMsg::GetAccount {
            account_pubkey: voter_voted_pubkey,
        };
        match connection().send_up_msg(up_msg).await {
            Ok(cor_id) => {
                wait_for_cor_id(cor_id).await;
                if app::account().lock_ref().as_ref().unwrap_throw().is_ok() {
                    return super::set_status("You've already voted for this party.");
                }
            }
            Err(error) => {
                return super::set_status(error.to_string());
            }
        }

        let up_msg = UpMsg::GetAccount {
            account_pubkey: voter_votes_pubkey,
        };
        let voter_votes = match connection().send_up_msg(up_msg).await {
            Err(error) => {
                return super::set_status(error.to_string());
            }
            Ok(cor_id) => {
                wait_for_cor_id(cor_id).await;
                match app::account().lock_ref().as_ref().unwrap_throw() {
                    Err(error) => {
                        println!("cannot get voter_votes_account: {}", error);
                        return super::set_status("You are not eligible for voting.");
                    }
                    Ok(account) => VoterVotes::try_from_slice(&account.data)
                        .expect("failed to deserialize VoterVotes account data"),
                }
            }
        };

        if positive {
            if voter_votes.positive_votes == 0 {
                return super::set_status("You've already spent all positive votes.");
            }
        } else {
            if voter_votes.negative_votes == 0 {
                return super::set_status("You've already spent all negative votes.");
            }
            if voter_votes.positive_votes != 0 {
                return super::set_status(
                    "Positive votes have to be spent before the negative ones.",
                );
            }
        }

        let recent_blockhash = match solana_helpers::recent_blockhash().await {
            Ok(blockhash) => blockhash,
            Err(error) => {
                return super::set_status(error);
            }
        };
        let message = Message::new(&[vote_ix], None);
        let transaction = Transaction::new(&[&voter_keypair], message, recent_blockhash);

        let up_msg = UpMsg::Vote {
            party_pubkey,
            positive,
            transaction,
        };
        if let Err(error) = connection().send_up_msg(up_msg).await {
            let error = error.to_string();
            eprintln!("vote request failed: {}", error);
            super::set_status(error);
        }

        println!("vote transaction sent.");
    });
}
