import {
  ExecutionResult,
  PrivateKey,
  PublicKey,
  PutTransactionResult
} from 'casper-js-sdk';

export enum EVENTS_MODE {
  NoEvents = 0,
  CES = 1,
  Native = 2,
  NativeBytes = 3
}
export type InstallArgs = {
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
};

export type UpgradeArgs = {
  /** token name */
  name: string;
  eventsMode?: EVENTS_MODE;
};

type TransferableArgs = {
  amount: string;
};

interface HasOwner {
  owner: PublicKey;
}

interface HasRecipient {
  recipient: PublicKey;
}

interface HasSpender {
  spender: PublicKey;
}

export type TransferArgs = TransferableArgs & HasRecipient;
export type TransferFromArgs = TransferArgs & HasOwner;
export type ApproveArgs = TransferableArgs & HasSpender;
export type IncreaseAllowanceArgs = ApproveArgs;
export type DecreaseAllowanceArgs = ApproveArgs;
export type MintArgs = TransferableArgs & HasOwner;
export type BurnArgs = TransferableArgs & HasOwner;

export type ChangeSecurityArgs = {
  adminList?: PublicKey[];
  minterList?: PublicKey[];
  burnerList?: PublicKey[];
  mintAndBurnList?: PublicKey[];
  noneList?: PublicKey[];
};

export type TransactionParams = {
  sender: PublicKey;
  paymentAmount: string;
  wasm?: Uint8Array;
  signingKeys?: PrivateKey[];
  chainName?: string;
};

export type TransactionResult = {
  transactionInfo: PutTransactionResult;
  executionResult?: ExecutionResult;
};

interface BaseParams {
  params: TransactionParams;
  waitForTransactionProcessed?: boolean;
}

export interface InstallParams extends BaseParams {
  args: InstallArgs;
}

export interface UpgradeParams extends BaseParams {
  args: UpgradeArgs;
}

export interface TransferParams extends BaseParams {
  args: TransferArgs;
}

export interface TransferFromParams extends BaseParams {
  args: TransferFromArgs;
}

export interface ApproveParams extends BaseParams {
  args: ApproveArgs;
}

export interface IncreaseAllowanceParams extends BaseParams {
  args: IncreaseAllowanceArgs;
}

export interface DecreaseAllowanceParams extends BaseParams {
  args: DecreaseAllowanceArgs;
}

export interface MintParams extends BaseParams {
  args: MintArgs;
}

export interface BurnParams extends BaseParams {
  args: BurnArgs;
}

export interface ChangeSecurityParams extends BaseParams {
  args: ChangeSecurityArgs;
}
