use zoon::{*, println};
use shared::{UpMsg, DownMsg};
use crate::*;

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        println!("DownMsg received: {:?}", down_msg);
        match down_msg {
            _ => ()
        }
    })
}
