use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct VoterVotes {
    pub is_initialized: bool,
    pub positive_votes: u8,
    pub negative_votes: u8,
}

impl VoterVotes {
    pub fn serialized_size() -> usize {
        VoterVotes::default()
            .try_to_vec()
            .expect("failed to serialize default VoterVotes")
            .len()
    }
}
