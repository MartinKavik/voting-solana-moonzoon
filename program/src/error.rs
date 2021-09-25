use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingError {
    #[error("invalid Instruction")]
    InvalidInstruction,

    #[error("illegal voting owner")]
    IllegalVotingOwner,

    #[error("illegal voter")]
    IllegalVoter,

    #[error("illegal voting state")]
    IllegalVotingState,

    #[error("the vote is over")]
    VoteIsOver,

    #[error("the voter is not eligible for voting")]
    NotEligibleForVoting,

    #[error("the voter already voted for the selected party")]
    AlreadyVoted,

    #[error("no positive votes left")]
    NoPositiveVotes,

    #[error("no negative votes left")]
    NoNegativeVotes,

    #[error("positive votes have to be spent before the negative ones")]
    PositiveVotesNotSpent,
}

impl From<VotingError> for ProgramError {
    fn from(e: VotingError) -> Self {
        Self::Custom(e as u32)
    }
}
