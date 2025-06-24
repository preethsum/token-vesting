use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    pub candidate: Pubkey,
    pub vesting_account: Pubkey,
    pub total_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub withdrawn_amount: u64,
    pub bump: u8,
}
