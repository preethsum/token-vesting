pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7VHr7hzjiGxp5sDw1HVrxFi2iaYNbgYb7GxSty64WnRX");

#[program]
pub mod token_vesting_2 {
    use super::*;

    pub fn initialize_vesting(ctx: Context<InitializeVesting>, company_name: String) -> Result<()> {
        initialize_vesting::create_vesting_account(ctx, company_name);
        Ok(())
    }

    pub fn add_candidate(ctx: Context<AddCandidate>) -> Result<()> {
        Ok(())
    }
}
