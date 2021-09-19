use zoon::{*, eprintln};
use std::sync::Arc;
use shared::{self, UpMsg};
use crate::connection::connection;

mod view;

// ------ ------
//     Types
// ------ ------

struct Party {
    pub_key: String,
    name: String,
    votes: Mutable<i64>,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn parties() -> &'static MutableVec<Arc<Party>> {
    MutableVec::new()
}

#[static_ref]
fn deadline() -> &'static Mutable<Option<i64>> {
    Mutable::new(None)
}

// ------ ------
//   Commands
// ------ ------

pub fn request_parties() {
    Task::start(async {
        if let Err(error) = connection().send_up_msg(UpMsg::GetParties).await {
            eprintln!("request parties failed: {}", error);
        }
    });
}

pub fn request_deadline() {
    Task::start(async {
        if let Err(error) = connection().send_up_msg(UpMsg::GetDeadline).await {
            eprintln!("request deadline failed: {}", error);
        }
    });
}

pub fn convert_and_set_parties(new_parties: Vec<shared::Party>) {
    fn convert_parties(parties: Vec<shared::Party>) -> Vec<Arc<Party>> {
        parties.into_iter().map(|party| {
            Arc::new(Party {
                pub_key: party.pub_key,
                name: party.name,
                votes: Mutable::new(party.votes),
            })
        })
        .collect()
    }
    parties().lock_mut().replace_cloned(convert_parties(new_parties));
}

pub fn set_deadline(timestamp: i64) {
    deadline().set_neq(Some(timestamp));
}

pub fn set_votes(party_pub_key: String, votes: i64) {
    let parties = parties().lock_ref();
    let party = parties.iter().find(|party| party.pub_key == party_pub_key);
    if let Some(party) = party {
        party.votes.set_neq(votes);
    }
}

fn vote(party_pub_key: String, positive: bool) {
    Task::start(async move {
        let msg = UpMsg::Vote {
            party_pub_key,
            positive,
        };
        if let Err(error) = connection().send_up_msg(msg).await {
            eprintln!("voting failed: {}", error);
        }
    });
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
