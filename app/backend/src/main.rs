use moon::*;
use shared::*;

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
    let mut broadcast = false;

    let down_msg = match up_msg {
        UpMsg::AddVoter { pub_key } => DownMsg::VoterAdded { pub_key },
        UpMsg::AddParty { name } => DownMsg::PartyAdded { name },
        UpMsg::GetParties => DownMsg::PartiesLoaded {parties: vec![
            Party {
                name: "Party A".to_owned(),
                pub_key: (0..45).map(|_| 'A').collect(),
                votes: 0,
            },
            Party {
                name: "Party B".to_owned(),
                pub_key: (0..45).map(|_| 'B').collect(),
                votes: 1,
            },
            Party {
                name: "Party C".to_owned(),
                pub_key: (0..45).map(|_| 'C').collect(),
                votes: 2,
            },
        ]},
        UpMsg::GetDeadline => DownMsg::DeadlineLoaded { timestamp: 0 },
        UpMsg::Vote { party_pub_key, positive } => {
            broadcast = true;
            DownMsg::VotesChanged {
                party_pub_key,
                votes: if positive { 123 } else { -123 }
            }
        }
    };

    if broadcast {
        sessions::broadcast_down_msg(&down_msg, cor_id).await;
        return;
    }

    // @TODO [MoonZoon] backoff + jitter + queue or something else?
    let mut down_msg_sent = false;
    for i in 0..10 {
        if let Some(session) = sessions::by_session_id().get(session_id) {
            session.send_down_msg(&down_msg, cor_id).await;
            down_msg_sent = true;
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(i * 200)).await;
    }
    // @TODO [MoonZoon] not(..) helper
    if !down_msg_sent {
        eprintln!("cannot find the session with id `{}`", session_id);
    }
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    println!("Voting owner keypair: {}", include_str!("../../../program/keypairs/voting-owner-keypair.json"));
    println!("Program pub_key: {}", include_str!("../../../program/keypairs/program-pubkey"));
    start(frontend, up_msg_handler, |_| {}).await
}
