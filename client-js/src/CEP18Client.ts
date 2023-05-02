import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import { blake2b } from '@noble/hashes/blake2b';
import {
  CasperClient,
  CasperServiceByJsonRPC,
  type CLKeyParameters,
  type CLPublicKey,
  type CLU256,
  CLValueBuilder,
  CLValueParsers,
  Contracts,
  DeployUtil,
  encodeBase16,
  GetDeployResult,
  type Keys,
  RuntimeArgs
} from 'casper-js-sdk';

import { ContractError } from './error';
import {
  ApproveArgs,
  InstallArgs,
  TransferArgs,
  TransferFromArgs
} from './types';

const { Contract } = Contracts;

export default class CEP18Client {
  public contractClient: Contracts.Contract;

  constructor(public nodeAddress: string, public networkName: string) {
    this.contractClient = new Contract(new CasperClient(nodeAddress));
  }

  public setContractHash(contractHash: string, contractPackageHash?: string) {
    this.contractClient.setContractHash(contractHash, contractPackageHash);
  }

  public get contractHash() {
    return this.contractClient.contractHash;
  }

  public get contractPackageHash() {
    return this.contractClient.contractPackageHash;
  }

  /**
   * Intalls CEP18
   * @param wasm contract representation of Uint8Array
   * @param args contract install arguments @see {@link InstallArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender deploy sender
   * @param chainName chain name which will be deployed to
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @returns Deploy object which can be send to the node.
   */
  public install(
    wasm: Uint8Array,
    args: InstallArgs,
    paymentAmount: BigNumberish,
    sender: CLPublicKey,
    chainName: string,
    signingKeys?: Keys.AsymmetricKey[]
  ): DeployUtil.Deploy {
    const { name, symbol, decimals, totalSupply } = args;
    const runtimeArgs = RuntimeArgs.fromMap({
      name: CLValueBuilder.string(name),
      symbol: CLValueBuilder.string(symbol),
      decimals: CLValueBuilder.u8(decimals),
      total_supply: CLValueBuilder.u256(totalSupply)
    });

    return this.contractClient.install(
      wasm,
      runtimeArgs,
      BigNumber.from(paymentAmount).toString(),
      sender,
      chainName,
      signingKeys
    );
  }

