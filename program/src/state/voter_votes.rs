use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct VoterVotes {
    pub is_initialized: bool,
    pub positive_votes: u8,
    pub negative_votes: u8,
    pub voter_pubkey: Pubkey,
    pub voting_state_pubkey: Pubkey,
}

impl VoterVotes {
    pub fn serialized_size() -> usize {
        Self::default()
            .try_to_vec()
            .expect("failed to serialize default VoterVotes")
            .len()
    }
}
