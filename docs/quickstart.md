# Quickstart

A five-minute walkthrough that goes from "nothing" to "there is a real
vesting stream in the cellar". Opinionated on purpose; every step has a
more flexible version somewhere else.

## 1. Decide what you are aging

Before anything else, answer three questions on paper:

- **Who is the recipient?** One wallet, a cohort, a treasury program?
- **How long?** Months, years, until an event?
- **Why a curve, not a line?** Every curve other than linear is a signal.
  If you cannot say in one sentence why you need a curve, pick linear.

If the answer to the third question is "because it looks cool", stop.
That is not a reason, and the recipient will resent it later.

## 2. Open the cellar

Visit `brwry.fun` and click **Enter the Cellar**. The app will ask for a
wallet; Phantom, Solflare, and Backpack are all supported. The cellar
scene is deliberately heavy on ambient light and low on motion, and it
respects `prefers-reduced-motion` at the OS level.

## 3. Draw the curve

Under **Visualizer**, pick one of the five presets or drop control points
freely. The curve compiler is live: a red outline means the curve is not
monotone, which the on-chain program will reject. Fix the dip, or pick
a preset and stop fighting it.

The JSON export button writes a minimal schema:

```json
{
  "preset": "s-curve",
  "months": 18,
  "cliffMonths": 1,
  "totalTokens": "1000000000000",
  "steepness": 6
}
```

Keep this file. It is the source of truth for the stream.

## 4. Mint the stream

Under **Vest**, paste the JSON and add the recipient's wallet. The form
validates three things before it lets you sign:

1. The mint has enough unlocked tokens in your wallet.
2. The recipient is a valid Solana address.
3. The curve compiles.

Sign with the connected wallet. The confirmation screen shows the
Streamflow address of the new stream. Bookmark it. That address is what
you hand to the recipient.

## 5. Turn on whispers

Optional, but the point of the service. At `brwry.fun/whispers`, paste
the stream address and link your Telegram account. The watcher sends
three messages per unlock milestone: 24 hours before, one hour before,
and at the moment of release. You can mute by responding `/quiet` to the
bot, or walk away by responding `/remove`.

---

## What actually happens on chain

The designer compiles your JSON into a monotone discrete function. The
service layer calls Streamflow's `create` with a `cliffAmount` that
matches the curve at the cliff boundary, an `amountPerPeriod` that
corresponds to the straight-line minimum, and a `name` that carries the
preset label. The on-chain program then uses Brwry's curve-aware
rebalancing to serve the correct unlocked amount on every claim.

The practical effect is that the recipient never sees "available to
claim" jitter. Every call to the program returns a number that matches
what the designer showed you on day one.
