use moon::{once_cell::sync::OnceCell, tokio::task};
use solana_sdk::{
    signer::keypair::{Keypair, read_keypair},
    hash::Hash,
};
use solana_client::rpc_client::RpcClient;

pub fn voting_owner_keypair() -> &'static Keypair {
    static INSTANCE: OnceCell<Keypair> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let keypair_file_content = include_str!("../../../program/keypairs/voting-owner-keypair.json");
        read_keypair(&mut keypair_file_content.as_bytes()).expect("cannot parse voting-owner-keypair")
    })
}

pub fn client() -> &'static RpcClient {
    static INSTANCE: OnceCell<RpcClient> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        RpcClient::new("http://localhost:8899".to_owned())
    })
}

pub async fn request_recent_blockhash() -> Hash {
    task::spawn_blocking(|| {
        client().get_latest_blockhash()
        .expect("get_recent_blockhash failed")
    }).await.expect("get_recent_blockhash task failed")
}

pub async fn request_minimum_balance_for_rent_exemption(size: usize) -> u64 {
    task::spawn_blocking(move || {
        client().get_minimum_balance_for_rent_exemption(size)
        .expect("get_minimum_balance_for_rent_exemption failed")
    }).await.expect("get_minimum_balance_for_rent_exemption task failed")
}
