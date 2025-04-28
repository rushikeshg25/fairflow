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
    #[msg("Cannot submit feedback score for Employee outside of the team")]
    EmployeeNotInTeam,
    #[msg("Cannot submit feedback score for yourself")]
    CannotVoteForSelf,
    #[msg("Unauthorized to provide feedback")]
    Unauthorized,
    #[msg("Feeback rating must be between 1 and 5")]
    InvalidFeedbackRating,
}
