//  since `deployProcessed` is any type in L49 the eslint gives error for this line
/* eslint-disable eslint-comments/disable-enable-pair */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { Parser } from '@make-software/ces-js-parser';
import {
  CasperClient,
  Contracts,
  encodeBase16,
  EventName,
  EventStream,
  ExecutionResult
} from 'casper-js-sdk';

import { CEP18Event, CEP18EventWithDeployInfo, WithDeployInfo } from './events';

const { Contract } = Contracts;

export default class EventEnabledContract {
  public contractClient: Contracts.Contract;

  casperClient: CasperClient;

  eventStream?: EventStream;

  parser?: Parser;

  private readonly events: Record<
    string,
    ((event: WithDeployInfo<CEP18Event>) => void)[]
  > = {};

  constructor(public nodeAddress: string, public networkName: string) {
    this.casperClient = new CasperClient(nodeAddress);
    this.contractClient = new Contract(this.casperClient);
  }

  async setupEventStream(eventStream: EventStream) {
    this.eventStream = eventStream;

    if (!this.parser) {
      this.parser = await Parser.create(this.casperClient.nodeClient, [
        this.contractClient.contractHash.slice(5)
      ]);
    }

    await this.eventStream.start();

    this.eventStream.subscribe(EventName.DeployProcessed, deployProcessed => {
      const {
        execution_result,
        timestamp,
        deploy_hash: deployHash
      } = deployProcessed.body.DeployProcessed;

      if (!execution_result.Success || !this.parser) {
        return;
      }

      const results = this.parseExecutionResult(
        execution_result as ExecutionResult
      );

      results
        .map(
          r =>
            ({
              ...r,
              deployInfo: { deployHash, timestamp }
            } as CEP18EventWithDeployInfo)
        )
        .forEach(event => this.emit(event));
    });
  }

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
        `Can't remove a listener. Event "${name}" doesn't exits.`
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

  parseExecutionResult(result: ExecutionResult): CEP18Event[] {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const results = this.parser.parseExecutionResult(result);

    return results
      .filter(r => r.error === null)
      .map(r => ({
        ...r.event,
        contractHash: `hash-${encodeBase16(r.event.contractHash)}`,
        contractPackageHash: `hash-${encodeBase16(r.event.contractPackageHash)}`
      })) as CEP18Event[];
  }
}
