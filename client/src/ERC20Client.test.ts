import * as fs from 'node:fs';
import * as path from 'node:path';

import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import { CasperClient, type CLPublicKey, encodeBase16 } from 'casper-js-sdk';

import { CHAIN_NAME, NODE_URL, users } from './config';
import { ERC20Client, type InstallArgs } from './ERC20Client';
import {
  expectDeployResultToSuccess,
  findKeyFromAccountNamedKeys,
  getAccountInfo
} from './utils';

describe('ERC20Client', () => {
  const client = new CasperClient(NODE_URL);
  const erc20 = new ERC20Client(client);

  const owner = users[0];
  const ali = users[1];
  const bob = users[2];

  const tokenInfo: InstallArgs = {
    name: 'TEST ERC20',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 50_000_000_000
  };

  const doApprove = async (
    spender: CLPublicKey,
    amount: BigNumberish
  ): Promise<void> => {
    const deploy = erc20.approve(
      {
        spender,
        amount
      },
      5_000_000_000,
      owner.publicKey,
      CHAIN_NAME,
      [owner]
    );

    await client.putDeploy(deploy);

    const result = await client.nodeClient.waitForDeploy(deploy);

    expectDeployResultToSuccess(result);

    const allowances = await erc20.allowances(owner.publicKey, ali.publicKey);

    expect(allowances.eq(amount));
  };

  beforeAll(async () => {
    const wasm = new Uint8Array(
      fs.readFileSync(
        path.resolve(
          __dirname,
          '../../target/wasm32-unknown-unknown/release/erc20_token.wasm'
        ),
        null
      ).buffer
    );

    const deploy = erc20.installERC20(
      wasm,
      tokenInfo,
      60_000_000_000,
      owner.publicKey,
      CHAIN_NAME,
      [owner]
    );

    await client.putDeploy(deploy);

    const result = await client.nodeClient.waitForDeploy(deploy);

    const accountInfo = await getAccountInfo(NODE_URL, owner.publicKey);

    const contarctHash = findKeyFromAccountNamedKeys(
      accountInfo,
      'erc20_token_contract'
    );
    erc20.setContractHash(contarctHash);

    expectDeployResultToSuccess(result);
  });

  it('should deploy contract', () => {
    const contracHash = erc20.contractHash;

    expect(contracHash).toBeDefined();
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

  it('should owner owns totalSupply amount of tokens', async () => {
    const balance = await erc20.balanceOf(owner.publicKey);

    expect(balance.eq(tokenInfo.totalSupply));
  });

  it('should return 0 when balance info not found from balances dictionary', async () => {
    const consoleWarnMock = jest.spyOn(console, 'warn').mockImplementation();

    const balance = await erc20.balanceOf(ali.publicKey);

    expect(console.warn).toHaveBeenCalledWith(
      `Not found balance for ${encodeBase16(ali.publicKey.value())}`
    );
    expect(console.warn).toHaveBeenCalledTimes(1);
    consoleWarnMock.mockRestore();

    expect(balance.eq(0));
  });

  it('should return 0 when allowances info not found and log warning', async () => {
    const consoleWarnMock = jest.spyOn(console, 'warn').mockImplementation();

    const allowances = await erc20.allowances(owner.publicKey, ali.publicKey);

    expect(console.warn).toHaveBeenCalledWith(
      `Not found allowances for ${encodeBase16(owner.publicKey.value())}`
    );
    expect(console.warn).toHaveBeenCalledTimes(1);
    consoleWarnMock.mockRestore();

    expect(allowances.eq(0));
  });

  it('should approve token', async () => {
    const amount = 50_000_000_000;
    await doApprove(ali.publicKey, amount);
  });

  it('should tranfer tokens by allowances', async () => {
    const amount = 50_000_000_000;
    await doApprove(ali.publicKey, amount);

    const transferAmount = 20_000_000_000;

    const deploy = erc20.transferFrom(
      {
        owner: owner.publicKey,
        recipient: bob.publicKey,
        amount: transferAmount
      },
      5_000_000_000,
      ali.publicKey,
      CHAIN_NAME,
      [ali]
    );

    await client.putDeploy(deploy);

    const result = await client.nodeClient.waitForDeploy(deploy);

    expectDeployResultToSuccess(result);

    const balance = await erc20.balanceOf(bob.publicKey);

    expect(balance.eq(transferAmount));

    const allowances = await erc20.allowances(owner.publicKey, ali.publicKey);

    expect(allowances.eq(BigNumber.from(amount).sub(transferAmount)));
  });

  it('should transfer tokens', async () => {
    const amount = 50_000_000_000;

    const deploy = erc20.transfer(
      { recipient: ali.publicKey, amount },
      5_000_000_000,
      owner.publicKey,
      CHAIN_NAME,
      [owner]
    );

    await client.putDeploy(deploy);

    const result = await client.nodeClient.waitForDeploy(deploy);

    expectDeployResultToSuccess(result);

    const balance = await erc20.balanceOf(ali.publicKey);

    expect(balance.eq(amount));
  });
});
