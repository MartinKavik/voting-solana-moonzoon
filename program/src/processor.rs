use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::instruction::VotingInstruction;

mod init_voting;
mod add_voter;
mod add_party;
mod vote;

#[cfg_attr(feature = "no-entrypoint", allow(dead_code))]
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    match VotingInstruction::unpack(instruction_data)? {
        VotingInstruction::InitVoting => {
            msg!("Instruction: InitVoting");
            init_voting::process(accounts, program_id)
        }
        VotingInstruction::AddVoter { voter_pubkey } => {
            msg!("Instruction: AddVoter");
            add_voter::process(accounts, program_id, &voter_pubkey)
        }
        VotingInstruction::AddParty { name } => {
            msg!("Instruction: AddParty");
            add_party::process(accounts, program_id, name)
        }
        VotingInstruction::Vote { positive } => {
            msg!("Instruction: Vote");
            vote::process(accounts, program_id, positive)
        }
    }
}    
