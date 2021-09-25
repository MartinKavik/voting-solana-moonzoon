use borsh::BorshDeserialize;
use moon::{once_cell::sync::OnceCell, tokio::task};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
    pubkey::Pubkey,
    signer::{
        keypair::{read_keypair, Keypair},
        Signer,
    },
};
use voting_program::{self, state::VotingState};

pub fn voting_owner_keypair() -> &'static Keypair {
    static INSTANCE: OnceCell<Keypair> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let keypair_file_content =
            include_str!("../../../program/keypairs/voting-owner-keypair.json");
        read_keypair(&mut keypair_file_content.as_bytes())
            .expect("cannot parse voting-owner-keypair")
    })
}

pub fn client() -> &'static RpcClient {
    static INSTANCE: OnceCell<RpcClient> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        // @TODO_QUESTION What are the best practices for setting the `commitment`?
        // @TODO_QUESTION Should I write a retry somewhere?
        RpcClient::new_with_commitment(
            "http://localhost:8899".to_owned(),
            CommitmentConfig::confirmed(),
        )
    })
}

pub fn voting_state_pubkey() -> &'static Pubkey {
    static INSTANCE: OnceCell<Pubkey> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Pubkey::create_with_seed(
            &voting_owner_keypair().pubkey(),
            "voting_state",
            &voting_program::id(),
        )
        .expect("failed to create voting_state_pubkey")
    })
}

pub async fn request_voting_state() -> Option<VotingState> {
    let voting_state_account =
        task::spawn_blocking(move || client().get_account(voting_state_pubkey()))
            .await
            .expect("get_acount VotingState task failed");

    if let Ok(account) = voting_state_account {
        let voting_state_data = VotingState::try_from_slice(&account.data)
            .expect("failed to deserialize VotingState account data");
        return Some(voting_state_data);
    }
    None
}

pub async fn request_recent_blockhash() -> Hash {
    task::spawn_blocking(|| {
        client()
            .get_latest_blockhash()
            .expect("get_recent_blockhash failed")
    })
    .await
    .expect("get_recent_blockhash task failed")
}

pub async fn request_minimum_balance_for_rent_exemption(size: usize) -> u64 {
    task::spawn_blocking(move || {
        client()
            .get_minimum_balance_for_rent_exemption(size)
            .expect("get_minimum_balance_for_rent_exemption failed")
    })
    .await
    .expect("get_minimum_balance_for_rent_exemption task failed")
}
