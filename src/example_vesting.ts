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
