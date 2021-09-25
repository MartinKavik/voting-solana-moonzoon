use crate::{
    error::VotingError,
    state::{VoterVotes, VotingState},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

pub fn process(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    voter_pubkey: &Pubkey,
    voter_votes_bump_seed: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let voting_owner_account = next_account_info(account_info_iter)?;

    if !voting_owner_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let voting_state_account = next_account_info(account_info_iter)?;

    if voting_state_account.owner != program_id {
        Err(ProgramError::IllegalOwner)?;
    }

    let voting_state_data = voting_state_account.try_borrow_data()?;
    let voting_state = VotingState::try_from_slice(&voting_state_data)?;

    if voting_state.voting_owner != *voting_owner_account.key {
        Err(VotingError::IllegalVotingOwner)?;
    }

    let voter_votes_account = next_account_info(account_info_iter)?;

    if !voter_votes_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(ProgramError::AccountAlreadyInitialized)?
    }

    let voter_votes_size = VoterVotes::serialized_size();
    let create_voter_votes_account_ix = system_instruction::create_account(
        voting_owner_account.key,
        voter_votes_account.key,
        Rent::get()?.minimum_balance(voter_votes_size),
        voter_votes_size as u64,
        program_id,
    );

    let signers_seeds = &[
        b"voter_votes".as_ref(),
        voter_pubkey.as_ref(),
        voting_state_account.key.as_ref(),
        &[voter_votes_bump_seed],
    ];

    invoke_signed(&create_voter_votes_account_ix, accounts, &[signers_seeds])?;
    msg!("VoterVotes account created.");

    let new_voter_votes = VoterVotes {
        is_initialized: true,
        positive_votes: 2,
        negative_votes: 1,
        voter_pubkey: *voter_pubkey,
        voting_state_pubkey: *voting_state_account.key,
    };
    new_voter_votes.serialize(&mut *voter_votes_account.try_borrow_mut_data()?)?;

    msg!("VoterVotes account initialized.");

    Ok(())
}
