use anchor_lang::prelude::*;

#[error_code]
pub enum BrwryError {
    #[msg("start timestamp must precede end timestamp")]
    InvalidSchedule,
    #[msg("cliff timestamp must be within the schedule window")]
    CliffOutOfRange,
    #[msg("total amount must be non-zero")]
    ZeroAmount,
    #[msg("nothing is claimable at this moment")]
    NothingToRelease,
    #[msg("only the recipient may release from this cask")]
    UnauthorizedRecipient,
    #[msg("curve parameters are outside supported bounds")]
    CurveOutOfBounds,
    #[msg("arithmetic overflow while computing release amount")]
    MathOverflow,
}
