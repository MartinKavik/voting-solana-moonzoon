use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct Party {
    pub is_initialized: bool,
    pub positive_votes: u32,
    pub negative_votes: u32,
    // @TODO How to store strings to avoid problems with fixed data length? 
    // Something like `arrayvec::ArrayString`?
    pub name: String,
}

impl Party {
    pub fn serialized_size() -> usize {
        Party::default()
            .try_to_vec()
            .expect("failed to serialize default Party")
            .len()
    }
}
