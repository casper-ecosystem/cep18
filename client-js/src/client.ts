import { Parser } from '@make-software/ces-js-parser';
import {
  Args as RuntimeArgs,
  ContractHash,
  ContractPackageHash,
  ContractCallBuilder,
  EventName,
  ExecutionResult,
  HttpHandler,
  RawEvent,
  RpcClient,
  SseClient,
  type InfoGetTransactionResult,
  type PublicKey,
  type PrivateKey,
  type QueryGlobalStateResult,
  type TransactionProcessedEvent
} from 'casper-js-sdk';
import { ContractError } from './error';
import {
  type CEP18Event,
  type CEP18EventResult,
  type WithTransactionInfo
} from './events';
import { type TransactionResult } from './types';

const defaulTransactionTimeout = 120_000; // 2 min
const contractErrorMessagePrefix = 'User error: ';

export default class Client {
  public chainName!: string;

  private _rpcClient!: RpcClient;
  private _sseClient!: SseClient;
  private _parser!: Parser;
  private _contractHash!: ContractHash;
  private _contractPackageHash!: ContractPackageHash;
  private readonly _events: Record<
    string,
    ((event: WithTransactionInfo<CEP18Event>) => void)[]
  > = {};

  constructor(rpcUrl: string, sseUrl?: string, chainName?: string) {
    this.rpcUrl = rpcUrl;
    sseUrl && (this.sseUrl = sseUrl);
    chainName && (this.chainName = chainName);
  }

  public set rpcUrl(url: string) {
    const rpcHandler = new HttpHandler(url);
    this._rpcClient = new RpcClient(rpcHandler);
  }

  public set sseUrl(url: string) {
    this._sseClient = new SseClient(url);
  }

  public get contractHash(): ContractHash {
    return this._contractHash;
  }

  public get contractPackageHash(): ContractPackageHash {
    return this._contractPackageHash;
  }

  public addEventListener(
    name: string,
    listener: (event: CEP18EventResult) => void
  ) {
    if (!this._events[name]) this._events[name] = [];
    this._events[name].push(listener);
  }

  public removeEventListener(
    name: string,
    listenerToRemove: (event: CEP18EventResult) => void
  ) {
    if (!this._events[name]) {
      throw new Error(
        `Can't remove a listener. Event "${name}" doesn't exist.`
      );
    }
    const filterListeners = (listener: (event: CEP18EventResult) => void) =>
      listener !== listenerToRemove;
    this._events[name] = this._events[name].filter(filterListeners);
  }

  // Alias for addEventListener
  public on(name: string, listener: (event: CEP18EventResult) => void) {
    this.addEventListener(name, listener);
  }

  // Alias for removeEventListener
  public off(
    name: string,
    listenerToRemove: (event: CEP18EventResult) => void
  ) {
    this.removeEventListener(name, listenerToRemove);
  }

  public removeListenersForEvent(name: string): void {
    if (!this._events[name]) {
      throw new Error(`No listeners found for event "${name}".`);
    }
    this._events[name] = []; // Clear all listeners for event
  }

  public removeAllListeners(): void {
    for (const name in this._events) {
      if (Object.prototype.hasOwnProperty.call(this._events, name)) {
        this._events[name] = []; // Clear all listeners for each event
      }
    }
  }

  /**
   * Get and parse transaction result by given hash.
   * It the transaction wasn't successful, throws `ContractError` if there was operational error, otherwise `Error` with original error message.
   * @param transactionHash transaction hash
   * @returns `InfoGetTransactionResult`
   */
  public async getTransactionResult(
    transactionHash: string
  ): Promise<InfoGetTransactionResult> {
    const result =
      await this.rpcClient.getTransactionByTransactionHash(transactionHash);

    const executionError = result.executionInfo?.executionResult.errorMessage;
    if (executionError) this.handleExecutionError(executionError);

    return result;
  }

  public waitForTransactionProcessed = (
    transactionHash: string,
    timeout?: number,
    sseUrl?: string
  ): Promise<TransactionProcessedEvent> => {
    sseUrl && (this.sseUrl = sseUrl);
    if (!this.sseClient) {
      throw Error('SSE Client is not set.');
    }

    return new Promise((resolve, reject) => {
      const timeoutId = setTimeout(() => {
        this.sseClient.stop();
        reject(
          new Error(`Transaction ${transactionHash} processing timed out.`)
        );
      }, timeout ?? defaulTransactionTimeout);

      const subscription = this.subscribeToTransactionProcessedEvent(
        async processEvent => {
          if (
            processEvent.transactionProcessedPayload.transactionHash
              .toString()
              .toLowerCase() === transactionHash.toLowerCase()
          ) {
            clearTimeout(timeoutId);
            this.sseClient.stop();
            this.sseClient.unsubscribe(EventName.TransactionProcessedEventType);
            resolve(processEvent);
          }
        },
        error => {
          clearTimeout(timeoutId);
          reject(error);
        }
      );
      subscription && this.sseClient.start();
    });
  };

  protected get rpcClient() {
    return this._rpcClient;
  }

  protected get sseClient() {
    return this._sseClient;
  }

  protected stopEventStream(): Client {
    this.sseClient?.stop();
    return this;
  }

  protected setContractHash(
    contractHash: ContractHash,
    contractPackageHash?: ContractPackageHash
  ): Client {
    contractHash && (this._contractHash = contractHash);
    contractPackageHash && (this._contractPackageHash = contractPackageHash);
    return this;
  }

