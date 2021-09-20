use solana_program::{
    program_error::ProgramError, msg,
};
use crate::error::VotingError::InvalidInstruction;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VotingInstruction {

    /// Starts the voting by creating and populating a VotingState account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The voting owner account.
    InitVoting,

    /// Makes the voter eligible for voting by creating a VoterVotes account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The voting owner account.
    /// 1. `[]` The voter account.
    AddVoter,

    /// Creates a new Party account with the requested name 
    /// and increments the parties counter in the VotingState account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The fee payer account.
    AddParty {
        /// The party name max length is 128 bytes.
        name: String,
    },

    /// Votes the provided party and creates a VoterVoted account.  
    /// 
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The eligible voter account.
    /// 1. `[]` The voted party account.
    Vote {
        /// The party will receive one negative or positive vote.
        positive: bool,
    },
}

impl VotingInstruction {
    /// Unpacks a byte buffer into a [VotingInstruction](enum.VotingInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(&input).map_err(|error| {
            msg!(&error.to_string());
            InvalidInstruction.into()
        })
    }
}
