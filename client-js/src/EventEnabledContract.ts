import { Parser } from '@make-software/ces-js-parser';
import {
  // EventName,
  ExecutionResult,
  Hash,
  HttpHandler,
  RpcClient,
  SseClient
} from 'casper-js-sdk';

import { CEP18Event, CEP18EventWithDeployInfo, WithDeployInfo } from './events';

export default class EventEnabledContract {
  private _rpcClient: RpcClient;
  private _sseClient!: SseClient;
  private _parser!: Parser;
  private _chainName!: string;

  private contractHash?: Hash;
  private contractPackageHash?: Hash;

  private readonly events: Record<
    string,
    ((event: WithDeployInfo<CEP18Event>) => void)[]
  > = {};

  constructor(rpcUrl: string, sseUrl?: string, chainName?: string) {
    const rpcHandler: HttpHandler = new HttpHandler(rpcUrl);
    this._rpcClient = new RpcClient(rpcHandler);
    chainName && (this._chainName = chainName);
    sseUrl && (this._sseClient = new SseClient(sseUrl));
  }

  get chainName() {
    return this._chainName;
  }

  get rpcClient() {
    return this._rpcClient;
  }

  get sseClient() {
    return this._sseClient;
  }

  get parser() {
    return this._parser;
  }

  public setContractHash(contractHash: Hash, contractPackageHash?: Hash) {
    this.contractHash = contractHash;
    this.contractPackageHash = contractPackageHash;
  }

  // async setupEventStream(eventStream: EventStream) {
  //   this.eventStream = eventStream;

  //   if (!this.parser) {
  //     this.parser = await Parser.create(this.rpcClient, [
  //       this.contractClient.contractHash.slice(5)
  //     ]);
  //   }

  //   this.eventStream.start();

  //   this.eventStream.subscribe(EventName.DeployProcessed, deployProcessed => {
  //     const {
  //       execution_result,
  //       timestamp,
  //       deploy_hash: deployHash
  //     } = deployProcessed.body.DeployProcessed;

  //     if (!execution_result.Success || !this.parser) {
  //       return;
  //     }

  //     const results = this.parseExecutionResult(
  //       execution_result as ExecutionResult
  //     );

  //     results
  //       .map(
  //         r =>
  //           ({
  //             ...r,
  //             deployInfo: { deployHash, timestamp }
  //           }) as CEP18EventWithDeployInfo
  //       )
  //       .forEach(event => this.emit(event));
  //   });
  // }

  on(name: string, listener: (event: CEP18EventWithDeployInfo) => void) {
    this.addEventListener(name, listener);
  }

  addEventListener(
    name: string,
    listener: (event: CEP18EventWithDeployInfo) => void
  ) {
    if (!this.events[name]) this.events[name] = [];

    this.events[name].push(listener);
  }

  off(name: string, listener: (event: CEP18EventWithDeployInfo) => void) {
    this.removeEventListener(name, listener);
  }

  removeEventListener(
    name: string,
    listenerToRemove: (event: CEP18EventWithDeployInfo) => void
  ) {
    if (!this.events[name]) {
      throw new Error(
        `Can't remove a listener. Event "${name}" doesn't exist.`
      );
    }

    const filterListeners = (
      listener: (event: CEP18EventWithDeployInfo) => void
    ) => listener !== listenerToRemove;

    this.events[name] = this.events[name].filter(filterListeners);
  }

  emit(event: CEP18EventWithDeployInfo) {
    this.events[event.name]?.forEach(cb => cb(event));
  }

  parseExecutionResult(result: ExecutionResult): CEP18Event[] | undefined {
    const results = this.parser?.parseExecutionResult(result);

    return (
      results &&
      (results
        .filter(r => r.error === null)
        .map(r => ({
          ...r.event,
          contractHash: `hash-${r.event.contractHash?.toHex()}`,
          contractPackageHash: `hash-${r.event.contractPackageHash?.toHex()}`
        })) as unknown as CEP18Event[])
    );
  }
}
