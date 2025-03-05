import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';
import {
  Args as RuntimeArgs,
  CLTypeKey,
  CLValue,
  ContractCallBuilder,
  type Hash,
  type InfoGetTransactionResult,
  Key,
  ParamDictionaryIdentifier,
  ParamDictionaryIdentifierContractNamedKey,
  type PrivateKey,
  type PublicKey,
  type PutTransactionResult,
  type QueryGlobalStateResult,
  SessionBuilder,
  TransactionProcessedEvent,
  EventName,
  RawEvent
} from 'casper-js-sdk';
import { Base64 } from 'js-base64';
import { ContractError } from './error';
import TypedContract from './TypedContract';
import {
  ApproveArgs,
  BurnArgs,
  ChangeSecurityArgs,
  EVENTS_MODE,
  InstallPayload,
  InstallResult,
  MintArgs,
  TransferArgs,
  TransferFromArgs
} from './types';

export const DEFAULT_TRANSACTION_TIMEOUT = 120_000;

export default class CEP18Client extends TypedContract {
  constructor(rpcUrl: string, ssUrl?: string, chainName?: string) {
    super(rpcUrl, ssUrl, chainName);
  }

  public setContractHash(contractHash: Hash, contractPackageHash?: Hash) {
    this.setContractHash(contractHash, contractPackageHash);
  }

  public get contractHash(): Hash {
    return this.contractHash;
  }

  public get contractPackageHash(): Hash {
    return this.contractPackageHash;
  }

