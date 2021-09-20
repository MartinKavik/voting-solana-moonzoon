use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// Not rent exempt
    #[error("Not rent exempt")]
    NotRentExempt,

    /// Expected amount mismatch
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,

    /// Amount overflow
    #[error("Amount overflow")]
    AmountOverflow,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
