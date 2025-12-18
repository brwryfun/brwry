use anchor_lang::prelude::*;

#[error_code]
pub enum BrwryError {
    #[msg("start timestamp must precede end timestamp")]
    InvalidSchedule,
    #[msg("cliff timestamp must be within the schedule window")]
    CliffOutOfRange,
