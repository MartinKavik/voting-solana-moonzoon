use zoon::{*, println};
use shared::{UpMsg, DownMsg};
use crate::*;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        println!("DownMsg received: {:#?}", down_msg);
        match down_msg {
            DownMsg::VoterAdded { pub_key} => add_voter_page::voter_added(pub_key),
            DownMsg::PartyAdded { name} => add_party_page::party_added(name),
            DownMsg::PartiesLoaded { parties } => home_page::convert_and_set_parties(parties),
            DownMsg::DeadlineLoaded { timestamp } => home_page::set_deadline(timestamp),
            DownMsg::VotesChanged { party_pub_key , votes } => {
                home_page::set_votes(party_pub_key, votes);
            },
        }
    })
}
