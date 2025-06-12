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

/**
 * Client class for interacting with the Casper blockchain.
 * Provides functionalities to connect to an RPC and SSE server,
 * manage smart contract interactions, and listen for blockchain events.
 *
 * IMPORTANT:
 * This class is duplicated in every CEP JS client.
 * Any modifications made to this class must be reflected across all instances of CEP clients to maintain consistency.
 * Ensure to update all relevant CEP JS clients whenever changes are made to this class.
 */
export default class Client {
  public chainName!: string;
  private _rpcUrl!: string;
  private _sseUrl!: string;
  private _rpcClient!: RpcClient;
  private _sseClient!: SseClient;
  private _parser!: Parser;
  private _contractHash!: ContractHash;
  private _contractPackageHash!: ContractPackageHash;

  /** Internal storage for registered event listeners. */
  private readonly _events: Record<
    string,
    ((event: WithTransactionInfo<CEP18Event>) => void)[]
  > = {};

  /**
   * Constructs a new Client instance.
   * @param rpcUrl - The URL of the Casper RPC server.
   * @param sseUrl - (Optional) The URL of the SSE event stream server.
   * @param chainName - (Optional) The name of the blockchain network.
   */
  constructor(rpcUrl: string, sseUrl?: string, chainName?: string) {
    this.rpcUrl = rpcUrl;
    sseUrl && (this.sseUrl = sseUrl);
    chainName && (this.chainName = chainName);
  }

  /**
   * Sets the RPC URL and initializes the RPC client.
   * @param url - The new RPC URL.
   */
  public set rpcUrl(url: string) {
    if (!url) {
      return;
    }
    const rpcHandler = new HttpHandler(url);
    this._rpcClient = new RpcClient(rpcHandler);
    this._rpcUrl = url;
  }

  /**
   * Sets the SSE URL and initializes the SSE client.
   * @param url - The new SSE URL.
   */
  public set sseUrl(url: string) {
    if (!url) {
      return;
    }
    this._sseClient = new SseClient(url);
    this._sseUrl = url;
  }

  /**
   * Gets the RPC URL.
   */
  public get rpcUrl(): string {
    return this._rpcUrl;
  }

  /**
   * Gets the SSE URL.
   */
  public get sseUrl(): string | undefined {
    return this._sseUrl;
  }

  /**
   * Gets the smart contract hash.
   * @returns The contract hash associated with this client.
   */
  public get contractHash(): ContractHash {
    return this._contractHash;
  }

  /**
   * Gets the smart contract package hash.
   * @returns The contract package hash associated with this client.
   */
  public get contractPackageHash(): ContractPackageHash {
    return this._contractPackageHash;
  }

  /**
   * Registers an event listener for a specific event.
   * @param name - The event name.
   * @param listener - The callback function to execute when the event occurs.
   */
  public addEventListener(
    name: string,
    listener: (event: CEP18EventResult) => void
  ) {
    if (!this._events[name]) this._events[name] = [];
    this._events[name].push(listener);
  }

  /**
   * Removes a specific listener for an event.
   * @param name - The event name.
   * @param listenerToRemove - The callback function to remove.
   * @throws An error if the event does not exist.
   */
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

  /**
   * Alias for `addEventListener`.
   * @param name - The event name.
   * @param listener - The callback function to execute when the event occurs.
   */
  public on(name: string, listener: (event: CEP18EventResult) => void) {
    this.addEventListener(name, listener);
  }

  /**
   * Alias for `removeEventListener`.
   * @param name - The event name.
   * @param listenerToRemove - The callback function to remove.
   */
  public off(
    name: string,
    listenerToRemove: (event: CEP18EventResult) => void
  ) {
    this.removeEventListener(name, listenerToRemove);
  }

  /**
   * Removes all listeners for a given event.
   * @param name - The event name.
   * @throws An error if no listeners exist for the event.
   */
  public removeListenersForEvent(name: string): void {
    if (!this._events[name]) {
      throw new Error(`No listeners found for event "${name}".`);
    }
    this._events[name] = []; // Clear all listeners for event
  }

