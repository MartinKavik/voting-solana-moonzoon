use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Illegal voting owner")]
    IllegalVotingOwner,

    #[error("Illegal voter")]
    IllegalVoter,

    #[error("Illegal voting state")]
    IllegalVotingState,

    #[error("The vote is over")]
    VoteIsOver,

    #[error("The voter is not eligible for voting")]
    NotEligibleForVoting,

    #[error("The voter already voted for the selected party")]
    AlreadyVoted,

    #[error("No positive votes left")]
    NoPositiveVotes,

    #[error("No negative votes left")]
    NoNegativeVotes,

    #[error("Positive votes have to be spent before negative voting")]
    PositiveVotesNotSpent,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
