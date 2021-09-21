use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Insufficient Lamports")]
    InsufficientLamports,

    #[error("Not rent exempt")]
    NotRentExempt,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