  /**
   * Transfers tokens to another user
   * @param args @see {@link TransferArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender deploy sender
   * @param chainName chain name which will be deployed to
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @returns Deploy object which can be send to the node.
   */
  public transfer(
    args: TransferArgs,
    paymentAmount: BigNumberish,
    sender: CLPublicKey,
    chainName: string,
    signingKeys?: Keys.AsymmetricKey[]
  ): DeployUtil.Deploy {
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: CLValueBuilder.key(args.recipient),
      amount: CLValueBuilder.u256(args.amount)
    });

    return this.contractClient.callEntrypoint(
      'transfer',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

  /**
   * Transfer tokens from the approved user to another user
   * @param args @see {@link TransferFromArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender deploy sender
   * @param chainName chain name which will be deployed to
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @returns Deploy object which can be send to the node.
   */
  public transferFrom(
    args: TransferFromArgs,
    paymentAmount: BigNumberish,
    sender: CLPublicKey,
    chainName: string,
    signingKeys?: Keys.AsymmetricKey[]
  ): DeployUtil.Deploy {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: CLValueBuilder.key(args.owner),
      recipient: CLValueBuilder.key(args.recipient),
      amount: CLValueBuilder.u256(args.amount)
    });

    return this.contractClient.callEntrypoint(
      'transfer_from',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

  /**
   * Approve tokens to other user
   * @param args @see {@link ApproveArgs}
   * @param paymentAmount payment amount required for installing the contract
   * @param sender deploy sender
   * @param chainName chain name which will be deployed to
   * @param signingKeys array of signing keys optional, returns signed deploy if keys are provided
   * @returns Deploy object which can be send to the node.
   */
  public approve(
    args: ApproveArgs,
    paymentAmount: BigNumberish,
    sender: CLPublicKey,
    chainName: string,
    signingKeys?: Keys.AsymmetricKey[]
  ): DeployUtil.Deploy {
    const runtimeArgs = RuntimeArgs.fromMap({
      spender: CLValueBuilder.key(args.spender),
      amount: CLValueBuilder.u256(args.amount)
    });

    return this.contractClient.callEntrypoint(
      'approve',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

  /**
   * Returns the given account's balance
   * @param account account info to get balance
   * @returns account's balance
   */
  public async balanceOf(account: CLKeyParameters): Promise<BigNumber> {
    const keyBytes = CLValueParsers.toBytes(
      CLValueBuilder.key(account)
    ).unwrap();
    const dictKey = Buffer.from(keyBytes).toString('base64');
    let balance = BigNumber.from(0);
    try {
      balance = (
        (await this.contractClient.queryContractDictionary(
          'balances',
          dictKey
        )) as CLU256
      ).value();
    } catch (error) {
      if (
        error instanceof Error &&
        error.toString().startsWith('Error: state query failed: ValueNotFound')
      ) {
        console.warn(`Not found balance for ${encodeBase16(account.data)}`);
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
    owner: CLKeyParameters,
    spender: CLKeyParameters
  ): Promise<BigNumber> {
    const keyOwner = CLValueParsers.toBytes(CLValueBuilder.key(owner)).unwrap();
    const keySpender = CLValueParsers.toBytes(
      CLValueBuilder.key(spender)
    ).unwrap();

    const finalBytes = new Uint8Array(keyOwner.length + keySpender.length);
    finalBytes.set(keyOwner);
    finalBytes.set(keySpender, keyOwner.length);

    const blaked = blake2b(finalBytes, {
      dkLen: 32
    });
    const dictKey = Buffer.from(blaked).toString('hex');

    let allowances = BigNumber.from(0);

    try {
      allowances = (
        (await this.contractClient.queryContractDictionary(
          'allowances',
          dictKey
        )) as CLU256
      ).value();
    } catch (error) {
      if (
        error instanceof Error &&
        error.toString().startsWith('Error: state query failed: ValueNotFound')
      ) {
        console.warn(`Not found allowances for ${encodeBase16(owner.data)}`);
      } else throw error;
    }
    return allowances;
  }

  /**
   * Returns the name of the CEP18 token.
   */
  public async name(): Promise<string> {
    return this.contractClient.queryContractData(['name']) as Promise<string>;
  }

  /**
   * Returns the symbol of the CEP18 token.
   */
  public async symbol(): Promise<string> {
    return this.contractClient.queryContractData(['symbol']) as Promise<string>;
  }

  /**
   * Returns the decimals of the CEP18 token.
   */
  public async decimals(): Promise<BigNumber> {
    return this.contractClient.queryContractData([
      'decimals'
    ]) as Promise<BigNumber>;
  }

  /**
   * Returns the total supply of the CEP18 token.
   */
  public async totalSupply(): Promise<BigNumber> {
    return this.contractClient.queryContractData([
      'total_supply'
    ]) as Promise<BigNumber>;
  }

  /**
   * Parse deploy result by given hash.
   * It the deploy wasn't successful, throws `ContractError` if there was operational error, otherwise `Error` with original error message.
   * @param deployHash deploy hash
   * @returns `GetDeployResult`
   */
  public async parseDeployResult(deployHash: string): Promise<GetDeployResult> {
    const casperClient = new CasperServiceByJsonRPC(this.nodeAddress);

    const result = await casperClient.getDeployInfo(deployHash);
    if (
      result.execution_results.length > 0 &&
      result.execution_results[0].result.Failure
    ) {
      // Parse execution result
      const { error_message } = result.execution_results[0].result.Failure;
      const contractErrorMessagePrefix = 'User error: ';
      if (error_message.startsWith(contractErrorMessagePrefix)) {
        const errorCode = parseInt(
          error_message.substring(
            contractErrorMessagePrefix.length,
            error_message.length
          ),
          10
        );
        throw new ContractError(errorCode);
      } else throw new Error(error_message);
    }
    return result;
  }
}
