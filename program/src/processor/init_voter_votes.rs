use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
};
use borsh::BorshSerialize;
use crate::state::VoterVotes;

pub fn process(
    accounts: &[AccountInfo],
    _program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let voting_owner_account = next_account_info(account_info_iter)?;
    let voter_votes_account = next_account_info(account_info_iter)?;

    if !voting_owner_account.is_signer && !voter_votes_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    if !Rent::get()?.is_exempt(voter_votes_account.lamports(), voter_votes_account.data_len()) {
        Err(ProgramError::AccountNotRentExempt)?
    }

    if !voter_votes_account.try_borrow_data()?.iter().all(|byte| *byte == 0) {
        Err(ProgramError::AccountAlreadyInitialized)?
    }

    let new_voter_votes = VoterVotes {
        is_initialized: true,
        positive_votes: 2,
        negative_votes: 1,
    };

    voter_votes_account
        .try_borrow_mut_data()?
        .copy_from_slice(&new_voter_votes.try_to_vec()?);

    msg!("VoterVotes initialized.");

    Ok(())
}
