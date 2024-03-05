// eslint-disable-next-line eslint-comments/disable-enable-pair
/* eslint-disable no-console */
import { BigNumber, type BigNumberish } from '@ethersproject/bignumber';
import {
  CasperServiceByJsonRPC,
  type CLPublicKey,
  encodeBase16,
  EventStream
} from 'casper-js-sdk';

import { CEP18Client, ContractWASM, EVENTS_MODE, InstallArgs } from '../../src';
import {
  DEPLOY_TIMEOUT,
  EVENT_STREAM_ADDRESS,
  NETWORK_NAME,
  NODE_URL,
  users
} from '../config';
import {
  expectDeployResultToSuccess,
  findKeyFromAccountNamedKeys,
  getAccountInfo
} from '../utils';

describe('CEP18Client', () => {
  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  const client = new CasperServiceByJsonRPC(NODE_URL);
  const eventStream = new EventStream(EVENT_STREAM_ADDRESS);
  const owner = users[0];
  const ali = users[1];
  const bob = users[2];

  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 200_000_000_000
  };

  const doApprove = async (
    spender: CLPublicKey,
    amount: BigNumberish
  ): Promise<void> => {
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

    await deploy.send(NODE_URL);

    const result = await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);

    expectDeployResultToSuccess(result);

    // check events are parsed properly
    const events = cep18.parseExecutionResult(
      result.execution_results[0].result
    );
    expect(events.length).toEqual(1);
    expect(events[0].name).toEqual('SetAllowance');

    const allowances = await cep18.allowances(owner.publicKey, ali.publicKey);

    expect(allowances.eq(amount));
  };

  beforeAll(async () => {
    const deploy = cep18.install(
      ContractWASM,
      { ...tokenInfo, eventsMode: EVENTS_MODE.CES },
      250_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    await deploy.send(NODE_URL);

    const result = await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);

    expectDeployResultToSuccess(result);

    const accountInfo = await getAccountInfo(NODE_URL, owner.publicKey);

    const contractHash = findKeyFromAccountNamedKeys(
      accountInfo,
      `cep18_contract_hash_${tokenInfo.name}`
    ) as `hash-${string}`;
    cep18.setContractHash(contractHash);

    expectDeployResultToSuccess(result);

    await cep18.setupEventStream(eventStream);

    cep18.on('SetAllowance', event => {
      expect(event.name).toEqual('SetAllowance');
    });
  });

  afterAll(() => {
    eventStream.stop();
  });

  it('should deploy contract', () => {
    const contracHash = cep18.contractHash;

    expect(contracHash).toBeDefined();
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

  it('should owner owns totalSupply amount of tokens', async () => {
    const balance = await cep18.balanceOf(owner.publicKey);

    expect(balance.eq(tokenInfo.totalSupply));
  });

  it('should return 0 when balance info not found from balances dictionary', async () => {
    const consoleWarnMock = jest.spyOn(console, 'warn').mockImplementation();

    const balance = await cep18.balanceOf(ali.publicKey);

    expect(console.warn).toHaveBeenCalledWith(
      `Not found balance for ${encodeBase16(ali.publicKey.value())}`
    );
    expect(console.warn).toHaveBeenCalledTimes(1);
    consoleWarnMock.mockRestore();

    expect(balance.eq(0));
  });

  it('should return 0 when allowances info not found and log warning', async () => {
    const consoleWarnMock = jest.spyOn(console, 'warn').mockImplementation();

    const allowances = await cep18.allowances(owner.publicKey, ali.publicKey);

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

    await deploy.send(NODE_URL);

    const result = await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);

    expectDeployResultToSuccess(result);

    const balance = await cep18.balanceOf(bob.publicKey);

    expect(balance.eq(transferAmount));

    const allowances = await cep18.allowances(owner.publicKey, ali.publicKey);

    expect(allowances.eq(BigNumber.from(amount).sub(transferAmount)));
  });

  it('should transfer tokens', async () => {
    const amount = 50_000_000_000;

    const deploy = cep18.transfer(
      { recipient: ali.publicKey, amount },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    await deploy.send(NODE_URL);

    const result = await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);
    expectDeployResultToSuccess(result);

    const balance = await cep18.balanceOf(ali.publicKey);

    expect(balance.eq(amount));
  });

  it('should throw error when try to transfer more than owned balance', async () => {
    const amount = 5_000_000_000_000;

    const deploy = cep18.transfer(
      { recipient: ali.publicKey, amount },
      5_000_000_000,
      owner.publicKey,
      NETWORK_NAME,
      [owner]
    );

    await deploy.send(NODE_URL);
    await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);
    await expect(
      cep18.parseDeployResult(encodeBase16(deploy.hash))
    ).rejects.toThrowError('InsufficientBalance');
  });
});
