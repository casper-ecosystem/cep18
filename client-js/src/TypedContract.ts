import { Parser } from '@make-software/ces-js-parser';
import { ExecutionResult, Hash, RpcClient, SseClient } from 'casper-js-sdk';
import EventEnabledContract from './EventEnabledContract';
import { CEP18Event, EventsMap } from './events';

interface ITypedContract {
  rpcClient: RpcClient;
  sseClient?: SseClient;
  parser?: Parser;
  chainName?: string;

  //setupEventStream(eventStream: EventStream): Promise<void>;
  setContractHash(contractHash: Hash, contractPackageHash?: Hash): void;
  parseExecutionResult(result: ExecutionResult): CEP18Event[] | undefined;

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

interface TypedContractConstructor {
  new (rpcUrl: string, ssUrl?: string, chainName?: string): ITypedContract;
  prototype: ITypedContract;
}

const TypedContract = EventEnabledContract as TypedContractConstructor;

export default TypedContract;
