use moonlight::*;

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    AddVoter { pub_key: String },
    AddParty { name: String },
}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    VoterAdded { pub_key: String },
    PartyAdded { name: String },
}
