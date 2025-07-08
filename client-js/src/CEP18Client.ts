import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';
import {
  Args as RuntimeArgs,
  CLTypeKey,
  CLValue,
  ContractHash,
  ContractPackageHash,
  Key,
  ParamDictionaryIdentifier,
  ParamDictionaryIdentifierContractNamedKey,
  PublicKey,
  SessionBuilder,
  AddressableEntityHash
} from 'casper-js-sdk';
import { Base64 } from 'js-base64';
import Client from './client';
import {
  EVENTS_MODE,
  type InstallParams,
  type TransactionResult,
  type TransferParams,
  type TransferFromParams,
  type ApproveParams,
  type DecreaseAllowanceParams,
  type MintParams,
  type BurnParams,
  type ChangeSecurityParams,
  UpgradeParams,
  Entity,
  ChangeEventsModeParams
} from './types';
import ContractWASM from './wasm/cep18';

/**
 * CEP18Client extends the base `Client` class to provide specific functionality
 * for interacting with CEP-18 token contracts on the Casper blockchain.
 */
export default class CEP18Client extends Client {
  /**
   * Initializes a new CEP18Client instance.
   *
   * @param rpcUrl - The RPC URL of the Casper network.
   * @param ssUrl - (Optional) The SSE URL for event streaming.
   * @param chainName - (Optional) The name of the blockchain network.
   */
  constructor(rpcUrl: string, ssUrl?: string, chainName?: string) {
    super(rpcUrl, ssUrl, chainName);
  }

  /**
   * Sets the contract hash and optionally the contract package hash.
   *
   * This method removes prefixes from the provided contract hash and package hash
   * before converting them into the appropriate `ContractHash` and `ContractPackageHash` objects.
   *
   * @param contractHash - The contract hash as a string or `ContractHash` instance.
   * @param contractPackageHash - (Optional) The contract package hash as a string or `ContractPackageHash` instance.
   * @returns The updated `CEP18Client` instance.
   * @throws `Error` if the contract hash is not provided or invalid.
   */
  public setContractHash(
    contractHash: string | ContractHash,
    contractPackageHash?: string | ContractPackageHash
  ): CEP18Client {
    const removePrefix = (str: string | undefined) =>
      str ? str.replace(/^.*-/, '') : '';

    const hexContractHash =
        typeof contractHash === 'string' ? removePrefix(contractHash) : '',
      hexContractPackageHash =
        typeof contractPackageHash === 'string'
          ? removePrefix(contractPackageHash)
          : '',
      newContractHash = hexContractHash
        ? ContractHash.newContract(hexContractHash)
        : undefined,
      newContractPackageHash = hexContractPackageHash
        ? ContractPackageHash.newContractPackage(hexContractPackageHash)
        : undefined;

    if (!newContractHash) {
      throw new Error('Contract hash must be provided.');
    }
    return super.setContractHash(
      newContractHash,
      newContractPackageHash
    ) as unknown as CEP18Client;
  }

  /**
   * Starts the SSE event stream to listen for contract-related events.
   *
   * This method enables real-time event listening for the contract by calling
   * the parent `startEventStream` method.
   *
   * @param sseUrl - (Optional) The SSE endpoint URL. If not provided, the previously set URL is used.
   * @returns The updated `CEP18Client` instance.
   */
  public startEventStream(sseUrl?: string): CEP18Client {
    return super.startEventStream(sseUrl) as unknown as CEP18Client;
  }

  /**
   * Stops the SSE event stream, preventing further event processing.
   *
   * This method ensures that the event stream is properly stopped and unsubscribed.
   *
   * @returns The updated `CEP18Client` instance.
   */
  public stopEventStream(): CEP18Client {
    return super.stopEventStream() as unknown as CEP18Client;
  }

