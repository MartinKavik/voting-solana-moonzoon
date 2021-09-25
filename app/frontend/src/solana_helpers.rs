use crate::{
    app,
    connection::{connection, wait_for_cor_id},
};
use shared::UpMsg;
use solana_sdk::{hash::Hash, pubkey::Pubkey};
use std::str::FromStr;
use zoon::{eprintln, println, *};

#[static_ref]
pub fn voting_owner_pubkey() -> &'static Pubkey {
    let voting_owner_pubkey = include_str!("../../../program/keypairs/voting-owner-pubkey");
    Pubkey::from_str(voting_owner_pubkey.trim()).expect_throw("cannot parse voting-owner-pubkey")
}

#[static_ref]
pub fn voting_state_pubkey() -> &'static Pubkey {
    Pubkey::create_with_seed(voting_owner_pubkey(), "voting_state", &voting_program::id())
        .expect("failed to create voting_state_pubkey")
}

pub async fn recent_blockhash() -> Result<Hash, String> {
    match connection().send_up_msg(UpMsg::GetRecentBlockhash).await {
        Ok(cor_id) => {
            wait_for_cor_id(cor_id).await;
            let recent_blockhash = app::recent_blockhash().get().unwrap_throw();
            println!("recent_blockhash: {:#?}", recent_blockhash);
            Ok(recent_blockhash)
        }
        Err(error) => {
            let error = error.to_string();
            eprintln!("recent_blockhash request failed: {}", error);
            Err(error)
        }
    }
}
