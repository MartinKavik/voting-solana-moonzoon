use moonlight::*;
use solana_sdk::{
    hash::Hash,
    transaction::Transaction,
    pubkey::Pubkey,
};

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    AddVoter { voter_pubkey: Pubkey, transaction: Transaction },
    AddParty { name: String },
    GetParties,
    GetDeadline,
    Vote { party_pubkey: String, positive: bool },
    RecentBlockhash,
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    VoterAdded { voter_pubkey_or_error: Result<Pubkey, String> },
    PartyAdded { name: String },
    PartyAddedBroadcasted { party: Party },
    PartiesLoaded { parties: Vec<Party> },
    DeadlineLoaded { timestamp: i64 },
    VotesChanged { status: String },
    VotesChangedBroadcasted { party_pubkey: String, votes: i64 },
    RecentBlockhashLoaded { blockhash: Hash },
}

// -- Party --

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Party {
    pub pubkey: String,
    pub name: String,
    pub votes: i64,
}
