import { expect, describe, it, beforeEach } from 'vitest';
import { RPC_URL, SSE_URL, CHAIN_NAME } from '../../config';
import {
  CEP18Client,
  ChangeSecurityArgs,
  EVENTS_MODE,
  TransactionParams,
  TransferArgs,
  TransferFromArgs
} from '../../src';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';
import {
  install,
  owner,
  ali,
  approve,
  bob,
  mint,
  increaseAllowance
} from './helpers';

let client: CEP18Client;
const name = `TEST_CEP18_E2E_${Math.floor(Math.random() * 1000000)}`;

describe('CEP18Client - E2E Usage', () => {
  beforeEach(async () => {
    client = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);
    const account = await getAccountInfo(RPC_URL, owner.publicKey),
      contractHash = findKeyFromAccountNamedKeys(
        account,
        `cep18_contract_hash_${name}`
      );
    expect(contractHash).toBeDefined();
    client.setContractHash(contractHash);
  }, 180000);

  it('should transfer tokens successfully', async () => {
    const initialBalance = await client.balanceOf(owner.publicKey),
      initialBalanceAli = await client.balanceOf(ali.publicKey),
      params: TransactionParams = {
        sender: owner.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [owner]
      },
      transferArgs: TransferArgs = {
        recipient: ali.publicKey,
        amount: String(1_000_000_000)
      },
      transferResult = await client.transfer({
        params,
        args: transferArgs,
        waitForTransactionProcessed: true
      });

    expect(transferResult.transactionInfo.transactionHash).toBeDefined();
    expect(transferResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(owner.publicKey);
    expect(BigInt(newBalance)).toBe(
      BigInt(initialBalance) - BigInt(1_000_000_000)
    );

    const newBalanceAli = await client.balanceOf(ali.publicKey);
    expect(BigInt(newBalanceAli)).toBe(
      BigInt(initialBalanceAli) + BigInt(1_000_000_000)
    );
  }, 180000);

  it('should approve tokens successfully', async () => {
    const approveResult = await approve(client);

    expect(approveResult.transactionInfo.transactionHash).toBeDefined();
    expect(approveResult.executionResult?.errorMessage).toBeFalsy();

    const allowances = await client.allowances(owner.publicKey, ali.publicKey);
    expect(BigInt(allowances)).toBe(BigInt(5_000_000_000));
  }, 180000);

  it('should transfer tokens by allowance successfully', async () => {
    await approve(client);

    const initialBobBalance = await client.balanceOf(bob.publicKey),
      params = {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali]
      },
      transferFromArgs: TransferFromArgs = {
        owner: owner.publicKey,
        recipient: bob.publicKey,
        amount: String(2_000_000_000)
      },
      transferFromResult = await client.transferFrom({
        params,
        args: transferFromArgs,
        waitForTransactionProcessed: true
      });

    expect(transferFromResult.transactionInfo.transactionHash).toBeDefined();

    expect(transferFromResult.executionResult?.errorMessage).toBeFalsy();

    const newBobBalance = await client.balanceOf(bob.publicKey);
    expect(BigInt(newBobBalance)).toBe(
      BigInt(initialBobBalance) + BigInt(2_000_000_000)
    );
  }, 180000);

  it('should increase allowance successfully', async () => {
    await approve(client);

    const initialAllowance = await client.allowances(
      owner.publicKey,
      ali.publicKey
    );

    const increaseAmount = BigInt(2_000_000_000);

    const increaseAllowanceResult = await increaseAllowance(
      client,
      increaseAmount
    );

    expect(
      increaseAllowanceResult.transactionInfo.transactionHash
    ).toBeDefined();
    expect(increaseAllowanceResult.executionResult?.errorMessage).toBeFalsy();

    const newAllowance = await client.allowances(
      owner.publicKey,
      ali.publicKey
    );
    expect(BigInt(newAllowance)).toBe(
      BigInt(initialAllowance) + increaseAmount
    );
  }, 180000);

  it('should decrease allowance successfully', async () => {
    await approve(client);

    const increaseAmount = BigInt(2_000_000_000);

    await increaseAllowance(client, increaseAmount);

    const initialAllowance = await client.allowances(
      owner.publicKey,
      ali.publicKey
    );

    const decreaseAmount = BigInt(1_000_000_000);

    const decreaseAllowanceResult = await client.decreaseAllowance({
      params: {
        sender: owner.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [owner]
      },
      args: {
        spender: ali.publicKey,
        amount: String(decreaseAmount)
      },
      waitForTransactionProcessed: true
    });

    expect(
      decreaseAllowanceResult.transactionInfo.transactionHash
    ).toBeDefined();
    expect(decreaseAllowanceResult.executionResult?.errorMessage).toBeFalsy();

    const newAllowance = await client.allowances(
      owner.publicKey,
      ali.publicKey
    );
    expect(BigInt(newAllowance)).toBe(
      BigInt(initialAllowance) - decreaseAmount
    );
  }, 180000);

  it('should mint tokens successfully', async () => {
    const initialBalance = await client.balanceOf(ali.publicKey);
    const mintAmount = BigInt(10_000_000_000);

    const mintResult = await mint(client, mintAmount);

    expect(mintResult.transactionInfo.transactionHash).toBeDefined();
    expect(mintResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(ali.publicKey);
    expect(BigInt(newBalance)).toBe(BigInt(initialBalance) + mintAmount);
  }, 180000);

  it('should burn tokens successfully', async () => {
    const mintAmount = BigInt(10_000_000_000);

    await mint(client, mintAmount);

    const initialBalance = await client.balanceOf(ali.publicKey);

    const burnAmount = BigInt(10_000_000_000);

    const burnResult = await client.burn({
      params: {
        sender: ali.publicKey,
        paymentAmount: String(5_000_000_000),
        signingKeys: [ali]
      },
      args: {
        owner: ali.publicKey,
        amount: String(burnAmount)
      },
      waitForTransactionProcessed: true
    });

    expect(burnResult.transactionInfo.transactionHash).toBeDefined();
    expect(burnResult.executionResult?.errorMessage).toBeFalsy();

    const newBalance = await client.balanceOf(ali.publicKey);
    expect(BigInt(newBalance)).toBe(BigInt(initialBalance) - burnAmount);
  }, 180000);

  it('should change security settings successfully', async () => {
    const newAdminList = [owner.publicKey];
    const newMinterList = [ali.publicKey];
    const newNoneList = [bob.publicKey];

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner]
    };

    const changeSecurityArgs: ChangeSecurityArgs = {
      adminList: newAdminList,
      minterList: newMinterList,
      noneList: newNoneList
    };

    const changeSecurityResult = await client.changeSecurity({
      params,
      args: changeSecurityArgs,
      waitForTransactionProcessed: true
    });

    expect(changeSecurityResult.transactionInfo.transactionHash).toBeDefined();
    expect(changeSecurityResult.executionResult?.errorMessage).toBeFalsy();
  }, 180000);

  it('should change events mode successfully', async () => {
    const eventsMode = EVENTS_MODE.Native;

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(3_000_000_000),
      signingKeys: [owner]
    };

    const changeEventsModeResult = await client.changeEventsMode({
      params,
      args: { eventsMode },
      waitForTransactionProcessed: true
    });

    expect(
      changeEventsModeResult.transactionInfo.transactionHash
    ).toBeDefined();
    expect(changeEventsModeResult.executionResult?.errorMessage).toBeFalsy();
    const newEventsMode = await client.eventsMode();
    expect(newEventsMode === eventsMode.toString());
  }, 180000);
});