  /**
   * Installs the CEP-18 contract on the Casper network.
   *
   * @param params - The installation parameters, including:
   *   - `wasm`: The compiled contract in `Uint8Array` format.
   *   - `paymentAmount`: The amount of payment required for contract installation.
   *   - `sender`: The public key of the account deploying the contract.
   *   - `chainName`: (Optional) The name of the network where the contract will be deployed.
   *   - `signingKeys`: (Optional) An array of private keys used for signing the transaction.
   *   - `args`: Contract-specific arguments, including:
   *     - `name`: Token name.
   *     - `symbol`: Token symbol.
   *     - `decimals`: Number of decimal places for the token.
   *     - `totalSupply`: The total supply of the token.
   *     - `eventsMode`: (Optional) The mode in which events are emitted.
   *     - `enableMintAndBurn`: (Optional) A boolean indicating whether minting and burning are enabled.
   *     - `adminList`: List of accounts with admin privileges.
   *     - `minterList`: List of accounts allowed to mint tokens.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed.
   *
   * @returns A `Promise` resolving to `TransactionResult`, containing the transaction details.
   *
   * @throws Will throw an error if the Wasm file is missing or if an error occurs during installation.
   *
   * @remarks
   * This method installs a new CEP-18 contract on the Casper network. It requires a compiled Wasm contract file and includes necessary arguments such as the token name, symbol, decimals, and total supply.
   * If `waitForTransactionProcessed` is `true`, it waits for the transaction to be processed and returns the execution result.
   * Ensure that the Wasm file is valid and the required arguments are properly provided before invoking the method.
   */
  public async install(params: InstallParams): Promise<TransactionResult> {
    const {
      params: { wasm, paymentAmount, sender, chainName, signingKeys },
      args: {
        name,
        symbol,
        decimals,
        totalSupply,
        eventsMode,
        enableMintAndBurn,
        adminList,
        minterList
      }
    } = params;

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

    if (adminList) {
      runtimeArgs.insert(
        'admin_list',
        CLValue.newCLList(
          CLTypeKey,
          adminList.map(key => CLValue.newCLKey(this.getPrefixedString(key)))
        )
      );
    }
    if (minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          minterList.map(key => CLValue.newCLKey(this.getPrefixedString(key)))
        )
      );
    }

    const wasmBytes = wasm || ContractWASM;

    if (!wasmBytes) {
      throw new Error('Wasm file is missing.');
    }

    const transaction = new SessionBuilder()
      .installOrUpgrade()
      .wasm(wasmBytes)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach(key => transaction.sign(key));
    }
    try {
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (
        params.waitForTransactionProcessed &&
        transactionInfo.transactionHash
      ) {
        const transactionProcessedEvent =
          await this.waitForTransactionProcessed(
            transactionInfo.transactionHash.toHex()
          );
        const executionResult =
          transactionProcessedEvent.transactionProcessedPayload.executionResult;
        if (executionResult?.errorMessage) {
          this.handleExecutionError(executionResult.errorMessage);
        }
        return {
          transactionInfo,
          executionResult
        };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(
        `Error during installation runtime.\n${error}\n${(error as any)?.sourceErr?.data}`
      );
    }
  }

  /**
   * Upgrades an existing contract with a new version.
   *
   * @param params - Parameters for the contract upgrade.
   * @param params.wasm - The WASM contract file representing the new version of the contract as a Uint8Array.
   * @param params.paymentAmount - The payment amount required for the upgrade.
   * @param params.sender - The transaction sender's account.
   * @param params.chainName - The name of the network where the contract will be deployed.
   * @param params.signingKeys - Array of signing keys. If provided, the transaction will be signed with these keys.
   * @param params.args.name - The name of the token for the new contract version.
   * @param params.args.eventsMode - The event mode for the new contract version (optional).
   *
   * @returns A `Promise` that resolves to a `TransactionResult` object containing transaction details and the execution result.
   *
   * @throws Error if the WASM file is missing or there is an error during the upgrade process.
   *
   * @remarks
   * This method allows upgrading an existing contract to a new version. If the `eventsMode` argument is provided, it is added to the contract's runtime arguments.
   * The `wasm` argument should be the compiled contract in the form of a Uint8Array. Once the upgrade transaction is processed, the result will be returned, including any execution results.
   */
  public async upgrade(params: UpgradeParams): Promise<TransactionResult> {
    const {
      params: { wasm, paymentAmount, sender, chainName, signingKeys },
      args: { name, eventsMode }
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValue.newCLString(name)
    });

    if (eventsMode !== undefined) {
      runtimeArgs.insert('events_mode', CLValue.newCLUint8(eventsMode));
    }

    const wasmBytes = wasm || ContractWASM;

    if (!wasmBytes) {
      throw new Error('Wasm file is missing.');
    }

    const transaction = new SessionBuilder()
      .installOrUpgrade()
      .wasm(wasmBytes)
      .runtimeArgs(runtimeArgs)
      .payment(Number(paymentAmount))
      .from(sender)
      .chainName(chainName ? chainName : this.chainName || '')
      .build();

    if (signingKeys) {
      signingKeys.forEach(key => transaction.sign(key));
    }
    try {
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (
        params.waitForTransactionProcessed &&
        transactionInfo.transactionHash
      ) {
        const transactionProcessedEvent =
          await this.waitForTransactionProcessed(
            transactionInfo.transactionHash.toHex()
          );
        const executionResult =
          transactionProcessedEvent.transactionProcessedPayload.executionResult;
        if (executionResult?.errorMessage) {
          this.handleExecutionError(executionResult.errorMessage);
        }
        return {
          transactionInfo,
          executionResult
        };
      }
      return { transactionInfo };
    } catch (error) {
      throw new Error(
        `Error during upgrade runtime.\n${error}\n${(error as any)?.sourceErr?.data}`
      );
    }
  }

  /**
   * Transfers tokens from the sender to another user.
   *
   * @param params - The transfer parameters, including:
   *   - `args`: Contains the transfer details:
   *     - `recipient`: The recipient's public key or account address.
   *     - `amount`: The amount of tokens to transfer.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender initiating the transfer.
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network to which the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public transfer(params: TransferParams): Promise<TransactionResult> {
    const {
      args: { recipient, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValue.newCLKey(this.getPrefixedString(recipient)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'transfer',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Transfers tokens from an approved user to another user.
   *
   * This method allows the transfer of tokens from one user (the owner) to another user (the recipient),
   * based on prior approval. The `owner` must approve the transaction before the `sender` can initiate the transfer.
   *
   * @param params - The transfer parameters, including:
   *   - `args`: Contains the transfer details:
   *     - `owner`: The public key or account address of the owner (the user who is approving the transfer).
   *     - `recipient`: The recipient's public key or account address.
   *     - `amount`: The amount of tokens to transfer.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the one initiating the transfer on behalf of the owner).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network to which the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public transferFrom(params: TransferFromParams): Promise<TransactionResult> {
    const {
      args: { owner, recipient, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(this.getPrefixedString(owner)),
      recipient: CLValue.newCLKey(this.getPrefixedString(recipient)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'transfer_from',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Approves tokens for another user to spend on behalf of the sender.
   *
   * This method allows the sender to approve another user (the spender) to spend a specified amount of tokens
   * on their behalf. The approved amount can then be used for transactions such as transfers.
   *
   * @param params - The approval parameters, including:
   *   - `args`: Contains the approval details:
   *     - `spender`: The public key or account address of the spender (the user who will be authorized to spend tokens).
   *     - `amount`: The amount of tokens that the spender is authorized to spend.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the user who is granting approval).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network to which the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public approve(params: ApproveParams): Promise<TransactionResult> {
    const {
      args: { spender, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(this.getPrefixedString(spender)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'approve',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Increases the allowance for the spender to spend more tokens on behalf of the sender.
   *
   * This method allows the sender to increase the amount of tokens that a specified spender is allowed to spend
   * on their behalf. It effectively adds to the current approved allowance, allowing the spender to spend more tokens.
   *
   * @param params - The parameters for increasing the allowance, including:
   *   - `args`: Contains the details of the increase:
   *     - `spender`: The public key or account address of the spender (the user who will be authorized to spend additional tokens).
   *     - `amount`: The amount of tokens to add to the spender’s current allowance.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the user granting the additional allowance).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network to which the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public increaseAllowance(params: ApproveParams): Promise<TransactionResult> {
    const {
      args: { spender, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(this.getPrefixedString(spender)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'increase_allowance',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Decreases the allowance for the spender to spend fewer tokens on behalf of the sender.
   *
   * This method allows the sender to decrease the amount of tokens that a specified spender is allowed to spend
   * on their behalf. It effectively subtracts from the current approved allowance, reducing the spender’s ability
   * to transfer tokens.
   *
   * @param params - The parameters for decreasing the allowance, including:
   *   - `args`: Contains the details of the decrease:
   *     - `spender`: The public key or account address of the spender (the user who will have their allowance decreased).
   *     - `amount`: The amount of tokens to subtract from the spender’s current allowance.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the user reducing the spender’s allowance).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network to which the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public decreaseAllowance(
    params: DecreaseAllowanceParams
  ): Promise<TransactionResult> {
    const {
      args: { spender, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(this.getPrefixedString(spender)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'decrease_allowance',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Creates a specified amount of tokens and assigns them to the specified owner.
   * This operation increases the total supply of tokens in circulation.
   *
   * @param params - The parameters for minting tokens, including:
   *   - `args`: Contains the details of the mint operation:
   *     - `owner`: The public key or account address of the owner who will receive the newly minted tokens.
   *     - `amount`: The amount of tokens to mint and assign to the owner.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the user initiating the mint operation).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network where the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public mint(params: MintParams): Promise<TransactionResult> {
    const {
      args: { owner, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(this.getPrefixedString(owner)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'mint',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Destroys a specified amount of tokens from the given owner, decreasing the total supply of tokens.
   *
   * @param params - The parameters for burning tokens, including:
   *   - `args`: Contains the details of the burn operation:
   *     - `owner`: The public key or account address of the owner whose tokens are being burned.
   *     - `amount`: The amount of tokens to burn from the owner's balance.
   *   - `paymentAmount`: The amount of payment required for executing the transaction.
   *   - `sender`: The public key of the sender (the user initiating the burn operation).
   *   - `signingKeys`: (Optional) An array of private keys used to sign the transaction.
   *   - `chainName`: (Optional) The name of the network where the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the details of the transaction.
   *
   * @throws Will throw an error if the transaction execution fails or if any of the required parameters are missing.
   */
  public burn(params: BurnParams): Promise<TransactionResult> {
    const {
      args: { owner, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(this.getPrefixedString(owner)),
      amount: CLValue.newCLUInt256(amount)
    });
    return this.callEntrypoint(
      'burn',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Changes the token security settings by updating various user lists with specific roles and permissions.
   * This operation modifies the lists of admins, minters, burners, mint-and-burn operators, and none-role users.
   *
   * @param params - The parameters for changing the token security settings, including:
   *   - `args`: Contains the details of the security change:
   *     - `adminList`: List of accounts with admin privileges.
   *     - `minterList`: List of accounts allowed to mint tokens.
   *     - `noneList`: List of accounts that do not have any specific roles.
   *   - `paymentAmount`: The payment amount required for executing the transaction.
   *   - `sender`: The public key of the sender initiating the change.
   *   - `signingKeys`: (Optional) An array of signing keys to sign the transaction.
   *   - `chainName`: (Optional) The network name where the transaction will be deployed.
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits for the transaction to be processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the transaction details.
   *
   * @throws Will throw an error if no arguments are provided or if the transaction execution fails.
   */
  public changeSecurity(
    params: ChangeSecurityParams
  ): Promise<TransactionResult> {
    const {
      args: { adminList, minterList, noneList },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({});
    // Add optional args
    if (adminList) {
      runtimeArgs.insert(
        'admin_list',
        CLValue.newCLList(
          CLTypeKey,
          adminList.map(key => CLValue.newCLKey(this.getPrefixedString(key)))
        )
      );
    }
    if (minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          minterList.map(key => CLValue.newCLKey(this.getPrefixedString(key)))
        )
      );
    }
    if (noneList) {
      runtimeArgs.insert(
        'none_list',
        CLValue.newCLList(
          CLTypeKey,
          noneList.map(key => CLValue.newCLKey(this.getPrefixedString(key)))
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
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Changes the events emission mode for the token contract.
   * This determines how the contract emits events, which may affect off-chain indexing or client behavior.
   *
   * @param params - The parameters for changing the events mode, including:
   *   - `args`: An object containing:
   *     - `eventsMode`: The mode in which events will be emitted.
   *   - `paymentAmount`: The payment amount required for executing the transaction.
   *   - `sender`: The public key of the sender initiating the change.
   *   - `signingKeys`: (Optional) An array of signing keys used to authorize the transaction.
   *   - `chainName`: (Optional) The name of the network (e.g., "casper-test").
   *   - `waitForTransactionProcessed`: (Optional) If `true`, waits until the transaction is fully processed before resolving.
   *
   * @returns A `Promise` that resolves to a `TransactionResult` containing the transaction details and outcome.
   *
   * @throws Will throw an error if the transaction fails to be submitted or executed.
   */
  public changeEventsMode(
    params: ChangeEventsModeParams
  ): Promise<TransactionResult> {
    const {
      args: { eventsMode },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      events_mode: CLValue.newCLUint8(eventsMode)
    });

    return this.callEntrypoint(
      'change_events_mode',
      runtimeArgs,
      paymentAmount,
      sender,
      signingKeys,
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Retrieves the balance of a given account by querying the contract's balance storage.
   *
   * @param account - The account whose balance is being queried. This should be a `PublicKey` instance representing the account's public key.
   *
   * @returns A `Promise` that resolves to a string representing the account's balance. If no balance is found, it will return `'0'`.
   *
   * @throws Will throw an error if the query fails due to an issue other than a missing balance.
   *
   * @remarks The method queries the contract's balance storage by constructing a dictionary identifier for the account's balance.
   *          If no balance is found, it logs a warning to the console.
   */
  public async balanceOf(account: Entity): Promise<string> {
    const entity = this.getPrefixedString(account);
    const keyAccount = entity.bytes();
    const dictionaryItemKey = Base64.fromUint8Array(keyAccount);

    if (!this.contractHash) {
      throw Error('Contract hash is not set.');
    }
    // ! TODO toPrefixedString() ?
    const key = `hash-${this.contractHash?.hash?.toHex()}`;

    const contractNamedKey: ParamDictionaryIdentifierContractNamedKey =
      new ParamDictionaryIdentifierContractNamedKey(
        key,
        'balances',
        dictionaryItemKey
      );

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
      if (error instanceof Error && error.toString().includes('Query failed')) {
        console.warn(`No balance found for ${entity.toPrefixedString()}`);
      } else throw error;
    }
    return balance;
  }

  /**
   * Retrieves the approved allowance from the owner to the spender.
   *
   * @param owner - The owner whose allowance is being queried. This should be a `PublicKey` instance representing the owner's public key.
   * @param spender - The spender for whom the allowance is being queried. This should be a `PublicKey` instance representing the spender's public key.
   *
   * @returns A `Promise` that resolves to a string representing the approved allowance. If no allowance is found, it will return `'0'`.
   *
   * @throws Will throw an error if the query fails due to an issue other than a missing allowance.
   *
   * @remarks The method constructs a dictionary identifier using both the owner's and spender's account information.
   *          It then queries the contract's allowance storage. If no allowance is found, it logs a warning to the console.
   *          The allowance is returned as a string.
   */
  public async allowances(owner: Entity, spender: Entity): Promise<string> {
    const entityOwner = this.getPrefixedString(owner);
    const keyOwner = entityOwner.bytes();
    const entitySpender = this.getPrefixedString(spender);
    const keySpender = entitySpender.bytes();

    const finalBytes = new Uint8Array(keyOwner.length + keySpender.length);
    finalBytes.set(keyOwner);
    finalBytes.set(keySpender, keyOwner.length);

    const blaked = blake2b(finalBytes, { dkLen: 32 });
    const dictionaryItemKey = bytesToHex(blaked);

    if (!this.contractHash) {
      throw Error('Contract hash is not set.');
    }

    // ! TODO toPrefixedString() ?
    const key = `hash-${this.contractHash?.hash?.toHex()}`;

    const contractNamedKey: ParamDictionaryIdentifierContractNamedKey =
      new ParamDictionaryIdentifierContractNamedKey(
        key,
        'allowances',
        dictionaryItemKey
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
        console.warn(`No allowances found for ${entityOwner}`);
      } else throw error;
    }
    return allowances;
  }

  /**
   * Returns the name of the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a string representing the name of the token.
   */
  public async name(): Promise<string> {
    return this.queryContractData(['name']) as Promise<string>;
  }

  /**
   * Returns the symbol of the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a string representing the symbol of the token.
   */
  public async symbol(): Promise<string> {
    return this.queryContractData(['symbol']) as Promise<string>;
  }

  /**
   * Returns the decimals of the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a string representing the decimals of the token.
   */
  public async decimals(): Promise<string> {
    return this.queryContractData(['decimals']) as Promise<string>;
  }

  /**
   * Returns the total supply of the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a string representing the total supply of the token.
   */
  public async totalSupply(): Promise<string> {
    return this.queryContractData(['total_supply']) as Promise<string>;
  }

  /**
   * Returns the event mode of the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a key of the `EVENTS_MODE` enum, indicating the event mode of the token.
   *
   * @remarks This method queries the `events_mode` field from the contract and returns the corresponding key from the `EVENTS_MODE` enum.
   */
  public async eventsMode(): Promise<keyof typeof EVENTS_MODE> {
    const internalValue = (await this.queryContractData([
      'events_mode'
    ])) as string;

    return EVENTS_MODE[internalValue] as keyof typeof EVENTS_MODE;
  }

  /**
   * Returns `true` if mint and burn are enabled for the CEP-18 token.
   *
   * @returns A `Promise` that resolves to a boolean indicating whether minting and burning are enabled.
   *
   * @remarks This method queries the `enable_mint_burn` field from the contract and returns `true` if minting and burning are enabled,
   *          otherwise it returns `false`.
   */
  public async isMintAndBurnEnabled(): Promise<boolean> {
    const internalValue = (await this.queryContractData([
      'enable_mint_burn'
    ])) as string;
    return internalValue !== '0';
  }

  // ! TODO toPrefixedString() ?
  // Error: prefix is not found, source: contract-0x, see Key.newKey()
  private getPrefixedString(entity: Entity): Key {
    if (entity instanceof PublicKey) {
      return Key.newKey(entity.accountHash().toPrefixedString());
    }
    if (
      entity instanceof ContractHash ||
      entity instanceof ContractPackageHash
    ) {
      return Key.newKey(`hash-${entity.hash.toHex()}`);
    }
    return Key.newKey((entity as AddressableEntityHash).toPrefixedString());
  }
}
