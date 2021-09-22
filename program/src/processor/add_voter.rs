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
    program_id: &Pubkey,
    voter_pubkey: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let voting_owner_account = next_account_info(account_info_iter)?;

    if !voting_owner_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let voter_votes_account = next_account_info(account_info_iter)?;

    if !Rent::get()?.is_exempt(voter_votes_account.lamports(), voter_votes_account.data_len()) {
        Err(ProgramError::AccountNotRentExempt)?
    }

    if !voter_votes_account.try_borrow_data()?.iter().all(|byte| *byte == 0) {
        Err(ProgramError::AccountAlreadyInitialized)?
    }

    let seeds = &[b"voter_votes", voter_pubkey.as_ref(), voting_owner_account.key.as_ref()];
    let expected_voter_votes_pubkey = Pubkey::find_program_address(seeds, program_id).0;
    if expected_voter_votes_pubkey != *voter_votes_account.key {
        Err(ProgramError::InvalidSeeds)?
    }

    let new_voter_votes = VoterVotes {
        is_initialized: true,
        positive_votes: 2,
        negative_votes: 1,
    };

    voter_votes_account
        .try_borrow_mut_data()?
        .copy_from_slice(&new_voter_votes.try_to_vec()?);

    msg!("Voter added.");

    Ok(())
}
