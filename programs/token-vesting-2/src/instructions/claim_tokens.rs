use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::error::ErrorCode;
use crate::{Candidate, Vesting};

pub use error::*;
#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub candidate: Signer<'info>,

    pub authority: SystemAccount<'info>,

    #[account(
        has_one = authority,
        has_one = token_mint,
        has_one = treasury_account,
        seeds = [b"vesting", authority.key().as_ref(), vesting.company_name.as_bytes()],
        bump = vesting.bump,
    )]
    pub vesting: Account<'info, Vesting>,

    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = vesting,
        associated_token::token_program = token_program,
    )]
    pub treasury_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        has_one = vesting,
        seeds = [b"candidate", candidate.key().as_ref(), vesting.key().as_ref()],
        bump = candidate_account.bump,
    )]
    pub candidate_account: Account<'info, Candidate>,

    #[account(
        init_if_needed,
        payer = candidate,
        associated_token::mint = token_mint,
        associated_token::authority = candidate,
        associated_token::token_program = token_program,
    )]
    pub candidate_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens_to_candidate(ctx: Context<ClaimTokens>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let candidate_account = &mut ctx.accounts.candidate_account;
    let mut amount: u64 = 0;

    if current_time < candidate_account.start_time && current_time < candidate_account.cliff_time {
        return Err(ErrorCode::VestingNotStarted.into());
    }

    if current_time > candidate_account.end_time {
        amount = candidate_account.total_amount - candidate_account.withdrawn_amount
    }

    if current_time > candidate_account.cliff_time && current_time < candidate_account.end_time {
        let time_passed_from_cliff = candidate_account.cliff_time - current_time;
        let total_time = candidate_account.end_time - candidate_account.cliff_time;

        amount = (time_passed_from_cliff / total_time) as u64 * candidate_account.total_amount;
    }

    candidate_account.withdrawn_amount += amount;

    let seeds = &[
        b"vesting",
        ctx.accounts.authority.to_account_info().key.as_ref(),
        ctx.accounts.vesting.company_name.as_bytes(),
        &[ctx.accounts.vesting.bump],
    ];
    let signer = &[&seeds[..]];
    let cpi_accounts = anchor_spl::token_interface::TransferChecked {
        from: ctx.accounts.treasury_account.to_account_info(),
        to: ctx.accounts.candidate_token_account.to_account_info(),
        authority: ctx.accounts.vesting.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
    };
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    anchor_spl::token_interface::transfer_checked(
        cpi_context,
        amount - candidate_account.withdrawn_amount,
        ctx.accounts.token_mint.decimals,
    )
}
