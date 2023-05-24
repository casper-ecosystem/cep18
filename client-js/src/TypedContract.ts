import { Contracts, EventStream } from 'casper-js-sdk';

import EventEnabledContract from './EventEnabledContract';
import { EventsMap } from './events';

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

const TypedContract = EventEnabledContract as {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  new (public nodeAddress: string, public networkName: string): ITypedContract;

  prototype: ITypedContract;
};

export default TypedContract;