  /**
   * Removes all registered event listeners for all events.
   */
  public removeAllListeners(): void {
    for (const name in this._events) {
      if (Object.prototype.hasOwnProperty.call(this._events, name)) {
        this._events[name] = []; // Clear all listeners for each event
      }
    }
  }

  /**
   * Retrieves and parses the transaction result for a given transaction hash.
   *
   * If the transaction was not successful, this method throws:
   * - `ContractError` if the failure is due to an operational error.
   * - A generic `Error` containing the original error message for other failures.
   *
   * @param transactionHash - The hash of the transaction to retrieve.
   * @returns A `Promise` resolving to the `InfoGetTransactionResult` containing transaction details.
   * @throws `ContractError` if the transaction failed due to an operational error.
   * @throws `Error` with the original error message if another failure occurs.
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

  /**
   * Waits for a transaction to be processed and resolves once the transaction is found.
   *
   * If the transaction is not processed within the given timeout, the method rejects with a timeout error.
   * The SSE client is automatically stopped when the transaction is found or if an error occurs.
   *
   * @param transactionHash - The hash of the transaction to wait for.
   * @param timeout - Optional timeout in milliseconds. Defaults to `defaulTransactionTimeout` if not provided.
   * @param sseUrl - Optional SSE URL. If provided, updates the client's SSE URL.
   * @returns A `Promise` that resolves to the `TransactionProcessedEvent` when the transaction is found.
   * @throws An `Error` if the transaction processing times out or if an error occurs during subscription.
   */
  public waitForTransactionProcessed = (
    transactionHash: string,
    timeout?: number,
    sseUrl?: string
  ): Promise<TransactionProcessedEvent> => {
    sseUrl && (this.sseUrl = sseUrl);
    return new Promise((resolve, reject) => {
      if (!this.sseClient) {
        reject(new Error(`SSE Client is not set.`));
      }
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
              .toHex()
              .toLowerCase() === transactionHash.toLowerCase()
          ) {
            clearTimeout(timeoutId);
            this.sseClient.stop();
            this.sseClient.unsubscribe(EventName.TransactionProcessedEventType);
            resolve(processEvent);
          }
        },
        error => {
          this.sseClient.stop();
          clearTimeout(timeoutId);
          reject(error);
        }
      );
      subscription && this.sseClient.start();
    });
  };

  /**
   * Retrieves the RPC client instance used for blockchain interactions.
   *
   * @returns The instance of `RpcClient` currently in use.
   */
  protected get rpcClient() {
    return this._rpcClient;
  }

  /**
   * Retrieves the SSE client instance used for event streaming.
   *
   * @returns The instance of `SseClient` currently in use.
   */
  protected get sseClient() {
    return this._sseClient;
  }

  /**
   * Stops the event stream by stopping the SSE client.
   *
   * @returns The current instance of the `Client` for method chaining.
   */
  protected stopEventStream(): Client {
    this.sseClient?.stop();
    // Clean SseClient instance of its subscriptions
    this._sseClient = new SseClient(this._sseUrl);
    return this;
  }

  /**
   * Sets the contract hash and optionally the contract package hash.
   *
   * This method updates the stored contract hash and, if provided, also updates
   * the contract package hash. It allows method chaining by returning the `Client` instance.
   *
   * @param contractHash - The contract hash to be set.
   * @param contractPackageHash - Optional contract package hash to be set.
   * @returns The current instance of the `Client` for method chaining.
   */
  protected setContractHash(
    contractHash: ContractHash,
    contractPackageHash?: ContractPackageHash
  ): Client {
    contractHash && (this._contractHash = contractHash);
    contractPackageHash && (this._contractPackageHash = contractPackageHash);
    return this;
  }

  /**
   * Starts the event stream by subscribing to transaction processing events.
   *
   * When a transaction is processed, it extracts relevant details such as execution result,
   * transaction hash, messages, and timestamp. If the execution result contains an error,
   * it is handled accordingly. Otherwise, the parsed result is emitted as an event.
   *
   * @param sseUrl - Optional SSE URL. If provided, updates the client's SSE URL.
   * @returns The current instance of the `Client` for method chaining.
   * @throws An `Error` if the SSE client is not set.
   */
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
                  transactionHash: transactionHash,
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
   * Executes a contract entry point with specified arguments and optional transaction processing wait.
   *
   * This method constructs and submits a transaction to invoke a contract entry point.
   * It allows specifying runtime arguments, payment amount, sender, signing keys, and the target network.
   * If `waitForTransactionProcessed` is enabled, the method waits for the transaction to be processed before returning.
   *
   * @param entryPoint - The name of the contract entry point to be invoked.
   * @param runtimeArgs - The runtime arguments required for the contract call.
   * @param paymentAmount - The amount to be paid for transaction execution.
   * @param sender - The public key of the sender initiating the transaction.
   * @param signingKeys - (Optional) An array of private keys to sign the transaction.
   * @param chainName - (Optional) The network name where the transaction is deployed. Defaults to the client's chain name.
   * @param waitForTransactionProcessed - (Optional) If `true`, waits for transaction processing before resolving.
   * @returns A promise resolving to `TransactionResult`, containing transaction info and execution results if applicable.
   * @throws An error if the transaction submission or processing fails.
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
        const processedEvent = await this.waitForTransactionProcessed(
          transactionInfo.transactionHash.toHex()
        );
        return {
          transactionInfo,
          executionResult:
            processedEvent.transactionProcessedPayload.executionResult
        };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(
        `error during transaction execution.\n${transaction.hash.toHex()}\n${error}\n${(error as any)?.sourceErr?.data}`
      );
    }
  }

  /**
   * Retrieves contract data from the global state.
   *
   * This method queries the contract's stored data using the contract hash.
   * An optional path can be provided to access specific subkeys within the contract storage.
   *
   * @param path - (Optional) An array of strings representing subkeys or a path within the contract storage.
   *               If not provided, the query retrieves the top-level contract data.
   * @returns A `Promise` resolving to the contract's stored value (`clValue`) as a string.
   * @throws An error if the contract hash is not set or if the stored value is invalid.
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

  /**
   * Handles execution errors returned from contract execution.
   *
   * If the error message contains a known contract error prefix, it extracts the error code and throws a `ContractError`.
   * Otherwise, it throws a generic `Error` with the provided message.
   *
   * @param errorMessage - The error message returned from contract execution.
   * @throws `ContractError` if the error is a recognized contract error.
   * @throws `Error` if the error message does not match the contract error format.
   */
  protected handleExecutionError(errorMessage: string) {
    if (errorMessage.startsWith(contractErrorMessagePrefix)) {
      const errorCode = parseInt(
        errorMessage.substring(contractErrorMessagePrefix.length),
        10
      );
      console.error(
        `Error: ${new ContractError(errorCode).message}\nError code: ${new ContractError(errorCode).code}`
      );
    } else {
      console.error(`Error: ${new Error(errorMessage).message}`);
    }
  }

  /**
   * Subscribes to the `TransactionProcessedEventType` SSE event.
   *
   * This method listens for transaction-processed events and triggers the provided callback when an event is received.
   * If an error occurs while processing the event, the SSE client is stopped, and the optional `onError` callback is invoked.
   *
   * @param onProcess - Callback function to handle the processed transaction event.
   * @param onError - (Optional) Callback function to handle errors during event processing.
   * @returns `true` if the subscription is successfully established.
   */
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

  /**
   * Parses the execution result to extract CEP18 events.
   *
   * This method initializes the parser and processes the execution result to extract contract events.
   * If any parsing errors occur, they are logged, and only successfully parsed events are returned.
   *
   * @param result - The execution result to parse.
   * @returns A `Promise` resolving to an array of parsed CEP18 events or `undefined` if no valid events were found.
   */
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

  /**
   * Emits an event to all registered listeners.
   *
   * This method triggers all callbacks associated with the given event name,
   * ensuring that all registered listeners receive the event data.
   *
   * @param event - The event to emit.
   */
  private emit(event: CEP18EventResult) {
    this._events[event.name]?.forEach(cb => cb(event));
  }
}
