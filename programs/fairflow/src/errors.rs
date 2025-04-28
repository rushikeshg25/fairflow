use anchor_lang::error_code;

#[error_code]
pub enum CompanyError {
    #[msg("Increment and Decrement Percentage can only be in between 0 and 100")]
    InvalidPercentage,
    #[msg("Company name can only be between 1 and 10 characters")]
    InvalidCompanyName,
    #[msg("Team name can only be between 1 and 10 characters")]
    InvalidTeamName,
    #[msg("Maximum number of teams reached")]
    MaxTeamsReached,
    #[msg("Employee name can only be between 1 and 10 characters")]
    InvalidEmployeeName,
    #[msg("Cannot vote for Employee outside of the team")]
    EmployeeNotInTeam,
    #[msg("Cannot vote for yourself")]
    CannotVoteForSelf,
    #[msg("Already voted for this Employee")]
    AlreadyVoted,
}
