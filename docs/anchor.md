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
| `last_claim_ts` | `i64` | Most recent release timestamp |
| `bump` | `u8` | PDA bump |

Seeds: `[b"schedule", cask]`.

## Instructions

### `create_cask`

Creates both PDAs, allocates the vault, writes parameters. All validation
happens up front: `start < end`, `cliff` within the window, `k_milli` and
`steepness_milli` below 20x. The vault is initialised with `cask` as its
authority so no one but the program can move tokens out.

Important: the instruction does not move any tokens. The caller is expected
to deposit the `total_amount` into the vault immediately afterwards, in a
separate instruction. Splitting the two lets the client keep the deposit
as a routine `transfer_checked` rather than a custom CPI, which matters for
Token-2022 mints that carry transfer fees or permanent delegates.

### `release_barrel`

Compares `now` to `start_ts`, `cliff_ts`, and `end_ts`, computes the scaled
progress `t`, samples the curve, and derives the amount claimable since the
last release. If non-zero it CPIs into `transfer_checked` on the token
program bound to the mint (SPL or Token-2022), signs with the cask PDA,
and bumps bookkeeping.

The curve math on-chain is a direct call into the `brwry-curves` crate, so
the fraction computed here matches the fraction shown in the designer for
the same `t` to the last unit of `SCALE`.

## Token-2022 notes

The instruction uses `InterfaceAccount` and `TokenInterface` so the same
binary supports both `spl-token` and `spl-token-2022`. When the mint has a
transfer fee extension the recipient receives the post-fee amount, not the
pre-fee amount; nothing special happens on our side. For interest-bearing
mints the `total_amount` baseline is fixed at deposit time and any accrued
interest stays in the vault as a surplus after the final claim.

Do not mix extensions that interact badly with fixed curves (confidential
transfers, permanent delegate) without testing against devnet first. None
of those are blocked by the program, but neither are any of them
meaningful to the vesting guarantee.

## CPI surface

Only one CPI: `anchor_spl::token_interface::transfer_checked`. Nothing
calls out to Streamflow, Jupiter, or anyone else from the program itself.
The service layer is where external SDKs live.

## Errors

Defined in `errors.rs`:

- `InvalidSchedule`
- `CliffOutOfRange`
- `ZeroAmount`
- `NothingToRelease`
- `UnauthorizedRecipient`
- `CurveOutOfBounds`
- `MathOverflow`
