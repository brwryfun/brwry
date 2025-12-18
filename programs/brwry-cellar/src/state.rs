use anchor_lang::prelude::*;

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveKindTag {
    Linear = 0,
    Cliff = 1,
    Exponential = 2,
    Logarithmic = 3,
    SCurve = 4,
}

#[account]
pub struct Cask {
    pub authority: Pubkey,
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub cliff_ts: i64,
    pub curve: CurveKindTag,
    pub k_milli: u64,
    pub steepness_milli: u64,
