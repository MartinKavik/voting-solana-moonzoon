use solana_sdk::signer::keypair::read_keypair;
use std::borrow::Cow;
use zoon::{format, *};

mod add_party_transaction;
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
    // Mutable::new(String::new())
    // a hardcoded value for easier manual testing
    Mutable::new("[142,26,26,248,251,208,51,10,116,93,98,146,211,32,153,244,188,50,216,153,186,31,43,22,175,132,223,117,141,144,139,189,36,44,206,11,125,213,237,45,134,87,176,198,5,181,173,165,18,254,34,219,78,194,168,66,63,223,123,6,165,8,205,183]".to_owned())
}

#[static_ref]
fn party_name() -> &'static Mutable<String> {
    // Mutable::new(String::new())
    // a hardcoded value for easier manual testing
    Mutable::new("My Party".to_owned())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: impl Into<Cow<'static, str>>) {
    status().set(Some(new_status.into()))
}

fn add_party() {
    status().take();
    let fee_payer_keypair = fee_payer_private_key().lock_ref();
    let party_name = party_name().lock_ref();

    let fee_payer_keypair = read_keypair(&mut fee_payer_keypair.as_bytes());
    let fee_payer_keypair = if let Ok(keypair) = fee_payer_keypair {
        keypair
    } else {
        status().set(Some(Cow::from("Sorry, invalid private key.")));
        return;
    };
    if party_name.is_empty() {
        status().set(Some(Cow::from("Sorry, invalid name.")));
        return;
    }
    add_party_transaction::create_and_send_transaction(fee_payer_keypair, party_name.to_owned());
}

fn set_fee_payer_private_key(private_key: String) {
    fee_payer_private_key().set_neq(private_key)
}

fn set_party_name(name: String) {
    party_name().set_neq(name)
}

pub fn party_added(party_name_or_error: Result<String, String>) {
    match party_name_or_error {
        Ok(name) => {
            set_status(format!("Party '{}' added.", name));
            party_name().take();
        }
        Err(error) => {
            set_status(error);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
