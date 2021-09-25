use solana_sdk::{pubkey::Pubkey, signer::keypair::read_keypair};
use std::{borrow::Cow, str::FromStr};
use zoon::{format, *};

mod add_voter_transaction;
mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn status() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

#[static_ref]
fn voting_owner_private_key() -> &'static Mutable<String> {
    // Mutable::new(String::new())
    // a hardcoded value for easier manual testing
    Mutable::new("[174,64,61,34,218,120,154,130,8,130,196,244,149,216,51,1,142,0,23,172,162,125,63,23,184,48,41,64,54,74,176,60,193,36,90,212,230,161,186,128,229,189,103,204,231,108,156,66,63,179,123,75,186,241,81,146,103,28,125,167,34,192,55,136]".to_owned())
}

#[static_ref]
fn voter_pubkey() -> &'static Mutable<String> {
    // Mutable::new(String::new())
    // a hardcoded value for easier manual testing
    Mutable::new("3SDJZWXFwbkSCy2tVrEpffG7Vq9dxTxguAM3vvjewr1L".to_owned())
}

// ------ ------
//   Commands
// ------ ------

pub fn set_status(new_status: impl Into<Cow<'static, str>>) {
    status().set(Some(new_status.into()))
}

fn add_voter() {
    status().take();
    let voting_owner_keypair = voting_owner_private_key().lock_ref();
    let voter_pubkey = voter_pubkey().lock_ref();

    let voting_owner_keypair =
        if let Ok(keypair) = read_keypair(&mut voting_owner_keypair.as_bytes()) {
            keypair
        } else {
            set_status("Sorry, invalid private key.");
            return;
        };
    let voter_pubkey = if let Ok(pubkey) = Pubkey::from_str(&voter_pubkey) {
        pubkey
    } else {
        set_status("Sorry, invalid PubKey.");
        return;
    };
    add_voter_transaction::create_and_send_transaction(voting_owner_keypair, voter_pubkey);
}

fn set_voting_owner_private_key(private_key: String) {
    voting_owner_private_key().set_neq(private_key)
}

fn set_voter_pubkey(pubkey: String) {
    voter_pubkey().set_neq(pubkey)
}

pub fn voter_added(voter_pubkey_or_error: Result<Pubkey, String>) {
    match voter_pubkey_or_error {
        Ok(pubkey) => {
            let pubkey_part = pubkey.to_string().chars().take(5).collect::<String>();
            set_status(format!("Voter '{}***' added.", pubkey_part));
            voter_pubkey().take();
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
