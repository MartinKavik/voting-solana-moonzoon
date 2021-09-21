use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VotingState {
    pub is_initialized: bool,
    pub deadline: i64,
    pub party_count: u32,
    pub voting_owner: Pubkey,
}
