use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent, clock::Clock},
    system_instruction,
    program::invoke_signed,
};
use crate::{state::{VotingState, Party}, error::VotingError};
use borsh::{BorshSerialize, BorshDeserialize};

pub fn process(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    party_name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let fee_payer_account = next_account_info(account_info_iter)?;

    if !fee_payer_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let party_account = next_account_info(account_info_iter)?;

    if !party_account.try_borrow_data()?.iter().all(|byte| *byte == 0) {
        Err(ProgramError::AccountAlreadyInitialized)?;
    }

    let voting_state_account = next_account_info(account_info_iter)?;

    let mut voting_state_account_data = voting_state_account.try_borrow_mut_data()?;
    let mut voting_state = VotingState::try_from_slice(&voting_state_account_data)?;

    if voting_state.deadline < Clock::get()?.unix_timestamp {
        Err(VotingError::VoteIsOver)?;
    }
    
    let new_party_index_bytes = voting_state.party_count.to_le_bytes();

    let seeds = &[b"party", new_party_index_bytes.as_ref(), voting_state_account.key.as_ref()];
    let (expected_party_pubkey, bump_seed) = Pubkey::find_program_address(seeds, program_id);
    if expected_party_pubkey != *party_account.key {
        Err(ProgramError::InvalidSeeds)?;
    }

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

    let signer_seeds = &[
        b"party".as_ref(), 
        &new_party_index_bytes.as_ref(), 
        &voting_state_account.key.as_ref(),
        &[bump_seed],
    ];

    invoke_signed(
        &create_party_account_ix, 
        accounts, 
        &[signer_seeds],
    )?;
    msg!("Party account created.");

    
    party_account
        .try_borrow_mut_data()?
        .copy_from_slice(&new_party_serialized);

    msg!("Party account initialized.");

    Ok(())
}
