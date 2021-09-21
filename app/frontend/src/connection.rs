use zoon::{*, println};
use shared::{UpMsg, DownMsg};
use crate::*;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, _cor_id| {
        println!("DownMsg received: {:#?}", down_msg);
        match down_msg {
            DownMsg::VoterAdded { pubkey} => add_voter_page::voter_added(pubkey),
            DownMsg::PartyAdded { name } => add_party_page::party_added(name),
            DownMsg::PartyAddedBroadcasted { party } => parties_page::push_party(party),
            DownMsg::PartiesLoaded { parties } => parties_page::convert_and_set_parties(parties),
            DownMsg::DeadlineLoaded { timestamp } => parties_page::set_deadline(timestamp),
            DownMsg::VotesChanged { status } => parties_page::set_status(status),
            DownMsg::VotesChangedBroadcasted { party_pubkey , votes } => {
                parties_page::set_votes(party_pubkey, votes);
            },
        }
    })
}
