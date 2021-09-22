use solana_program::{
    program_error::ProgramError, 
    msg,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::VotingError::InvalidInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VotingInstruction {

    /// Starts the voting by creating and populating a VotingState account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The voting owner account.
    /// 1. `[writable]` The voting state account.
    InitVoting,

    /// Makes the voter eligible for voting by creating a VoterVotes account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The voting owner account.
    /// 1. `[writable]` The voter votes account.
    AddVoter {
        voter_pubkey: Pubkey,
    },

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

pub fn init_voting(
    voting_owner_pubkey: &Pubkey,
    voting_state_pubkey: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new(*voting_state_pubkey, false),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::InitVoting,
        account_metas,
    )
}

pub fn add_voter(
    voting_owner_pubkey: &Pubkey,
    voter_pubkey: &Pubkey,
    voter_votes_pubkey: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new(*voter_votes_pubkey, false),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddVoter { voter_pubkey: *voter_pubkey },
        account_metas,
    )
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
