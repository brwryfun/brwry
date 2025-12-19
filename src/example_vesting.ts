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
