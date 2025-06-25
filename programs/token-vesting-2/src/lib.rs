pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("7VHr7hzjiGxp5sDw1HVrxFi2iaYNbgYb7GxSty64WnRX");

#[program]
pub mod token_vesting_2 {
    use super::*;

    pub fn initialize_vesting(ctx: Context<InitializeVesting>, company_name: String) -> Result<()> {
        initialize_vesting::create_vesting_account(ctx, company_name)
    }

    pub fn add_candidate(
        ctx: Context<AddCandidate>,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        amount: u64,
    ) -> Result<()> {
        add_candidate::transfer_tokens_to_treasury(&ctx, amount)?;
        add_candidate::create_candidate_account(ctx, start_time, end_time, cliff_time, amount)
    }
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        // claim_tokens::claim(ctx)
        Ok(())
    }
}
