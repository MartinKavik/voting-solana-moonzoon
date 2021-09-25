use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct Party {
    pub is_initialized: bool,
    pub positive_votes: u32,
    pub negative_votes: u32,
    // @TODO_QUESTION How to store strings to avoid problems with fixed data length?
    // Something like `arrayvec::ArrayString` with a max length 128 bytes?
    // @TODO_QUESTION How to store long texts with very different lengths (imagine a blog article)?
    // Chunk into accounts?
    pub name: String,
    pub voting_state_pubkey: Pubkey,
}

impl Party {
    pub fn serialized_size() -> usize {
        Self::default()
            .try_to_vec()
            .expect("failed to serialize default Party")
            .len()
    }
}
