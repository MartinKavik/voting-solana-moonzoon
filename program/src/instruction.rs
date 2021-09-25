#![allow(clippy::use_self)]

use crate::error::VotingError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

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

    // @TODO_QUESTION: Is ok to mirror the account order from the processor implementation?
    // (for instance mix writable and read-only accounts)
    /// Makes the voter eligible for voting by creating a VoterVotes account.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The voting owner account.
    /// 1. `[]` The voting state account.
    /// 2. `[writable]` The voter votes account.
    /// 3. `[]` The system program.
    AddVoter {
        voter_pubkey: Pubkey,
        voter_votes_bump_seed: u8,
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
    AddParty { name: String, party_bump_seed: u8 },

    /// Votes the provided party and creates a VoterVoted account.  
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The voter account.
    /// 1. `[]` The voting state account.
    /// 2. `[writable]` The voter voted account.
    /// 3. `[writable]` The voter votes account.
    /// 4. `[writable]` The party account.
    /// 5. `[]` The system program.
    Vote {
        /// The party will receive one negative or positive vote.
        positive: bool,
        voter_votes_bump_seed: u8,
    },
}

impl VotingInstruction {
    /// Unpacks a byte buffer into a [VotingInstruction](enum.VotingInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|error| {
            msg!(&error.to_string());
            VotingError::InvalidInstruction.into()
        })
    }
}

fn voting_state_pubkey(voting_owner_pubkey: &Pubkey) -> Pubkey {
    Pubkey::create_with_seed(voting_owner_pubkey, "voting_state", &crate::id())
        .expect("failed to create voting_state_pubkey")
}

pub fn init_voting(voting_owner_pubkey: &Pubkey) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new(voting_state_pubkey(voting_owner_pubkey), false),
    ];
    Instruction::new_with_borsh(crate::id(), &VotingInstruction::InitVoting, account_metas)
}

pub fn add_voter(voting_owner_pubkey: &Pubkey, voter_pubkey: &Pubkey) -> (Instruction, Pubkey) {
    let voting_state_pubkey = voting_state_pubkey(voting_owner_pubkey);

    let seeds = &[
        b"voter_votes".as_ref(),
        voter_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (voter_votes_pubkey, voter_votes_bump_seed) =
        Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new_readonly(voting_state_pubkey, false),
        AccountMeta::new(voter_votes_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddVoter {
            voter_pubkey: *voter_pubkey,
            voter_votes_bump_seed,
        },
        account_metas,
    );
    (ix, voter_votes_pubkey)
}

// @TODO_QUESTION: How to correctly represent a `Vec<Party>` on chain?
// @TODO_QUESTION: Can I create a Party account without the need
// to define the new index outside of transaction to prevent conflicts?
pub fn add_party(
    fee_payer: &Pubkey,
    party_name: &str,
    party_count: u32,
    voting_state_pubkey: &Pubkey,
) -> (Instruction, Pubkey) {
    let new_party_index_bytes = party_count.to_le_bytes();
    let seeds = &[
        b"party",
        new_party_index_bytes.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (party_pubkey, party_bump_seed) = Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*fee_payer, true),
        AccountMeta::new(party_pubkey, false),
        AccountMeta::new(*voting_state_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddParty {
            name: party_name.to_owned(),
            party_bump_seed,
        },
        account_metas,
    );
    (ix, party_pubkey)
}

pub fn vote(
    voter_pubkey: &Pubkey,
    voting_state_pubkey: &Pubkey,
    party_pubkey: &Pubkey,
    positive: bool,
) -> (Instruction, Pubkey, Pubkey) {
    let seeds = &[
        b"voter_votes".as_ref(),
        voter_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let voter_votes_pubkey = Pubkey::find_program_address(seeds, &crate::id()).0;

    let seeds = &[
        b"voter_voted".as_ref(),
        voter_pubkey.as_ref(),
        party_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (voter_voted_pubkey, voter_votes_bump_seed) =
        Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*voter_pubkey, true),
        AccountMeta::new_readonly(*voting_state_pubkey, false),
        AccountMeta::new(voter_voted_pubkey, false),
        AccountMeta::new(voter_votes_pubkey, false),
        AccountMeta::new(*party_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::Vote {
            positive,
            voter_votes_bump_seed,
        },
        account_metas,
    );
    (ix, voter_votes_pubkey, voter_voted_pubkey)
}
