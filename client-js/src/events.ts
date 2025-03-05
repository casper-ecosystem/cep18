import { CLValue } from 'casper-js-sdk';

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

export type Mint = { recipient: CLValue; amount: CLValue };

export type Burn = { owner: CLValue; amount: CLValue };

export type SetAllowance = {
  owner: CLValue;
  spender: CLValue;
  allowance: CLValue;
};

export type IncreaseAllowance = {
  owner: CLValue;
  spender: CLValue;
  allowance: CLValue;
  inc_by: CLValue;
};

export type DecreaseAllowance = {
  owner: CLValue;
  spender: CLValue;
  allowance: CLValue;
  decr_by: CLValue;
};

export type Transfer = { sender: CLValue; recipient: CLValue; amount: CLValue };

export type TransferFrom = {
  spender: CLValue;
  owner: CLValue;
  recipient: CLValue;
  amount: CLValue;
};
