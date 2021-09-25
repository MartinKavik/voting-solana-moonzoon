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
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use voting_program::{instruction as voting_instruction, state::VotingState};

pub fn create_and_send_transaction(fee_payer_keypair: Keypair, party_name: String) {
    super::set_status("Adding the party...");

    Task::start(async move {
        let up_msg = UpMsg::GetAccount {
            account_pubkey: *solana_helpers::voting_state_pubkey(),
        };
        match connection().send_up_msg(up_msg).await {
            Err(error) => return super::set_status(error.to_string()),
            Ok(cor_id) => wait_for_cor_id(cor_id).await,
        };
        let party_count = match app::account().lock_ref().as_ref().unwrap_throw() {
            Err(error) => return super::set_status(error.clone()),
            Ok(account) => {
                let voting_state_data = VotingState::try_from_slice(&account.data)
                    .expect("failed to deserialize VotingState account data");

                println!("voting_state_account: {:#?}", account);
                println!("voting_state_account data: {:#?}", voting_state_data);
                voting_state_data.party_count
            }
        };

        let (add_party_ix, party_pubkey) = voting_instruction::add_party(
            &fee_payer_keypair.pubkey(),
            &party_name,
            party_count,
            solana_helpers::voting_state_pubkey(),
        );
        println!("party_pubkey: {}", party_pubkey);

        let recent_blockhash = match solana_helpers::recent_blockhash().await {
            Ok(blockhash) => blockhash,
            Err(error) => {
                return super::set_status(error);
            }
        };
        let message = Message::new(&[add_party_ix], None);
        let transaction = Transaction::new(&[&fee_payer_keypair], message, recent_blockhash);

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
