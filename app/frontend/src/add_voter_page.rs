use zoon::{*, eprintln};
use std::borrow::Cow;
use shared::UpMsg;

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn add_voter_error() -> &'static Mutable<Option<Cow<'static, str>>> {
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

pub fn set_add_voter_error(error: String) {
    add_voter_error().set(Some(Cow::from(error)))
}

fn add_voter() {
    add_voter_error().take();
    if voting_owner_privkey().map(String::is_empty) || voter_pubkey().map(String::is_empty) {
        add_voter_error().set(Some(Cow::from("Sorry, invalid private key or PubKey.")));
        return;
    }
    Task::start(async {
        zoon::println!("@TODO Add Voter");
        // let msg = UpMsg::AddVoter {
        //     name: name().get_cloned(),
        //     password: password().get_cloned(),
        // };
        // if let Err(error) = connection().send_up_msg(msg).await {
        //     let error = error.to_string();
        //     eprintln!("add_voter request failed: {}", error);
        //     set_add_voter_error(error);
        // }
    });
}

fn set_voting_owner_privkey(private_key: String) {
    voting_owner_privkey().set_neq(private_key)
}

fn set_voter_pubkey(pub_key: String) {
    voter_pubkey().set_neq(pub_key)
}

pub fn voter_added(pub_key: String) {
    // @TODO, call from connection
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
