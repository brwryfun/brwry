use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::BrwryError;
use crate::state::{Cask, CurveKindTag, Schedule};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateCaskParams {
    pub total_amount: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub cliff_ts: i64,
    pub curve: CurveKindTag,
    pub k_milli: u64,
    pub steepness_milli: u64,
    pub periods: u32,
}

#[derive(Accounts)]
#[instruction(params: CreateCaskParams)]
pub struct CreateCask<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: SystemAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Cask::SPACE,
        seeds = [Cask::SEED, authority.key().as_ref(), recipient.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub cask: Account<'info, Cask>,

    #[account(
        init,
        payer = authority,
        space = Schedule::SPACE,
        seeds = [Schedule::SEED, cask.key().as_ref()],
        bump,
    )]
    pub schedule: Account<'info, Schedule>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = cask,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateCask>, params: CreateCaskParams) -> Result<()> {
    require!(params.total_amount > 0, BrwryError::ZeroAmount);
    require!(params.start_ts < params.end_ts, BrwryError::InvalidSchedule);
    require!(
        params.cliff_ts >= params.start_ts && params.cliff_ts <= params.end_ts,
        BrwryError::CliffOutOfRange,
    );
    require!(params.k_milli <= 20_000, BrwryError::CurveOutOfBounds);
    require!(params.steepness_milli <= 20_000, BrwryError::CurveOutOfBounds);
    require!(params.periods > 0, BrwryError::InvalidSchedule);

    let cask = &mut ctx.accounts.cask;
    cask.authority = ctx.accounts.authority.key();
    cask.recipient = ctx.accounts.recipient.key();
    cask.mint = ctx.accounts.mint.key();
    cask.vault = ctx.accounts.vault.key();
    cask.total_amount = params.total_amount;
    cask.released_amount = 0;
    cask.start_ts = params.start_ts;
    cask.end_ts = params.end_ts;
