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
fn fee_payer_private_key() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn party_name() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: String) {
    status().set(Some(Cow::from(new_status)))
}

fn add_party() {
    status().take();
    if fee_payer_private_key().map(String::is_empty) || party_name().map(String::is_empty) {
        status().set(Some(Cow::from("Sorry, invalid private key or name.")));
        return;
    }
    Task::start(async {
        let msg = UpMsg::AddParty { name: party_name().get_cloned() };
        if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            eprintln!("add_party request failed: {}", error);
            set_status(error);
        }
    });
}

fn set_fee_payer_private_key(private_key: String) {
    fee_payer_private_key().set_neq(private_key)
}

fn set_party_name(name: String) {
    party_name().set_neq(name)
}

pub fn party_added(name: String) {
    set_status(format!("Party '{}' added.", name));
    party_name().take();
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
