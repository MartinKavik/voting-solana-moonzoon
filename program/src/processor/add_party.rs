use crate::{
    error::VotingError,
    state::{Party, VotingState},
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
    party_name: String,
    party_bump_seed: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let fee_payer_account = next_account_info(account_info_iter)?;

    if !fee_payer_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let party_account = next_account_info(account_info_iter)?;

    if !party_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(ProgramError::AccountAlreadyInitialized)?;
    }

    let voting_state_account = next_account_info(account_info_iter)?;

    let mut voting_state_account_data = voting_state_account.try_borrow_mut_data()?;
    let mut voting_state = VotingState::try_from_slice(&voting_state_account_data)?;

    if voting_state.deadline < Clock::get()?.unix_timestamp {
        Err(VotingError::VoteIsOver)?;
    }

    let new_party_index_bytes = voting_state.party_count.to_le_bytes();

    voting_state.party_count += 1;
    voting_state.serialize(&mut *voting_state_account_data)?;

    let new_party = Party {
        is_initialized: true,
        positive_votes: 0,
        negative_votes: 0,
        name: party_name,
        voting_state_pubkey: *voting_state_account.key,
    };
    let new_party_serialized = new_party.try_to_vec()?;

    let party_size = new_party_serialized.len();
    let create_party_account_ix = system_instruction::create_account(
        fee_payer_account.key,
        party_account.key,
        Rent::get()?.minimum_balance(party_size),
        party_size as u64,
        program_id,
    );

    let signers_seeds = &[
        b"party".as_ref(),
        new_party_index_bytes.as_ref(),
        voting_state_account.key.as_ref(),
        &[party_bump_seed],
    ];

    invoke_signed(&create_party_account_ix, accounts, &[signers_seeds])?;
    msg!("Party account created.");

    party_account
        .try_borrow_mut_data()?
        .copy_from_slice(&new_party_serialized);

    msg!("Party account initialized.");

    Ok(())
}
