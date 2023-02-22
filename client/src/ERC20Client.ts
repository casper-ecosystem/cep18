import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import * as blake from 'blakejs';
import {
  type CasperClient,
  type CLKeyParameters,
  type CLPublicKey,
  type CLU256,
  CLValueBuilder,
  CLValueParsers,
  Contracts,
  type DeployUtil,
  encodeBase16,
  type Keys,
  RuntimeArgs
} from 'casper-js-sdk';

const { Contract } = Contracts;

export class ERC20Client extends Contract {
  constructor(
    client: CasperClient,
    contractHash?: string,
    contractPackageHash?: string
  ) {
    super(client);

    if (contractHash) {
      this.setContractHash(contractHash, contractPackageHash);
    }
  }

  public attatch(client: CasperClient): ERC20Client {
    this.casperClient = client;
    return this;
  }

  public installERC20(
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

    return this.install(
      wasm,
      runtimeArgs,
      BigNumber.from(paymentAmount).toString(),
      sender,
      chainName,
      signingKeys
    );
  }

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

    return this.callEntrypoint(
      'transfer',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

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

    return this.callEntrypoint(
      'transfer_from',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

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

    return this.callEntrypoint(
      'approve',
      runtimeArgs,
      sender,
      chainName,
      BigNumber.from(paymentAmount).toString(),
      signingKeys
    );
  }

  public async balanceOf(account: CLKeyParameters): Promise<BigNumber> {
    const keyBytes = CLValueParsers.toBytes(
      CLValueBuilder.key(account)
    ).unwrap();
    const dictKey = Buffer.from(keyBytes).toString('base64');
    let balance = BigNumber.from(0);
    try {
      balance = (
        (await this.queryContractDictionary('balances', dictKey)) as CLU256
      ).value();
    } catch (error) {
      console.warn(`Not found balance for ${encodeBase16(account.data)}`);
    }
    return balance;
  }

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
    const blaked = blake.blake2b(finalBytes, undefined, 32);
    const dictKey = Buffer.from(blaked).toString('hex');

    const balance = (await this.queryContractDictionary(
      'balances',
      dictKey
    )) as CLU256;
    return balance.value();
  }

  /**
   * Returns the name of the ERC20 token.
   */
  public async name(): Promise<string> {
    return await this.queryContractData(['name']);
  }

  /**
   * Returns the symbol of the ERC20 token.
   */
  public async symbol(): Promise<string> {
    return await this.queryContractData(['symbol']);
  }

  /**
   * Returns the decimals of the ERC20 token.
   */
  public async decimals(): Promise<BigNumber> {
    return await this.queryContractData(['decimals']);
  }

  /**
   * Returns the total supply of the ERC20 token.
   */
  public async totalSupply(): Promise<BigNumber> {
    return await this.queryContractData(['total_supply']);
  }
}

export interface InstallArgs {
  name: string;
  symbol: string;
  decimals: BigNumberish;
  totalSupply: BigNumberish;
}

export interface TransferableArgs {
  amount: BigNumberish;
}

export interface TransferArgs extends TransferableArgs {
  recipient: CLKeyParameters;
}

export interface TransferFromArgs extends TransferArgs {
  owner: CLKeyParameters;
}

export interface ApproveArgs extends TransferableArgs {
  spender: CLKeyParameters;
}
