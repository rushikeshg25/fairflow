use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Team {
    #[max_len(10)]
    pub team_name: String,
    #[max_len(5)]
    pub employees: Vec<Pubkey>,
    pub bump: u8,
}
