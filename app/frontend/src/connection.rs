use zoon::{*, println};
use shared::{UpMsg, DownMsg};
use crate::*;

// ------ connection ------

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        println!("DownMsg received: {:#?}", down_msg);
        match down_msg {
            DownMsg::VoterAdded { voter_pubkey_or_error} => {
                add_voter_page::voter_added(voter_pubkey_or_error)
            },
            DownMsg::PartyAdded { name } => add_party_page::party_added(name),
            DownMsg::PartyAddedBroadcasted { party } => parties_page::push_party(party),
            DownMsg::PartiesLoaded { parties } => parties_page::convert_and_set_parties(parties),
            DownMsg::DeadlineLoaded { timestamp } => parties_page::set_deadline(timestamp),
            DownMsg::VotesChanged { status } => parties_page::set_status(status),
            DownMsg::VotesChangedBroadcasted { party_pubkey , votes } => {
                parties_page::set_votes(party_pubkey, votes);
            },
            DownMsg::RecentBlockhashLoaded { blockhash } => {
                app::set_recent_blockhash(blockhash);
            }
        }
        recent_cor_id().set_neq(Some(cor_id));
    })
}

// ------ wait_for_cor_id ------

#[static_ref]
fn recent_cor_id() -> &'static Mutable<Option<CorId>> {
    Mutable::new(None)
}

pub async fn wait_for_cor_id(cor_id: CorId) {
    recent_cor_id().signal().wait_for(Some(cor_id)).await;
}
