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
