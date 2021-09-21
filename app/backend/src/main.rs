use moon::*;
use shared::*;

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

async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{:?}", req);

    let UpMsgRequest { up_msg, cor_id, session_id, .. } = req;

    let down_msg = match up_msg {
        UpMsg::AddVoter { pubkey } => DownMsg::VoterAdded { pubkey },
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
            let deadline = Local::now() + Duration::days(7);
            DownMsg::DeadlineLoaded { timestamp: deadline.timestamp() }
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
    init_voting_state().await;
    start(frontend, up_msg_handler, |_| {}).await
}
