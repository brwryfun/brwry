# Anchor program

`programs/brwry-cellar` is the Anchor 0.29 program that holds tokens for a
vesting stream and releases them along one of the five presets. It is
intentionally small: one state account, one schedule account, two
instructions, a handful of error codes. The curve math comes from the
sibling `brwry-curves` crate so the designer, the service layer, and the
on-chain program all compute the same number for the same `t`.

The program id `Brwry11111111111111111111111111111111111111` is a
deliberately invalid placeholder. Deploy the crate yourself and replace it
with whatever the toolchain prints before you use the binary anywhere real.

## Account layout

### `Cask`

The long-lived account. Created once at `create_cask`, mutated on every
release, never closed.

| Field | Type | Purpose |
| --- | --- | --- |
| `authority` | `Pubkey` | Payer and creator of the cask |
| `recipient` | `Pubkey` | The only wallet allowed to release |
| `mint` | `Pubkey` | SPL or Token-2022 mint being vested |
| `vault` | `Pubkey` | Token account owned by the cask PDA |
| `total_amount` | `u64` | Deposit in raw token units |
| `released_amount` | `u64` | Running total already claimed |
| `start_ts` | `i64` | Unix second when `t = 0` |
| `end_ts` | `i64` | Unix second when `t = 1` |
| `cliff_ts` | `i64` | First moment a release is allowed |
| `curve` | `CurveKindTag` | One of the five preset tags |
| `k_milli` | `u64` | Exp/log curvature knob (x1000) |
| `steepness_milli` | `u64` | S-curve steepness knob (x1000) |
| `bump` | `u8` | PDA bump |

Seeds: `[b"cask", authority, recipient, mint]`. One cask per
authority-recipient-mint tuple; that keeps the PDA deterministic so the
client can always find it again without extra indices.

### `Schedule`

A small side account that tracks release bookkeeping. Separated from `Cask`
so the hot loop of claims does not fight for space with the immutable
configuration.

| Field | Type | Purpose |
| --- | --- | --- |
| `cask` | `Pubkey` | Owning cask PDA |
| `periods` | `u32` | Total number of release buckets |
| `current_period` | `u32` | Bucket the next claim will occupy |
