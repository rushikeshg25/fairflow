use anchor_lang::error_code;

#[error_code]
pub enum CompanyError {
    #[msg("Increment and Decrement Percentage can only be in between 0 and 100")]
    InvalidPercentage,
}
