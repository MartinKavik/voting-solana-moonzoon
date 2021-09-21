use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct VotingState {
    pub is_initialized: bool,
    pub deadline: i64,
    pub party_count: u32,
    pub voting_owner: Pubkey,
}

impl VotingState {
    pub fn serialized_size() -> usize {
        // @TODO compute once?
        VotingState::default()
            .try_to_vec()
            .expect("failed to serialize default VotingState")
            .len()
    }
}
