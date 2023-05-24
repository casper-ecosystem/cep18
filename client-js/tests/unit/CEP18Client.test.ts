// eslint-disable-next-line eslint-comments/disable-enable-pair
/* eslint-disable no-console */

import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import { type CLPublicKey, DeployUtil } from 'casper-js-sdk';

import { ContractWASM } from '../../src';
import CEP18Client from '../../src/CEP18Client';
import { InstallArgs } from '../../src/types';
import { NETWORK_NAME, NODE_URL, users } from '../config';
import APPROVE_ARGS_JSON from './json/approve-args.json';
import BURN_ARGS_JSON from './json/burn-args.json';
import CHANGE_SECURITY_ARGS_JSON from './json/change-security-args.json';
import DECREASE_ALLOWANCE_ARGS_JSON from './json/decrase-allowance-args.json';
import INCREASE_ALLOWANCE_ARGS_JSON from './json/incrase-allowance-args.json';
import INSTALL_ARGS_JSON from './json/install-args.json';
import MINT_ARGS_JSON from './json/mint-args.json';
import TRANSFER_ARGS_JSON from './json/transfer-args.json';
import TRANSFER_FROM_ARGS_JSON from './json/transfer-from-args.json';

describe('CEP18Client', () => {
  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);

  const owner = users[0];
  const ali = users[1];
  const bob = users[2];

  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 50_000_000_000
  };

  let spies: jest.SpyInstance[] = [];

  const doApprove = (spender: CLPublicKey, amount: BigNumberish) => {
    const deploy = cep18.approve(
      {
        spender,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'approve'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      APPROVE_ARGS_JSON
    );
  };

  beforeAll(() => {
    const deploy = cep18.install(
      ContractWASM,
      tokenInfo,
      60_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );
    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);
    cep18.setContractHash(
      'hash-6797fc45c106bd1f4c9f00cb416d63fd71fecfb90ba8f9c24e597b678569d095'
    );

    const mockedFns = {
      name: async () => Promise.resolve(tokenInfo.name),
      symbol: async () => Promise.resolve(tokenInfo.symbol),
      decimals: async () => Promise.resolve(BigNumber.from(tokenInfo.decimals)),
      totalSupply: async () =>
        Promise.resolve(BigNumber.from(tokenInfo.totalSupply)),
      balanceOf: async () =>
        Promise.resolve(BigNumber.from(tokenInfo.totalSupply))
    };

    spies = Object.keys(mockedFns).map(m => {
      const method = m as keyof typeof mockedFns;
      return jest.spyOn(cep18, method).mockImplementation(mockedFns[method]);
    });

    expect(deploy).toBeInstanceOf(DeployUtil.Deploy);
    expect((JsonDeploy as any).session.ModuleBytes.args).toEqual(
      INSTALL_ARGS_JSON
    );
  });

  afterAll(() => {
    spies.forEach(spy => spy.mockClear());
  });

  it('should match on-chain info with install info', async () => {
    const name = await cep18.name();
    const symbol = await cep18.symbol();
    const decimals = await cep18.decimals();
    const totalSupply = await cep18.totalSupply();

    expect(name).toBe(tokenInfo.name);
    expect(symbol).toBe(tokenInfo.symbol);
    expect(decimals.eq(tokenInfo.decimals));
    expect(totalSupply.eq(tokenInfo.totalSupply));
  });

  it('should construct approve args properly', () => {
    const amount = 50_000_000_000;
    doApprove(ali.publicKey, amount);
  });

  it('should construct transfer_from args properly', () => {
    const transferAmount = 20_000_000_000;

    const deploy = cep18.transferFrom(
      {
        owner: owner.publicKey,
        recipient: bob.publicKey,
        amount: transferAmount
      },
      5_000_000_000,
      ali.publicKey,
      NETWORK_NAME,
      [ali]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'transfer_from'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      TRANSFER_FROM_ARGS_JSON
    );
  });

  it('should construct transfer args properly', () => {
    const amount = 50_000_000_000;

    const deploy = cep18.transfer(
      { recipient: ali.publicKey, amount },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'transfer'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      TRANSFER_ARGS_JSON
    );
  });

  it('should construct increaseAllowance args properly', () => {
    const spender = ali.publicKey;
    const amount = 50_000_000_000;
    const deploy = cep18.increaseAllowance(
      {
        spender,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'increase_allowance'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      INCREASE_ALLOWANCE_ARGS_JSON
    );
  });

  it('should construct decreaseAllowance args properly', () => {
    const spender = ali.publicKey;
    const amount = 50_000_000_000;
    const deploy = cep18.decreaseAllowance(
      {
        spender,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'decrease_allowance'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      DECREASE_ALLOWANCE_ARGS_JSON
    );
  });

  it('should construct mint args properly', () => {
    const recipient = ali.publicKey;
    const amount = 50_000_000_000;
    const deploy = cep18.mint(
      {
        owner: recipient,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'mint'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      MINT_ARGS_JSON
    );
  });

  it('should construct burn args properly', () => {
    const recipient = ali.publicKey;
    const amount = 50_000_000_000;
    const deploy = cep18.burn(
      {
        owner: recipient,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'burn'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      BURN_ARGS_JSON
    );
  });

  it('should revert if no args were provided to changeSecurity', () => {
    const invalidFunc = () =>
      cep18.changeSecurity({}, 5_000_000_000, owner.publicKey, NETWORK_NAME, [
        owner
      ]);

    expect(invalidFunc).toThrowError('Should provide at least one arg');
  });

  it('should construct changeSecurity args properly', () => {
    const minterList = [ali.publicKey];
    const burnerList = [ali.publicKey, bob.publicKey];
    const deploy = cep18.changeSecurity(
      {
        adminList: [owner.publicKey],
        minterList,
        burnerList
      },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);

    expect((JsonDeploy as any).session.StoredContractByHash.entry_point).toBe(
      'change_security'
    );
    expect((JsonDeploy as any).session.StoredContractByHash.args).toEqual(
      CHANGE_SECURITY_ARGS_JSON
    );
  });
});
