import { Contracts, EventStream } from 'casper-js-sdk';

import EventEnabledContract from './EventEnabledContract';
import {
  Burn,
  DecreaseAllowance,
  Event,
  IncreaseAllowance,
  Mint,
  SetAllowance,
  Transfer,
  TransferFrom
} from './events';

export type EventsMap = {
  Mint: Event<Mint>;
  Burn: Event<Burn>;
  SetAllowance: Event<SetAllowance>;
  IncreaseAllowance: Event<IncreaseAllowance>;
  DecreaseAllowance: Event<DecreaseAllowance>;
  Transfer: Event<Transfer>;
  TransferFrom: Event<TransferFrom>;
};

interface ITypedContract {
  contractClient: Contracts.Contract;

  setupEventStream(eventStream: EventStream): Promise<void>;

  on<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  addEventListener<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  off<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;

  removeEventListener<K extends keyof EventsMap>(
    type: K,
    listener: (ev: EventsMap[K]) => void
  ): void;
}

export const TypedContract = EventEnabledContract as {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  new (public nodeAddress: string, public networkName: string): ITypedContract;

  prototype: ITypedContract;
};
