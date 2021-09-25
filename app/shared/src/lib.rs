use moonlight::*;
use solana_sdk::{account::Account, hash::Hash, pubkey::Pubkey, transaction::Transaction};

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    AddVoter {
        pubkey: Pubkey,
        transaction: Transaction,
    },
    AddParty {
        name: String,
        pubkey: Pubkey,
        transaction: Transaction,
    },
    GetParties,
    GetDeadline,
    Vote {
        party_pubkey: Pubkey,
        positive: bool,
        transaction: Transaction,
    },
    GetRecentBlockhash,
    GetAccount {
        account_pubkey: Pubkey,
    },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    VoterAdded {
        voter_pubkey_or_error: Result<Pubkey, String>,
    },
    PartyAdded {
        party_name_or_error: Result<String, String>,
    },
    PartyAddedBroadcasted {
        party: Party,
    },
    PartiesLoaded {
        parties: Vec<Party>,
    },
    DeadlineLoaded {
        timestamp: i64,
    },
    VotesChanged {
        status: String,
    },
    VotesChangedBroadcasted {
        party_pubkey: Pubkey,
        positive: bool,
    },
    RecentBlockhashLoaded {
        blockhash: Hash,
    },
    AccountLoaded {
        account: Result<Account, String>,
    },
}

// -- Party --

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Party {
    pub pubkey: Pubkey,
    pub name: String,
    pub votes: i64,
}
