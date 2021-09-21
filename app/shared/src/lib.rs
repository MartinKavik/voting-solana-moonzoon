use moonlight::*;

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    AddVoter { pubkey: String },
    AddParty { name: String },
    GetParties,
    GetDeadline,
    Vote { party_pubkey: String, positive: bool },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    VoterAdded { pubkey: String },
    PartyAdded { name: String },
    PartyAddedBroadcasted { party: Party },
    PartiesLoaded { parties: Vec<Party> },
    DeadlineLoaded { timestamp: i64 },
    VotesChanged { status: String },
    VotesChangedBroadcasted { party_pubkey: String, votes: i64 },
}

// -- Party --

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Party {
    pub pubkey: String,
    pub name: String,
    pub votes: i64,
}
