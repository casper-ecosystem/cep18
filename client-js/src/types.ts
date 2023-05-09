import { type BigNumberish } from '@ethersproject/bignumber';
import { type CLKeyParameters } from 'casper-js-sdk';

export enum EVENTS_MODE {
  NoEvents = 0,
  CES = 1
}

/**
 * Arguments required for install CEP18
 * @param name token name
 * @param symbol token symbol
 * @param decimals token decimals
 * @param totalSupply token total supply
 */
export interface InstallArgs {
  /** token name */
  name: string;
  symbol: string;
  decimals: BigNumberish;
  totalSupply: BigNumberish;
  eventsMode?: EVENTS_MODE;
}

export interface TransferableArgs {
  amount: BigNumberish;
}

export interface TransferArgs extends TransferableArgs {
  recipient: CLKeyParameters;
}

export interface TransferFromArgs extends TransferArgs {
  owner: CLKeyParameters;
}

export interface ApproveArgs extends TransferableArgs {
  spender: CLKeyParameters;
}
