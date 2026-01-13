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
    fn default() -> Self {
        Self {
            kind: CurveKind::Linear,
            cliff_at: SCALE / 4,
            k_milli: 3_000,
            steepness_milli: 6_000,
        }
    }
}

pub fn sample_curve(params: CurveParams, t: u64) -> u64 {
    match params.kind {
        CurveKind::Linear => linear(t),
        CurveKind::Cliff => cliff(t, params.cliff_at),
        CurveKind::Exponential => exponential(t, params.k_milli),
        CurveKind::Logarithmic => logarithmic(t, params.k_milli),
        CurveKind::SCurve => s_curve(t, params.steepness_milli),
    }

