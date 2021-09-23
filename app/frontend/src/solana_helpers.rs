use zoon::{*, println, eprintln};
use shared::UpMsg;
use crate::{app, connection::{connection, wait_for_cor_id}};
use solana_sdk::hash::Hash;

pub async fn recent_blockhash() -> Result<Hash, String> {
     match connection().send_up_msg(UpMsg::GetRecentBlockhash).await {
        Ok(cor_id) => {
            wait_for_cor_id(cor_id).await;
            let recent_blockhash = app::recent_blockhash().get().unwrap_throw();
            println!("recent_blockhash: {:#?}", recent_blockhash);
            Ok(recent_blockhash)
        },
        Err(error) => {
            let error = error.to_string();
            eprintln!("recent_blockhash request failed: {}", error);
            Err(error)
        }
    }
}
