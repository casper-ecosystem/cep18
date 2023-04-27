// eslint-disable-next-line eslint-comments/disable-enable-pair
/* eslint-disable no-console */
import * as fs from 'node:fs';
import * as path from 'node:path';

import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import { type CLPublicKey, DeployUtil } from 'casper-js-sdk';

import ERC20Client from '../../src/ERC20Client';
import { InstallArgs } from '../../src/types';
import { NETWORK_NAME, NODE_URL, users } from '../config';
import APPROVE_ARGS_JSON from './json/approve-args.json';
import INSTALL_ARGS_JSON from './json/install-args.json';
import TRANSFER_ARGS_JSON from './json/transfer-args.json';
import TRANSFER_FROM_ARGS_JSON from './json/transfer-from-args.json';

describe('ERC20Client', () => {
  const erc20 = new ERC20Client(NODE_URL, NETWORK_NAME);

  const owner = users[0];
  const ali = users[1];
  const bob = users[2];

  const tokenInfo: InstallArgs = {
    name: 'TEST ERC20',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 50_000_000_000
  };

  let spies: jest.SpyInstance[] = [];

  const doApprove = (spender: CLPublicKey, amount: BigNumberish) => {
    const deploy = erc20.approve(
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
    const wasm = new Uint8Array(
      fs.readFileSync(
        path.resolve(
          __dirname,
          '../../../target/wasm32-unknown-unknown/release/erc20_token.wasm'
        ),
        null
      ).buffer
    );

    const deploy = erc20.install(
      wasm,
      tokenInfo,
      60_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );
    const { deploy: JsonDeploy } = DeployUtil.deployToJson(deploy);
    erc20.setContractHash(
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
      return jest.spyOn(erc20, method).mockImplementation(mockedFns[method]);
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
    const name = await erc20.name();
    const symbol = await erc20.symbol();
    const decimals = await erc20.decimals();
    const totalSupply = await erc20.totalSupply();

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

    const deploy = erc20.transferFrom(
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

    const deploy = erc20.transfer(
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
});
