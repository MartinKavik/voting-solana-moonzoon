use borsh::BorshDeserialize;
use moon::{tokio::task, *};
use shared::*;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use voting_program::state::Party as PartyState;

mod solana_helpers;

mod init_voting_state;
use init_voting_state::init_voting_state;

async fn frontend() -> Frontend {
    Frontend::new().title("Voting").append_to_head(
        "
            <style>
                body {
                    background: black;
                }
            </style>
        ",
    )
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>, deadline: i64) {
    println!("{:?}", req);

    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        ..
    } = req;

    let down_msg = match up_msg {
        UpMsg::AddVoter {
            pubkey,
            transaction,
        } => {
            println!("Waiting for add_voter transaction...");

            let transaction_result = task::spawn_blocking(move || {
                // @TODO_QUESTION How to use `RpcClient::send_and_confirm_transaction_with_spinner`?
                solana_helpers::client().send_and_confirm_transaction(&transaction)
            })
            .await
            .expect("add_voter transaction task failed");

            let voter_pubkey_or_error = match transaction_result {
                Ok(_) => {
                    println!("add_voter transaction committed");
                    Ok(pubkey)
                }
                Err(error) => {
                    eprintln!("add_voter transaction rollbacked ({:?})", error);
                    Err(error.to_string())
                }
            };
            DownMsg::VoterAdded {
                voter_pubkey_or_error,
            }
        }
        UpMsg::AddParty {
            name,
            pubkey,
            transaction,
        } => {
            println!("Waiting for add_party transaction...");

            let transaction_result = task::spawn_blocking(move || {
                solana_helpers::client().send_and_confirm_transaction(&transaction)
            })
            .await
            .expect("add_party transaction task failed");

            let party_name_or_error = match transaction_result {
                Ok(_) => {
                    println!("add_party transaction committed");
                    let party = Party {
                        name: name.clone(),
                        pubkey,
                        votes: 0,
                    };
                    let down_msg = DownMsg::PartyAddedBroadcasted { party };
                    sessions::broadcast_down_msg(&down_msg, cor_id).await;
                    Ok(name)
                }
                Err(error) => {
                    eprintln!("add_party transaction rollbacked ({:?})", error);
                    Err(error.to_string())
                }
            };
            DownMsg::PartyAdded {
                party_name_or_error,
            }
        }
        UpMsg::GetParties => {
            let voting_state = solana_helpers::request_voting_state()
                .await
                .expect("request VotingState failed");
            println!("Party count: {}", voting_state.party_count);

            let party_pubkeys = (0..voting_state.party_count)
                .map(|party_index| {
                    let party_index_bytes = party_index.to_le_bytes();
                    let seeds = &[
                        b"party",
                        party_index_bytes.as_ref(),
                        solana_helpers::voting_state_pubkey().as_ref(),
                    ];
                    Pubkey::find_program_address(seeds, &voting_program::id()).0
                })
                .collect::<Vec<_>>();

            let party_pubkeys = Arc::new(party_pubkeys);
            let party_pubkeys_for_task = Arc::clone(&party_pubkeys);

            let party_accounts = task::spawn_blocking(move || {
                solana_helpers::client()
                    .get_multiple_accounts(&party_pubkeys_for_task)
                    .expect("get_party_accounts failed")
            })
            .await
            .expect("get_party_accounts task failed");

            let parties = party_accounts
                .into_iter()
                .enumerate()
                .map(|(index, party_account)| {
                    let party_state = PartyState::try_from_slice(&party_account.unwrap().data)
                        .expect("cannot deserialize Party");

                    let votes = i64::from(party_state.positive_votes)
                        - i64::from(party_state.negative_votes);
                    Party {
                        name: party_state.name,
                        pubkey: party_pubkeys[index],
                        votes,
                    }
                })
                .collect();
            DownMsg::PartiesLoaded { parties }
        }
        UpMsg::GetDeadline => DownMsg::DeadlineLoaded {
            timestamp: deadline,
        },
        UpMsg::Vote {
            party_pubkey,
            positive,
            transaction,
        } => {
            println!("Waiting for vote transaction...");

            let transaction_result = task::spawn_blocking(move || {
                solana_helpers::client().send_and_confirm_transaction(&transaction)
            })
            .await
            .expect("vote transaction task failed");

            let status = match transaction_result {
                Ok(_) => {
                    println!("vote transaction committed");
                    let down_msg = DownMsg::VotesChangedBroadcasted {
                        party_pubkey,
                        positive,
                    };
                    sessions::broadcast_down_msg(&down_msg, cor_id).await;
                    format!(
                        "{} voted for '{}***'.",
                        if positive { "Positively" } else { "Negatively" },
                        party_pubkey.to_string().chars().take(5).collect::<String>()
                    )
                }
                Err(error) => {
                    eprintln!("vote transaction rollbacked ({:?})", error);
                    error.to_string()
                }
            };
            DownMsg::VotesChanged { status }
        }
        UpMsg::GetRecentBlockhash => DownMsg::RecentBlockhashLoaded {
            blockhash: solana_helpers::request_recent_blockhash().await,
        },
        UpMsg::GetAccount { account_pubkey } => {
            let account = task::spawn_blocking(move || {
                solana_helpers::client()
                    .get_account(&account_pubkey)
                    .map_err(|error| error.to_string())
            })
            .await;
            let account = match account {
                Ok(account) => account,
                Err(error) => Err(error.to_string()),
            };
            DownMsg::AccountLoaded { account }
        }
    };

    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        session.send_down_msg(&down_msg, cor_id).await;
    } else {
        eprintln!("cannot find the session with id `{}`", session_id);
    }
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    println!(
        "voting_state_pubkey: {}",
        solana_helpers::voting_state_pubkey()
    );
    let voting_state = init_voting_state().await;
    println!("voting_state_account data {:?}", voting_state);
    let deadline = voting_state.deadline;

    let up_msg_handler_wrapper = move |req| up_msg_handler(req, deadline);
    start(frontend, up_msg_handler_wrapper, |_| {}).await
}
