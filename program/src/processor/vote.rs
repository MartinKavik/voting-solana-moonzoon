use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
};
use crate::{instruction::VotingInstruction, error::VotingError};

pub fn process(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    positive: bool,
) -> ProgramResult {
    Ok(())
}
