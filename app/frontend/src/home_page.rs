use zoon::{*, eprintln};
use std::{sync::Arc, borrow::Cow};
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

fn convert_party(party: shared::Party) -> Arc<Party> {
    Arc::new(Party {
        pub_key: party.pub_key,
        name: party.name,
        votes: Mutable::new(party.votes),
    })
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn status() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

#[static_ref]
fn parties() -> &'static MutableVec<Arc<Party>> {
    MutableVec::new()
}

#[static_ref]
fn deadline() -> &'static Mutable<Option<i64>> {
    Mutable::new(None)
}

#[static_ref]
fn voter_private_key() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: String) {
    status().set(Some(Cow::from(new_status)))
}

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
    let new_parties = new_parties.into_iter().map(convert_party).collect();
    parties().lock_mut().replace_cloned(new_parties);
}

pub fn push_party(party: shared::Party) {
    parties().lock_mut().push_cloned(convert_party(party));
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

fn set_voter_private_key(private_key: String) {
    voter_private_key().set_neq(private_key)
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
