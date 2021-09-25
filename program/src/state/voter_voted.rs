use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct VoterVoted {
    pub is_initialized: bool,
    pub voter_pubkey: Pubkey,
    pub voting_state_pubkey: Pubkey,
}

impl VoterVoted {
    pub fn serialized_size() -> usize {
        Self::default()
            .try_to_vec()
            .expect("failed to serialize default VoterVoted")
            .len()
    }
}
