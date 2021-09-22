use moon::{*, tokio::task};
use shared::*;

mod solana_helpers;

mod init_voting_state;
use init_voting_state::init_voting_state;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Voting")
        .append_to_head("
            <style>
                body {
                    background: black;
                }
            </style>
        ")
}

async fn up_msg_handler(req: UpMsgRequest<UpMsg>, deadline: i64) {
    println!("{:?}", req);

    let UpMsgRequest { up_msg, cor_id, session_id, .. } = req;

    let down_msg = match up_msg {
        UpMsg::AddVoter { voter_pubkey, transaction } => {
            println!("Waiting for add_voter transaction...");
            let transaction_result = task::spawn_blocking(move || {
                solana_helpers::client().send_and_confirm_transaction(&transaction)
            }).await.expect("add_voter transaction task failed");
            let voter_pubkey_or_error = match transaction_result {
                Ok(_) => {
                    println!("add_voter transaction committed");
                    Ok(voter_pubkey)
                },
                Err(error) => {
                    eprintln!("add_voter transaction rollbacked ({:?})", error);
                    Err(error.to_string())
                },
            };
            DownMsg::VoterAdded { voter_pubkey_or_error }
        },
        UpMsg::AddParty { name } => {
            let party = Party {
                name: name.clone(),
                pubkey: "11111NEWPARTY1111111111".to_owned(),
                votes: 0,
            };
            let down_msg = DownMsg::PartyAddedBroadcasted { party };
            sessions::broadcast_down_msg(&down_msg, cor_id).await;
            DownMsg::PartyAdded { name }
        },
        UpMsg::GetParties => DownMsg::PartiesLoaded {parties: vec![
            Party {
                name: "Party A".to_owned(),
                pubkey: (0..45).map(|_| 'A').collect(),
                votes: 0,
            },
            Party {
                name: "Party B".to_owned(),
                pubkey: (0..45).map(|_| 'B').collect(),
                votes: 1,
            },
            Party {
                name: "Party C".to_owned(),
                pubkey: (0..45).map(|_| 'C').collect(),
                votes: -2,
            },
        ]},
        UpMsg::GetDeadline => {
            DownMsg::DeadlineLoaded { timestamp: deadline }
        },
        UpMsg::Vote { party_pubkey, positive } => {
            let down_msg = DownMsg::VotesChangedBroadcasted {
                party_pubkey: party_pubkey.clone(),
                votes: if positive { 123 } else { -123 }
            };
            sessions::broadcast_down_msg(&down_msg, cor_id).await;

            let status = format!(
                "{} voted for '{}***'.",
                if positive { "Positively" } else { "Negatively" }, 
                party_pubkey.chars().take(5).collect::<String>()
            );
            DownMsg::VotesChanged { status }
        },
        UpMsg::RecentBlockhash => {
            DownMsg::RecentBlockhashLoaded { blockhash: solana_helpers::request_recent_blockhash().await }
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
    let voting_state = init_voting_state().await;
    let deadline = voting_state.deadline;

    let up_msg_handler_wrapper = move |req| {
        up_msg_handler(req, deadline)
    };
    start(frontend, up_msg_handler_wrapper, |_| {}).await
}
