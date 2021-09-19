use zoon::{*, eprintln, format};
use std::borrow::Cow;
use shared::UpMsg;
use crate::connection::connection;

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn status() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

#[static_ref]
fn voting_owner_privkey() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn voter_pubkey() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: String) {
    status().set(Some(Cow::from(new_status)))
}

fn add_voter() {
    status().take();
    if voting_owner_privkey().map(String::is_empty) || voter_pubkey().map(String::is_empty) {
        status().set(Some(Cow::from("Sorry, invalid private key or PubKey.")));
        return;
    }
    Task::start(async {
        let msg = UpMsg::AddVoter { pub_key: voter_pubkey().get_cloned() };
        if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            eprintln!("add_voter request failed: {}", error);
            set_status(error);
        }
    });
}

fn set_voting_owner_privkey(private_key: String) {
    voting_owner_privkey().set_neq(private_key)
}

fn set_voter_pubkey(pub_key: String) {
    voter_pubkey().set_neq(pub_key)
}

pub fn voter_added(pub_key: String) {
    let pub_key_part = pub_key.chars().take(5).collect::<String>();
    set_status(format!("Voter {}*** added.", pub_key_part));
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
