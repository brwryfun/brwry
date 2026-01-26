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

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut curve = "s-curve".to_string();
    let mut total: u128 = 1_000_000;
    let mut start: i64 = 0;
    let mut end: i64 = 365 * 24 * 60 * 60;
    let mut periods: u32 = 12;

    let mut iter = args.iter().skip(1);
    while let Some(flag) = iter.next() {
        match flag.as_str() {
            "--curve" => curve = iter.next().cloned().unwrap_or_default(),
            "--total" => total = iter.next().and_then(|v| v.parse().ok()).unwrap_or(total),
            "--start" => start = iter.next().and_then(|v| v.parse().ok()).unwrap_or(start),
            "--end" => end = iter.next().and_then(|v| v.parse().ok()).unwrap_or(end),
            "--periods" => periods = iter.next().and_then(|v| v.parse().ok()).unwrap_or(periods),
            "-h" | "--help" => {
                usage();
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("unknown flag: {other}");
                usage();
                return ExitCode::from(2);
            }
        }
    }

    let kind = match parse_curve(&curve) {
        Some(k) => k,
        None => {
            eprintln!("unknown curve: {curve}");
            usage();
            return ExitCode::from(2);
        }
    };
