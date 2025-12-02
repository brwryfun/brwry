use alloc::vec::Vec;

use crate::{cliff, exponential, linear, logarithmic, s_curve, SCALE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveKind {
    Linear,
    Cliff,
    Exponential,
    Logarithmic,
    SCurve,
}

#[derive(Clone, Copy, Debug)]
pub struct CurveParams {
    pub kind: CurveKind,
    pub cliff_at: u64,
    pub k_milli: u64,
    pub steepness_milli: u64,
}

impl Default for CurveParams {
