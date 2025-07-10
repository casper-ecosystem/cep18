import { CLValue, Hash, Message, TransactionHash } from 'casper-js-sdk';

export enum CEP18_EVENTS {
  Mint = 'Mint',
  Burn = 'Burn',
  SetAllowance = 'SetAllowance',
  IncreaseAllowance = 'IncreaseAllowance',
  DecreaseAllowance = 'DecreaseAllowance',
  Transfer = 'Transfer',
  TransferFrom = 'TransferFrom',
  ChangeSecurity = 'ChangeSecurity',
  ChangeEventsMode = 'ChangeEventsMode'
}

type EventName = keyof typeof CEP18_EVENTS;

export type Event<E extends Record<string, CLValue>> = {
  name: EventName;
  contractHash: Hash;
  contractPackageHash: Hash;
  eventId: number;
  data: E;
};

export interface TransactionInfo {
  transactionHash: TransactionHash;
  timestamp: string;
  messages: Message[];
}

export type WithTransactionInfo<E> = E & { transactionInfo: TransactionInfo };

export type CEP18EventResult = WithTransactionInfo<CEP18Event>;

export type CEP18Event = Event<
  | Mint
  | Burn
  | SetAllowance
  | IncreaseAllowance
  | DecreaseAllowance
  | Transfer
  | TransferFrom
  | ChangeSecurity
  | ChangeEventsMode
>;

export type EventsMap = {
  Mint: WithTransactionInfo<Event<Mint>>;
  Burn: WithTransactionInfo<Event<Burn>>;
  SetAllowance: WithTransactionInfo<Event<SetAllowance>>;
  IncreaseAllowance: WithTransactionInfo<Event<IncreaseAllowance>>;
  DecreaseAllowance: WithTransactionInfo<Event<DecreaseAllowance>>;
  Transfer: WithTransactionInfo<Event<Transfer>>;
  TransferFrom: WithTransactionInfo<Event<TransferFrom>>;
  ChangeSecurity: WithTransactionInfo<Event<ChangeSecurity>>;
  ChangeEventsMode: WithTransactionInfo<Event<ChangeEventsMode>>;
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

export type ChangeSecurity = {
  admin: CLValue;
  sec_change_map: CLValue;
};

export type ChangeEventsMode = {
  events_mode: CLValue;
};
