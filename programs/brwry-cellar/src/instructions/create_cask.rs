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
