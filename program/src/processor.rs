use crate::instruction::VotingInstruction;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

mod add_party;
mod add_voter;
mod init_voting;
mod vote;

#[cfg_attr(feature = "no-entrypoint", allow(dead_code))]
pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match VotingInstruction::unpack(instruction_data)? {
        VotingInstruction::InitVoting => {
            msg!("Instruction: InitVoting");
            init_voting::process(accounts, program_id)
        }
        VotingInstruction::AddVoter {
            voter_pubkey,
            voter_votes_bump_seed,
        } => {
            msg!("Instruction: AddVoter");
            add_voter::process(accounts, program_id, &voter_pubkey, voter_votes_bump_seed)
        }
        VotingInstruction::AddParty {
            name,
            party_bump_seed,
        } => {
            msg!("Instruction: AddParty");
            add_party::process(accounts, program_id, name, party_bump_seed)
        }
        VotingInstruction::Vote {
            positive,
            voter_votes_bump_seed,
        } => {
            msg!("Instruction: Vote");
            vote::process(accounts, program_id, positive, voter_votes_bump_seed)
        }
    }
}
