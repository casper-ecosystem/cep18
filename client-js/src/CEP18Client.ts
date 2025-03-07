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
  type PublicKey,
  SessionBuilder,
  KeyTypeID
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
  type ChangeSecurityParams
} from './types';

export default class CEP18Client extends Client {
  constructor(rpcUrl: string, ssUrl?: string, chainName?: string) {
    super(rpcUrl, ssUrl, chainName);
  }

  public setContractHash(
    contractHash: string | ContractHash,
    contractPackageHash?: string | ContractPackageHash
  ): CEP18Client {
    const removePrefix = (str: string | undefined) =>
      str ? str.replace(/^[^-]+-/, '') : '';

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

  public startEventStream(sseUrl?: string): CEP18Client {
    return super.startEventStream(sseUrl) as unknown as CEP18Client;
  }

  public stopEventStream(): CEP18Client {
    return super.stopEventStream() as unknown as CEP18Client;
  }

  /**
   * Intalls CEP-18
   * @param wasm contract representation of Uint8Array
   * @param args contract install arguments @see {@link InstallArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, sign transaction if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
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
        enableMintAndBurn
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
    if (!wasm) {
      throw new Error('Wasm file is missing.');
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
      const transactionInfo = await this.rpcClient.putTransaction(transaction);
      if (
        params.waitForTransactionProcessed &&
        transactionInfo.transactionHash
      ) {
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
   * @returns TransactionResult promise
   */
  public transfer(params: TransferParams): Promise<TransactionResult> {
    const {
      args: { recipient, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValue.newCLKey(
        Key.newKey(recipient.accountHash().toPrefixedString())
      ),
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
   * Transfer tokens from the approved user to another user
   * @param args @see {@link TransferFromArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public transferFrom(params: TransferFromParams): Promise<TransactionResult> {
    const {
      args: { owner, recipient, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(owner.accountHash().toPrefixedString())
      ),
      recipient: CLValue.newCLKey(
        Key.newKey(recipient.accountHash().toPrefixedString())
      ),
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
   * Approve tokens to other user
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public approve(params: ApproveParams): Promise<TransactionResult> {
    const {
      args: { spender, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(
        Key.newKey(spender.accountHash().toPrefixedString())
      ),
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
   * Increase allowance to the spender
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public increaseAllowance(params: ApproveParams): Promise<TransactionResult> {
    const {
      args: { spender, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;

    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValue.newCLKey(
        Key.newKey(spender.accountHash().toPrefixedString())
      ),
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
   * Decrease allowance from the spender
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
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
      spender: CLValue.newCLKey(
        Key.newKey(spender.accountHash().toPrefixedString())
      ),
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
   * Create `amount` tokens and assigns them to `owner`.
   * Increases the total supply
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public mint(params: MintParams): Promise<TransactionResult> {
    const {
      args: { owner, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(owner.accountHash().toPrefixedString())
      ),
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
   * Destroy `amount` tokens from `owner`. Decreases the total supply
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public burn(params: BurnParams): Promise<TransactionResult> {
    const {
      args: { owner, amount },
      params: { sender, paymentAmount, signingKeys, chainName },
      waitForTransactionProcessed
    } = params;
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValue.newCLKey(
        Key.newKey(owner.accountHash().toPrefixedString())
      ),
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
   * Change token security
   * @param args @see {@link ChangeSecurityArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender transaction sender
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @param chainName network name which will be deployed to
   * @returns TransactionResult promise
   */
  public changeSecurity(
    params: ChangeSecurityParams
  ): Promise<TransactionResult> {
    const {
      args: { adminList, minterList, burnerList, mintAndBurnList, noneList },
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
          adminList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (minterList) {
      runtimeArgs.insert(
        'minter_list',
        CLValue.newCLList(
          CLTypeKey,
          minterList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (burnerList) {
      runtimeArgs.insert(
        'burner_list',
        CLValue.newCLList(
          CLTypeKey,
          burnerList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (mintAndBurnList) {
      runtimeArgs.insert(
        'mint_and_burn_list',
        CLValue.newCLList(
          CLTypeKey,
          mintAndBurnList.map(key =>
            CLValue.newCLKey(Key.newKey(key.accountHash().toPrefixedString()))
          )
        )
      );
    }
    if (noneList) {
      runtimeArgs.insert(
        'none_list',
        CLValue.newCLList(
          CLTypeKey,
          noneList.map(key =>
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
      chainName,
      waitForTransactionProcessed
    );
  }

  /**
   * Returns the given account's balance
   * @param account account info to get balance
   * @returns account's balance
   */
  public async balanceOf(account: PublicKey): Promise<string> {
    const keyAccount = Key.newKey(
      account.accountHash().toPrefixedString()
    ).bytes();
    const dictionaryItemKey = Base64.fromUint8Array(keyAccount);

    // ! TODO toPrefixedString() ?
    const key = `hash-${this.contractHash.hash.toHex()}`;

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
      if (
        error instanceof Error &&
        error.toString().includes('Error: Query failed')
      ) {
        console.warn(`Not balance found for ${account.toHex()}`);
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
}
