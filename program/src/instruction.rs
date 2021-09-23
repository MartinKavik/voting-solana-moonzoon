use solana_program::{
    program_error::ProgramError, 
    msg,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::VotingError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VotingInstruction {

    /// Starts the voting by creating and populating a VotingState account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The voting owner account.
    /// 1. `[writable]` The voting state account.
    InitVoting,

    /// Makes the voter eligible for voting by creating a VoterVotes account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The voting owner account.
    /// 1. `[writable]` The voter votes account.
    /// 2. `[]` The system program.
    AddVoter {
        voter_pubkey: Pubkey,
    },

    /// Creates a new Party account with the requested name 
    /// and increments the parties counter in the VotingState account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The fee payer account.
    /// 1. `[writable]` The party account.
    /// 2. `[writable]` The voting state account.
    /// 3. `[]` The system program.
    AddParty {
        name: String,
    },

    /// Votes the provided party and creates a VoterVoted account.  
    /// 
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The eligible voter account.
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
            VotingError::InvalidInstruction.into()
        })
    }
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
    voter_votes_pubkey: &Pubkey,
    voter_pubkey: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new(*voter_votes_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddVoter { voter_pubkey: *voter_pubkey },
        account_metas,
    )
}

pub fn add_party(
    fee_payer: &Pubkey,
    party_pubkey: &Pubkey,
    party_name: &str,
    voting_state_pubkey: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*fee_payer, true),
        AccountMeta::new(*party_pubkey, false),
        AccountMeta::new(*voting_state_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddParty { name: party_name.to_owned() },
        account_metas,
    )
}
