use moonlight::*;

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    AddVoter { pub_key: String },
    AddParty { name: String },
    GetParties,
    GetDeadline,
    Vote { party_pub_key: String, positive: bool },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    VoterAdded { pub_key: String },
    PartyAdded { name: String },
    PartyAddedBroadcasted { party: Party },
    PartiesLoaded { parties: Vec<Party> },
    DeadlineLoaded { timestamp: i64 },
    VotesChanged { status: String },
    VotesChangedBroadcasted { party_pub_key: String, votes: i64 },
}

// -- Party --

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Party {
    pub pub_key: String,
    pub name: String,
    pub votes: i64,
}
