# Rust crates

The Rust workspace under `programs/` holds two crates. Neither depends on the
web client, the Hono service, or the Python demos; they are pure curve math
and a small Anchor program that shares that math with the on-chain side.

## brwry-curves

`programs/brwry-curves` is a `no_std` library (with an opt-in `std` feature)
that implements the five unlock presets in fixed-point arithmetic. The scale
is `SCALE = 1_000_000`, so `t = 500_000` means progress through half the
schedule and a return value of `750_000` means 75 percent unlocked.

The public entry points:

```rust
use brwry_curves::{sample_curve, CurveKind, CurveParams, SCALE};

let params = CurveParams {
    kind: CurveKind::SCurve,
