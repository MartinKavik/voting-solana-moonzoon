use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    sysvar::{Sysvar, rent::Rent},
    system_instruction,
    program::invoke_signed,
};
use crate::{state::VoterVotes, instruction as voting_instruction};

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

    if !voter_votes_account.try_borrow_data()?.iter().all(|byte| *byte == 0) {
        Err(ProgramError::AccountAlreadyInitialized)?
    }

    let seeds = &[b"voter_votes".as_ref(), &voter_pubkey.as_ref(), &voting_owner_account.key.as_ref()];
    let (expected_voter_votes_pubkey, bump_seed) = Pubkey::find_program_address(seeds, program_id);
    if expected_voter_votes_pubkey != *voter_votes_account.key {
        Err(ProgramError::InvalidSeeds)?
    }
    
    let voter_votes_size = VoterVotes::serialized_size();
    let create_voter_votes_account_ix = system_instruction::create_account(
        voting_owner_account.key, 
        voter_votes_account.key, 
        Rent::get()?.minimum_balance(voter_votes_size), 
        voter_votes_size as u64, 
        program_id,
    );

    let signer_seeds = &[
        b"voter_votes".as_ref(), 
        &voter_pubkey.as_ref(), 
        &voting_owner_account.key.as_ref(),
        &[bump_seed],
    ];

    invoke_signed(
        &create_voter_votes_account_ix, 
        accounts, 
        &[signer_seeds],
    )?;
    msg!("VoterVotes account created.");

    let init_voter_votes_ix = voting_instruction::init_voter_votes(
        voting_owner_account.key, 
        voter_votes_account.key,
    );
    invoke_signed(
        &init_voter_votes_ix, 
        accounts, 
        &[signer_seeds],
    )?;

    msg!("VoterVotes account finished.");

    Ok(())
}