  protected startEventStream(sseUrl?: string): Client {
    sseUrl && (this.sseUrl = sseUrl);
    if (!this.sseClient) {
      throw Error('SSE Client is not set.');
    }

    const subscription = this.subscribeToTransactionProcessedEvent(
      async processEvent => {
        const { executionResult, transactionHash, messages, timestamp } =
          processEvent.transactionProcessedPayload;

        if (executionResult.errorMessage) {
          this.handleExecutionError(executionResult.errorMessage);
        }

        (await this.parseExecutionResult(executionResult))
          ?.map(
            result =>
              ({
                ...result,
                transactionInfo: {
                  transactionHash: transactionHash.toString(),
                  timestamp,
                  messages
                }
              }) as unknown as CEP18EventResult
          )
          .forEach(event => this.emit(event));
      }
    );
    subscription && this.sseClient.start();
    return this;
  }

  /**
   * Calls a contract entry point with given arguments.
   * @param entryPoint The name of the contract entry point.
   * @param runtimeArgs The runtime arguments for the contract call.
   * @param paymentAmount The payment amount required for execution.
   * @param sender The transaction sender.
   * @param signingKeys (Optional) Array of signing keys to sign the transaction.
   * @param chainName (Optional) Network name where the transaction will be deployed.
   * @returns TransactionResult promise.
   */
  protected async callEntrypoint(
    entryPoint: string,
    runtimeArgs: RuntimeArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string,

    waitForTransactionProcessed?: boolean
  ): Promise<TransactionResult> {
    let contractCallBuilder = new ContractCallBuilder()
      .entryPoint(entryPoint)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '');

    if (this.contractPackageHash) {
      contractCallBuilder = contractCallBuilder.byPackageHash(
        this.contractPackageHash.hash.toHex()
      );
    } else if (this.contractHash) {
      contractCallBuilder = contractCallBuilder.byHash(
        this.contractHash.hash.toHex()
      );
    }

    const transaction = contractCallBuilder.build();

    if (signingKeys) {
      signingKeys.forEach(key => transaction.sign(key));
    }

    try {
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (waitForTransactionProcessed && transactionInfo.transactionHash) {
        const deployEvent = await this.waitForTransactionProcessed(
          transactionInfo.transactionHash.toString()
        );
        return {
          transactionInfo,
          executionResult:
            deployEvent.transactionProcessedPayload.executionResult
        };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(`Error during entry point call.\n${error}`);
    }
  }

  /**
   * Queries contract data from the global state.
   *
   * @param path Optional array of strings representing the subkeys or path within the contract storage.
   *             If no path is provided, it queries the top-level contract data.
   * @returns A `Promise` resolving to the contract's stored value (`clValue`) or throws an error if the stored value is invalid.
   * @throws Will throw an error if the contract data does not contain a valid stored value.
   */
  protected async queryContractData(path: string[] = []): Promise<string> {
    if (!this.contractHash) {
      throw new Error(
        `Error during queryContractData. Contract hash is not set`
      );
    }
    const contractData: QueryGlobalStateResult =
      await this.rpcClient.queryGlobalStateByStateHash(
        null,
        // TODO Fix that with toPrefixedString ?
        `hash-${this.contractHash.hash.toHex()}`,
        path
      );

    const result = contractData.storedValue.clValue?.toString();
    if (result) {
      return result;
    }
    throw Error('Invalid stored value');
  }

  private subscribeToTransactionProcessedEvent(
    onProcess: (event: TransactionProcessedEvent) => Promise<void>,
    onError?: (error: unknown) => void
  ): boolean {
    const eventName = EventName.TransactionProcessedEventType;
    const subscription = this.sseClient
      .subscribe(eventName, async (rawEvent: RawEvent) => {
        try {
          const processEvent = rawEvent.parseAsTransactionProcessedEvent();
          await onProcess(processEvent);
        } catch (error) {
          console.error('Error processing event:', error);
          this.sseClient.stop();
          subscription && this.sseClient.unsubscribe(eventName);
          onError?.(error);
        }
      })
      .unwrap();
    return subscription;
  }

  private handleExecutionError(errorMessage: string) {
    if (errorMessage.startsWith(contractErrorMessagePrefix)) {
      const errorCode = parseInt(
        errorMessage.substring(contractErrorMessagePrefix.length),
        10
      );
      throw new ContractError(errorCode);
    } else {
      throw new Error(errorMessage);
    }
  }

  private async parseExecutionResult(
    result: ExecutionResult
  ): Promise<CEP18Event[] | undefined> {
    this._parser = await Parser.create(this.rpcClient, [
      this.contractHash.hash.toHex()
    ]);
    const results = this._parser?.parseExecutionResult(result);

    return (
      results &&
      (results
        .filter(result => {
          if (result.error !== null) {
            console.error('Error parsing execution result:', result.error);
          }
          return result.error === null;
        })
        .map(result => {
          return {
            ...result.event,
            contractHash: result.event.contractHash?.toHex(),
            contractPackageHash: result.event.contractPackageHash?.toHex()
          };
        }) as unknown as CEP18Event[])
    );
  }

  private emit(event: CEP18EventResult) {
    this._events[event.name]?.forEach(cb => cb(event));
  }
}
