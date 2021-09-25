use crate::{
    error::VotingError,
    state::{Party, VoterVoted, VoterVotes, VotingState},
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
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

pub fn process(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    positive: bool,
    voter_votes_bump_seed: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let voter_account = next_account_info(account_info_iter)?;

    if !voter_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let voting_state_account = next_account_info(account_info_iter)?;

    if voting_state_account.owner != program_id {
        Err(ProgramError::IllegalOwner)?;
    }

    let voting_state_account_data = voting_state_account.try_borrow_data()?;
    let voting_state = VotingState::try_from_slice(&voting_state_account_data)?;

    if voting_state.deadline < Clock::get()?.unix_timestamp {
        Err(VotingError::VoteIsOver)?;
    }

    let voter_voted_account = next_account_info(account_info_iter)?;

    if !voter_voted_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(VotingError::AlreadyVoted)?;
    }

    let voter_votes_account = next_account_info(account_info_iter)?;

    if voter_votes_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(VotingError::NotEligibleForVoting)?;
    }

    let mut voter_votes_account_data = voter_votes_account.try_borrow_mut_data()?;
    let mut voter_votes = VoterVotes::try_from_slice(&voter_votes_account_data)?;

    if positive {
        if voter_votes.positive_votes == 0 {
            Err(VotingError::NoPositiveVotes)?;
        }
    } else {
        if voter_votes.negative_votes == 0 {
            Err(VotingError::NoNegativeVotes)?;
        }
        if voter_votes.positive_votes != 0 {
            Err(VotingError::PositiveVotesNotSpent)?;
        }
    }

    let party_account = next_account_info(account_info_iter)?;

    if party_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(ProgramError::UninitializedAccount)?;
    }

    let mut party_account_data = party_account.try_borrow_mut_data()?;
    let mut party = Party::try_from_slice(&party_account_data)?;

    if voter_votes.voter_pubkey != *voter_account.key {
        Err(VotingError::IllegalVoter)?;
    }
    if voter_votes.voting_state_pubkey != *voting_state_account.key {
        Err(VotingError::IllegalVotingState)?;
    }
    if party.voting_state_pubkey != *voting_state_account.key {
        Err(VotingError::IllegalVotingState)?;
    }

    if positive {
        voter_votes.positive_votes -= 1;
        party.positive_votes += 1;
    } else {
        voter_votes.negative_votes -= 1;
        party.negative_votes += 1;
    }

    party.serialize(&mut *party_account_data)?;
    voter_votes.serialize(&mut *voter_votes_account_data)?;

    let voter_voted_size = VoterVoted::serialized_size();
    let create_voter_voted_account_ix = system_instruction::create_account(
        voter_account.key,
        voter_voted_account.key,
        Rent::get()?.minimum_balance(voter_voted_size),
        voter_voted_size as u64,
        program_id,
    );

    let signers_seeds = &[
        b"voter_voted".as_ref(),
        voter_account.key.as_ref(),
        party_account.key.as_ref(),
        voting_state_account.key.as_ref(),
        &[voter_votes_bump_seed],
    ];

    invoke_signed(&create_voter_voted_account_ix, accounts, &[signers_seeds])?;
    msg!("VoterVoted account created.");

    let voter_voted = VoterVoted {
        is_initialized: true,
        voter_pubkey: *voter_account.key,
        voting_state_pubkey: *voting_state_account.key,
    };
    voter_voted.serialize(&mut *voter_voted_account.try_borrow_mut_data()?)?;

    msg!("Voted.");

    Ok(())
}