  /**
   * Intalls CEP-18
   * @param wasm contract representation of Uint8Array
   * @param args contract install arguments @see {@link InstallArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, sign transaction if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public async install(payload: InstallPayload): Promise<InstallResult> {
    const {
      params: { wasm, paymentAmount, sender, chainName, signingKeys },
      args: {
        name,
        symbol,
        decimals,
        totalSupply,
        eventsMode,
        enableMintAndBurn
      }
    } = payload;

    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValue.newCLString(name),
      symbol: CLValue.newCLString(symbol),
      decimals: CLValue.newCLUint8(decimals),
      total_supply: CLValue.newCLUInt256(totalSupply)
    });

    if (eventsMode !== undefined) {
      runtimeArgs.insert('events_mode', CLValue.newCLUint8(eventsMode));
    }
    if (enableMintAndBurn !== undefined) {
      runtimeArgs.insert(
        'enable_mint_burn',
        CLValue.newCLUint8(enableMintAndBurn ? 1 : 0)
      );
    }
    const transaction = new SessionBuilder()
      .installOrUpgrade()
      .wasm(wasm)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach(key => transaction.sign(key));
    }
    try {
      const transactionResult =
        await this.rpcClient.putTransaction(transaction);
      if (
        payload.waitForTransactionProcessed &&
        transactionResult.transactionHash
      ) {
        const deployEvent = await this.waitForTransactionProcessed(
          transactionResult.transactionHash.toString()
        );
        return {
          transactionResult,
          executionResult:
            deployEvent.transactionProcessedPayload.executionResult
        };
      }
      return { transactionResult };
    } catch (error) {
      throw new Error(`Error during installation.\n${error}`);
    }
  }

  /**
   * Transfers tokens to another user
   * @param args @see {@link TransferArgs}
   * @param paymentAmount payment amount required for calling the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, sign transaction if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public transfer(
    args: TransferArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValue.newCLKey(
        Key.newKey(args.recipient.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'transfer',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Transfer tokens from the approved user to another user
   * @param args @see {@link TransferFromArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public transferFrom(
    args: TransferFromArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(args.owner.accountHash().toPrefixedString())
      ),
      recipient: CLValue.newCLKey(
        Key.newKey(args.recipient.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'transfer_from',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Approve tokens to other user
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public approve(
    args: ApproveArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(
        Key.newKey(args.spender.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'approve',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Increase allowance to the spender
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public increaseAllowance(
    args: ApproveArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(
        Key.newKey(args.spender.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'increase_allowance',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Decrease allowance from the spender
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public decreaseAllowance(
    args: ApproveArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(
        Key.newKey(args.spender.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'decrease_allowance',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Create `args.amount` tokens and assigns them to `args.owner`.
   * Increases the total supply
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public mint(
    args: MintArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(args.owner.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'mint',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Destroy `args.amount` tokens from `args.owner`. Decreases the total supply
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public burn(
    args: BurnArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(args.owner.accountHash().toPrefixedString())
      ),
      amount: CLValue.newCLUInt256(args.amount)
    });
    return this.callEntrypoint(
      'burn',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Change token security
   * @param args @see {@link ChangeSecurityArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns PutTransactionResult promise
   */
  public changeSecurity(
    args: ChangeSecurityArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const runtimeArgs = RuntimeArgs.fromMap({});
    // Add optional args
    if (args.adminList) {
      runtimeArgs.insert(
        'admin_list',
        CLValue.newCLList(
          CLTypeKey,
          args.adminList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (args.minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          args.minterList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (args.burnerList) {
      runtimeArgs.insert(
        'burner_list',
        CLValue.newCLList(
          CLTypeKey,
          args.burnerList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (args.mintAndBurnList) {
      runtimeArgs.insert(
        'mint_and_burn_list',
        CLValue.newCLList(
          CLTypeKey,
          args.mintAndBurnList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (args.noneList) {
      runtimeArgs.insert(
        'none_list',
        CLValue.newCLList(
          CLTypeKey,
          args.noneList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    // Check if at least one arg is provided and revert if none was provided
    if (runtimeArgs.args.size === 0) {
      throw new Error('Should provide at least one arg');
    }
    return this.callEntrypoint(
      'change_security',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName
    );
  }

  /**
   * Returns the given account's balance
   * @param account account info to get balance
   * @returns account's balance
   */
  public async balanceOf(account: PublicKey): Promise<string> {
    const key = Key.newKey(account.accountHash().toPrefixedString()).bytes();
    // const keyBytes = CLValue.newCLKey(key).bytes();
    const keyString = String.fromCharCode(...key);
    const dictKey = Base64.encode(keyString);
    // ! TODO
    const test = '';
    const contractNamedKey: ParamDictionaryIdentifierContractNamedKey =
      new ParamDictionaryIdentifierContractNamedKey(test, 'balances', dictKey);

    const identifier = new ParamDictionaryIdentifier(
      undefined,
      contractNamedKey,
      undefined,
      undefined
    );
    let balance = '0';
    try {
      balance =
        (
          await this.rpcClient.getDictionaryItemByIdentifier(null, identifier)
        ).storedValue.clValue?.toString() || balance;
    } catch (error) {
      if (
        error instanceof Error &&
        error.toString().includes('Error: Query failed')
      ) {
        console.warn(`Not found balance for ${account.toHex()}`);
      } else throw error;
    }
    return balance;
  }

  /**
   * Returns approved amount from the owner
   * @param owner owner info
   * @param spender spender info
   * @returns approved amount
   */
  public async allowances(
    owner: PublicKey,
    spender: PublicKey
  ): Promise<string> {
    const keyOwner = Key.newKey(owner.accountHash().toPrefixedString()).bytes();
    const keySpender = Key.newKey(
      spender.accountHash().toPrefixedString()
    ).bytes();

    const finalBytes = new Uint8Array(keyOwner.length + keySpender.length);
    finalBytes.set(keyOwner);
    finalBytes.set(keySpender, keyOwner.length);

    const blaked = blake2b(finalBytes, { dkLen: 32 });
    const dictKey = bytesToHex(blaked);

    // ! TODO
    const test = '';
    const contractNamedKey: ParamDictionaryIdentifierContractNamedKey =
      new ParamDictionaryIdentifierContractNamedKey(
        test,
        'allowances',
        dictKey
      );

    const identifier = new ParamDictionaryIdentifier(
      undefined,
      contractNamedKey,
      undefined,
      undefined
    );

    let allowances = '0';
    try {
      allowances =
        (
          await this.rpcClient.getDictionaryItemByIdentifier(null, identifier)
        ).storedValue.clValue?.toString() || allowances;
    } catch (error) {
      if (
        error instanceof Error &&
        error.toString().includes('Error: Query failed')
      ) {
        console.warn(`Not found allowances for ${owner.toHex()}`);
      } else throw error;
    }
    return allowances;
  }

  /**
   * Returns the name of the CEP-18 token.
   */
  public async name(): Promise<string> {
    return this.queryContractData(['name']) as Promise<string>;
  }

  /**
   * Returns the symbol of the CEP-18 token.
   */
  public async symbol(): Promise<string> {
    return this.queryContractData(['symbol']) as Promise<string>;
  }

  /**
   * Returns the decimals of the CEP-18 token.
   */
  public async decimals(): Promise<string> {
    return this.queryContractData(['decimals']) as Promise<string>;
  }

  /**
   * Returns the total supply of the CEP-18 token.
   */
  public async totalSupply(): Promise<string> {
    return this.queryContractData(['total_supply']) as Promise<string>;
  }

  /**
   * Returns the event mode of the CEP-18 token
   */
  public async eventsMode(): Promise<keyof typeof EVENTS_MODE> {
    const internalValue = (await this.queryContractData([
      'events_mode'
    ])) as string;
    return EVENTS_MODE[internalValue] as keyof typeof EVENTS_MODE;
  }

  /**
   * Returns `true` if mint and burn is enabled
   */
  public async isMintAndBurnEnabled(): Promise<boolean> {
    const internalValue = (await this.queryContractData([
      'enable_mint_burn'
    ])) as string;
    return internalValue !== '0';
  }

  TRANSACTION_TIMEOUT = 30_000; // 30 seconds (adjust as needed)

  waitForTransactionProcessed = (
    transactionHash: string,
    timeout?: number
  ): Promise<TransactionProcessedEvent> => {
    if (!this.sseClient) {
      throw Error('SSE Client is not set.');
    }
    return new Promise((resolve, reject) => {
      const timeoutId = setTimeout(
        () => {
          this.sseClient?.stop();
          reject(new Error('Transaction processing timed out'));
        },
        timeout ? timeout : DEFAULT_TRANSACTION_TIMEOUT
      );

      this.sseClient?.subscribe(
        EventName.TransactionProcessedEventType,
        async (rawEvent: RawEvent) => {
          try {
            const deployEvent = rawEvent.parseAsTransactionProcessedEvent();
            if (
              deployEvent.transactionProcessedPayload.transactionHash.toString() ===
              transactionHash
            ) {
              clearTimeout(timeoutId);
              this.sseClient?.stop();
              resolve(deployEvent);
            }
          } catch (error) {
            console.error('Error processing event:', error);
            clearTimeout(timeoutId);
            this.sseClient?.stop();
            reject(error);
          }
        }
      );
      this.sseClient?.start();
    });
  };

  /**
   * Parse transaction result by given hash.
   * It the transaction wasn't successful, throws `ContractError` if there was operational error, otherwise `Error` with original error message.
   * @param transactionHash transaction hash
   * @returns `InfoGetTransactionResult`
   */
  private async parseDeployResult(
    transactionHash: string
  ): Promise<InfoGetTransactionResult> {
    const result: InfoGetTransactionResult =
      await this.rpcClient.getTransactionByTransactionHash(transactionHash);
    if (result.executionInfo?.executionResult.errorMessage) {
      // Parse execution result
      const { errorMessage } = result.executionInfo.executionResult;
      const contractErrorMessagePrefix = 'User error: ';
      if (errorMessage.startsWith(contractErrorMessagePrefix)) {
        const errorCode = parseInt(
          errorMessage.substring(
            contractErrorMessagePrefix.length,
            errorMessage.length
          ),
          10
        );
        throw new ContractError(errorCode);
      } else throw new Error(errorMessage);
    }
    return result;
  }

  /**
   * Calls a contract entry point with given arguments.
   * @param entryPoint The name of the contract entry point.
   * @param runtimeArgs The runtime arguments for the contract call.
   * @param paymentAmount The payment amount required for execution.
   * @param sender The transaction sender.
   * @param signingKeys (Optional) Array of signing keys to sign the transaction.
   * @param chainName (Optional) Network name where the transaction will be deployed.
   * @returns PutTransactionResult promise.
   */
  private callEntrypoint(
    entryPoint: string,
    runtimeArgs: RuntimeArgs,
    paymentAmount: string,
    sender: PublicKey,
    signingKeys?: PrivateKey[],
    chainName?: string
  ): Promise<PutTransactionResult> {
    const transaction = new ContractCallBuilder()
      .entryPoint(entryPoint)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach(key => transaction.sign(key));
    }

    return this.rpcClient.putTransaction(transaction);
  }

  /**
   * Queries contract data from the global state.
   *
   * @param path Optional array of strings representing the subkeys or path within the contract storage.
   *             If no path is provided, it queries the top-level contract data.
   * @returns A `Promise` resolving to the contract's stored value (`clValue`) or throws an error if the stored value is invalid.
   * @throws Will throw an error if the contract data does not contain a valid stored value.
   */
  private async queryContractData(path: string[] = []): Promise<string> {
    const contractData: QueryGlobalStateResult =
      await this.rpcClient.queryGlobalStateByStateHash(
        null,
        this.contractHash.toHex(),
        path
      );

    const result = contractData.storedValue.clValue?.toString();
    if (result) {
      return result;
    }
    throw Error('Invalid stored value');
  }
}
