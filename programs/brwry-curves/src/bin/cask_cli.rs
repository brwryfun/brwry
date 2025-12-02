use std::env;
use std::process::ExitCode;

use brwry_curves::{sample_schedule, CurveKind, CurveParams, SCALE};

fn parse_curve(name: &str) -> Option<CurveKind> {
    match name {
        "linear" => Some(CurveKind::Linear),
        "cliff" => Some(CurveKind::Cliff),
        "exponential" | "exp" => Some(CurveKind::Exponential),
        "logarithmic" | "log" => Some(CurveKind::Logarithmic),
        "s-curve" | "scurve" | "s" => Some(CurveKind::SCurve),
        _ => None,
    }
}

fn usage() {
    eprintln!(
        "usage: cask_cli --curve <kind> --total <tokens> --start <unix> --end <unix> [--periods N]\n\
         kinds: linear, cliff, exponential, logarithmic, s-curve"
    );
}

