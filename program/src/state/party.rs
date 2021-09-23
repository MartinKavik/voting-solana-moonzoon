use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct Party {
    pub is_initialized: bool,
    pub positive_votes: u32,
    pub negative_votes: u32,
}

impl Party {
    pub fn serialized_size() -> usize {
        Party::default()
            .try_to_vec()
            .expect("failed to serialize default Party")
            .len()
    }
}
