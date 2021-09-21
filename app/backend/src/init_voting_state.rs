use moon::{once_cell::sync::OnceCell, tokio::task};
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    message::Message,
    signer::{Signer, keypair::{Keypair, read_keypair}},
    hash::Hash,
};
use solana_client::rpc_client::RpcClient;
use voting_program::{state::VotingState, instruction as voting_instruction};
use std::{mem, str::FromStr};
use borsh::BorshDeserialize;

fn program_pubkey() -> &'static Pubkey {
    static INSTANCE: OnceCell<Pubkey> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let pubkey_file_content = include_str!("../../../program/keypairs/program-pubkey");
        Pubkey::from_str(pubkey_file_content.trim()).expect("cannot parse program-pubkey")
    })
}

fn voting_owner_keypair() -> &'static Keypair {
    static INSTANCE: OnceCell<Keypair> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let keypair_file_content = include_str!("../../../program/keypairs/voting-owner-keypair.json");
        read_keypair(&mut keypair_file_content.as_bytes()).expect("cannot parse voting-owner-keypair")
    })
}

fn solana_client() -> &'static RpcClient {
    static INSTANCE: OnceCell<RpcClient> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        RpcClient::new("http://localhost:8899".to_owned())
    })
}

async fn request_recent_blockhash() -> Hash {
    task::spawn_blocking(|| {
        solana_client().get_recent_blockhash()
        .expect("get_recent_blockhash failed").0
    }).await.expect("get_recent_blockhash task failed")
}

async fn request_minimum_balance_for_rent_exemption(size: usize) -> u64 {
    task::spawn_blocking(move || {
        solana_client().get_minimum_balance_for_rent_exemption(size)
        .expect("get_minimum_balance_for_rent_exemption failed")
    }).await.expect("get_minimum_balance_for_rent_exemption task failed")
}

pub async fn init_voting_state() { 
    let voting_owner_pubkey = voting_owner_keypair().pubkey();

    let voting_state_pubkey_seed = "voting_state";
    let voting_state_pubkey = Pubkey::create_with_seed(
        &voting_owner_pubkey,
        voting_state_pubkey_seed,
        program_pubkey(),
    ).expect("failed to create voting_state_pubkey");
    println!("voting_state_pubkey: {}", voting_state_pubkey);

    let voting_state_account = task::spawn_blocking(move || {
        solana_client().get_account(&voting_state_pubkey)
    }).await.expect("get_acount VotingState task failed");
    if let Ok(account) = voting_state_account {
        println!("voting_state_account {:?}", account);
        println!(
            "voting_state_account data {:?}", 
            VotingState::try_from_slice(&account.data)
                .expect("failed to deserialize VotingState account data")
        );
        return;
    }

    let voting_state_size = VotingState::serialized_size();
    let create_voting_state_account_ix = system_instruction::create_account_with_seed(
        &voting_owner_pubkey, 
        &voting_state_pubkey, 
        &voting_owner_pubkey, 
        voting_state_pubkey_seed, 
        request_minimum_balance_for_rent_exemption(voting_state_size).await as u64, 
        voting_state_size as u64, 
        program_pubkey(),
    );

    let init_voting_ix = voting_instruction::init_voting(
        &voting_owner_pubkey, 
        &voting_state_pubkey
    );
    
    let recent_blockhash = request_recent_blockhash().await;
    println!("recent_blockhash: {}", recent_blockhash);

    let message = Message::new(
        &[create_voting_state_account_ix, init_voting_ix], 
        None
    );
    let transaction = Transaction::new(
        &[voting_owner_keypair()], 
        message, 
        recent_blockhash
    );

    println!("Waiting for init_voting transaction...");
    task::spawn_blocking(move || {
        solana_client().send_and_confirm_transaction(&transaction).expect("init_voting transaction failed");
    }).await.expect("init_voting transaction task failed");

    println!("VotingState initialized.")
}
