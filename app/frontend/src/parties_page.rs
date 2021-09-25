use crate::connection::connection;
use shared::{self, UpMsg};
use solana_sdk::{pubkey::Pubkey, signer::keypair::read_keypair};
use std::{borrow::Cow, sync::Arc};
use zoon::{eprintln, *};

mod view;
mod vote_transaction;

// ------ ------
//     Types
// ------ ------

struct Party {
    pubkey: Pubkey,
    name: String,
    votes: Mutable<i64>,
}

fn convert_party(party: shared::Party) -> Arc<Party> {
    Arc::new(Party {
        pubkey: party.pubkey,
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
    // Mutable::new(String::new())
    // a hardcoded value for easier manual testing
    Mutable::new("[142,26,26,248,251,208,51,10,116,93,98,146,211,32,153,244,188,50,216,153,186,31,43,22,175,132,223,117,141,144,139,189,36,44,206,11,125,213,237,45,134,87,176,198,5,181,173,165,18,254,34,219,78,194,168,66,63,223,123,6,165,8,205,183]".to_owned())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: impl Into<Cow<'static, str>>) {
    status().set(Some(new_status.into()))
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

pub fn add_vote(party_pubkey: Pubkey, positive: bool) {
    let parties = parties().lock_ref();
    let party = parties.iter().find(|party| party.pubkey == party_pubkey);
    if let Some(party) = party {
        party
            .votes
            .update(|votes| if positive { votes + 1 } else { votes - 1 });
    }
}

fn vote(party_pubkey: Pubkey, positive: bool) {
    status().take();
    let voter_keypair = voter_private_key().lock_ref();

    let voter_keypair = read_keypair(&mut voter_keypair.as_bytes());
    let voter_keypair = if let Ok(keypair) = voter_keypair {
        keypair
    } else {
        set_status("Sorry, invalid private key.");
        return;
    };
    vote_transaction::create_and_send_transaction(voter_keypair, party_pubkey, positive);
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
