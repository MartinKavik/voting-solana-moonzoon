use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    sysvar::{Sysvar, clock::Clock},
};
use borsh::BorshSerialize;
use crate::state::VotingState;

pub fn process(
    accounts: &[AccountInfo],
    _program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let voting_owner_account = next_account_info(account_info_iter)?;

    if !voting_owner_account.is_signer {
        Err(ProgramError::MissingRequiredSignature)?;
    }

    let voting_state_account = next_account_info(account_info_iter)?;

    // @TODO_QUESTION A better way? Use something like https://crates.io/crates/binary-layout?
    if !voting_state_account.try_borrow_data()?.iter().all(|byte| *byte == 0) {
        Err(ProgramError::AccountAlreadyInitialized)?
    }

    let voting_state = VotingState {
        is_initialized: true,
        deadline: Clock::get()?.unix_timestamp + 7 * 86_400,
        party_count: 0,
        voting_owner: *voting_owner_account.key,
    };

    voting_state.serialize(&mut *voting_state_account.try_borrow_mut_data()?)?;

    msg!("VotingState initialized.");

    Ok(())
}
