import { CLKey, CLU256, CLValue } from 'casper-js-sdk';

export type Event<E extends Record<string, CLValue>> = {
  name: string;
  contractHash: `hash-${string}`;
  contractPackageHash: `hash-${string}`;
  data: E;
};

export interface DeployInfo {
  deployHash: string;
  timestamp: string;
}

export type WithDeployInfo<E> = E & { deployInfo: DeployInfo };

export type CEP18EventWithDeployInfo = WithDeployInfo<CEP18Event>;

export type CEP18Event = Event<
  | Mint
  | Burn
  | SetAllowance
  | IncreaseAllowance
  | DecreaseAllowance
  | Transfer
  | TransferFrom
>;

export type EventsMap = {
  Mint: Event<Mint>;
  Burn: Event<Burn>;
  SetAllowance: Event<SetAllowance>;
  IncreaseAllowance: Event<IncreaseAllowance>;
  DecreaseAllowance: Event<DecreaseAllowance>;
  Transfer: Event<Transfer>;
  TransferFrom: Event<TransferFrom>;
};

export type Mint = {
  recipient: CLKey;
  amount: CLU256;
};

export type Burn = {
  owner: CLKey;
  amount: CLU256;
};

export type SetAllowance = {
  owner: CLKey;
  spender: CLKey;
  allowance: CLU256;
};

export type IncreaseAllowance = {
  owner: CLKey;
  spender: CLKey;
  allowance: CLU256;
  inc_by: CLU256;
};

export type DecreaseAllowance = {
  owner: CLKey;
  spender: CLKey;
  allowance: CLU256;
  decr_by: CLU256;
};

export type Transfer = {
  sender: CLKey;
  recipient: CLKey;
  amount: CLU256;
};

export type TransferFrom = {
  spender: CLKey;
  owner: CLKey;
  recipient: CLKey;
  amount: CLU256;
};
