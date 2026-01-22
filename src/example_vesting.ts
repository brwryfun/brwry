/**
 * Sketch of the Streamflow call pattern Brwry uses internally.
 *
 * This file is not a full integration. It builds the configuration
 * object, logs the parameters Streamflow expects, and stops before
 * signing or broadcasting. Real deployment happens from the web
 * client with a connected wallet; this sketch is here so a curious
 * reader can see the shape of the call without cloning the whole
 * service layer.
 */

import { PublicKey } from "@solana/web3.js";

export type UnlockPreset =
  | "linear"
  | "cliff"
  | "exponential"
  | "logarithmic"
  | "s-curve";

export interface VestingPlan {
  recipient: string;
  mint: string;
  totalAmount: bigint;
  startUnixSeconds: number;
  durationSeconds: number;
  preset: UnlockPreset;
  cliffSeconds?: number;
  canTopUp?: boolean;
  canTransfer?: boolean;
  telegramChatId?: string;
}

interface StreamflowCreateArgs {
  recipient: PublicKey;
  mint: PublicKey;
  depositedAmount: bigint;
  start: number;
  period: number;
  cliff: number;
  cliffAmount: bigint;
  amountPerPeriod: bigint;
  name: string;
  transferableBySender: boolean;
  transferableByRecipient: boolean;
  canTopup: boolean;
  automaticWithdrawal: boolean;
}

const SECONDS_PER_PERIOD = 24 * 60 * 60; // daily release ticks

function fractionForPreset(preset: UnlockPreset, t: number): number {
  const clamp = (x: number) => Math.max(0, Math.min(1, x));
  switch (preset) {
    case "linear":
      return clamp(t);
    case "cliff": {
      const cliff = 0.25;
      return t < cliff ? 0 : clamp((t - cliff) / (1 - cliff));
    }
    case "exponential": {
      const k = 3;
      return clamp((Math.exp(k * clamp(t)) - 1) / (Math.exp(k) - 1));
    }
    case "logarithmic": {
      const k = 4;
      return clamp(Math.log1p(k * clamp(t)) / Math.log1p(k));
    }
    case "s-curve": {
      const steepness = 6;
      const raw = (x: number) =>
        1 / (1 + Math.exp(-steepness * (x - 0.5)));
      const lo = raw(0);
      const hi = raw(1);
      return clamp((raw(clamp(t)) - lo) / (hi - lo));
    }
  }
}

export function toStreamflowArgs(plan: VestingPlan): StreamflowCreateArgs {
  const periods = Math.max(
    1,
    Math.floor(plan.durationSeconds / SECONDS_PER_PERIOD),
  );
  const amountPerPeriod =
    plan.totalAmount / BigInt(periods);
  const cliffSeconds = plan.cliffSeconds ?? 0;

  // Brwry computes the non-linear shape as a discrete "cliffAmount" plus
  // a ramp of amountPerPeriod buckets. The on-chain program rebalances
  // these buckets to match the preset exactly when the recipient claims.
  const t = cliffSeconds / plan.durationSeconds;
  const cliffFraction = fractionForPreset(plan.preset, t);
