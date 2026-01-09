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
