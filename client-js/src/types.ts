import {
  ExecutionResult,
  PrivateKey,
  PublicKey,
  PutTransactionResult
} from 'casper-js-sdk';

export enum EVENTS_MODE {
  NoEvents = 0,
  CES = 1
}

export interface InstallArgs {
  /** token name */
  name: string;
  /** token symbol */
  symbol: string;
  /** token decimals */
  decimals: number;
  /** token total supply */
  totalSupply: string;
  /** events mode, disabled by default */
  eventsMode?: EVENTS_MODE;
  /** flag for mint and burn, false by default */
  enableMintAndBurn?: boolean;
}

export interface TransferableArgs {
  amount: string;
}

export interface HasOwner {
  owner: PublicKey;
}

export interface HasRecipient {
  recipient: PublicKey;
}

export interface HasSpender {
  spender: PublicKey;
}

export type TransferArgs = TransferableArgs & HasRecipient;
export type TransferFromArgs = TransferArgs & HasOwner;
export type ApproveArgs = TransferableArgs & HasSpender;
export type MintArgs = TransferableArgs & HasOwner;
export type BurnArgs = TransferableArgs & HasOwner;

export interface ChangeSecurityArgs {
  adminList?: PublicKey[];
  minterList?: PublicKey[];
  burnerList?: PublicKey[];
  mintAndBurnList?: PublicKey[];
  noneList?: PublicKey[];
}

export interface InstallParams {
  wasm: Uint8Array;
  paymentAmount: string;
  sender: PublicKey;
  signingKeys?: PrivateKey[];
  chainName?: string;
}

export interface InstallPayload {
  params: InstallParams;
  args: InstallArgs;
  waitForTransactionProcessed?: boolean;
}

export interface InstallResult {
  transactionResult: PutTransactionResult;
  executionResult?: ExecutionResult;
}
