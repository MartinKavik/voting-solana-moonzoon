use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar, clock::Clock},
    program::{invoke, invoke_signed},
    system_instruction,
    system_program,
};
use std::mem;
use crate::{instruction::VotingInstruction, error::VotingError, state::VotingState};

pub fn process(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    msg!("hello from init_voting!");

    // let account_info_iter = &mut accounts.iter();
    // let voting_owner = next_account_info(account_info_iter)?;

    // if !voting_owner.is_signer {
    //     return Err(ProgramError::MissingRequiredSignature);
    // }

    // let voting_state = VotingState {
    //     is_initialized: true,
    //     deadline: Clock::get()?.unix_timestamp + 7 * 86_400,
    //     party_count: 0,
    // };

    // let voting_state_size = mem::size_of_val(&voting_state);
    // let minimum_lamports = Rent::get()?.minimum_balance(voting_state_size);

    // voting_owner
    //     .try_borrow_mut_lamports()?
    //     .checked_sub(minimum_lamports)
    //     .ok_or(VotingError::InsufficientLamports)?;

    // let voting_state_pubkey = Pubkey::find_program_address(&[b"voting_state"], program_id).0;
    // let create_voting_state_account_ix = system_instruction::create_account(
    //     voting_owner.key, 
    //     &voting_state_pubkey, 
    //     minimum_lamports, 
    //     voting_state_size as u64, 
    //     program_id,
    // );

    // msg!("Calling the token program to transfer token account ownership...");
    // invoke(
    //     &create_voting_state_account_ix,
    //     &[
    //         temp_token_account.clone(),
    //         initializer.clone(),
    //         system_program.id(),
    //     ],
    // )?;


    // let account_info_iter = &mut accounts.iter();
    // let initializer = next_account_info(account_info_iter)?;

    // if !initializer.is_signer {
    //     return Err(ProgramError::MissingRequiredSignature);
    // }

    // let temp_token_account = next_account_info(account_info_iter)?;

    // let token_to_receive_account = next_account_info(account_info_iter)?;
    // if *token_to_receive_account.owner != spl_token::id() {
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // let escrow_account = next_account_info(account_info_iter)?;
    // // NOTE: Use Rent::default() or Rent::get() instead!
    // let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    // if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
    //     return Err(VotingError::NotRentExempt.into());
    // }

    // let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
    // if escrow_info.is_initialized() {
    //     return Err(ProgramError::AccountAlreadyInitialized);
    // }

    // escrow_info.is_initialized = true;
    // escrow_info.initializer_pubkey = *initializer.key;
    // escrow_info.temp_token_account_pubkey = *temp_token_account.key;
    // escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
    // escrow_info.expected_amount = amount;

    // Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;
    // let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    // let token_program = next_account_info(account_info_iter)?;
    // let owner_change_ix = spl_token::instruction::set_authority(
    //     token_program.key,
    //     temp_token_account.key,
    //     Some(&pda),
    //     spl_token::instruction::AuthorityType::AccountOwner,
    //     initializer.key,
    //     &[&initializer.key],
    // )?;

    // msg!("Calling the token program to transfer token account ownership...");
    // invoke(
    //     &owner_change_ix,
    //     &[
    //         temp_token_account.clone(),
    //         initializer.clone(),
    //         token_program.clone(),
    //     ],
    // )?;

    Ok(())
}
