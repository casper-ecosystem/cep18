import { expect, describe, it, beforeEach } from 'vitest';
import { owner, paymentAmount, install } from './helpers';
import { RPC_URL, SSE_URL, CHAIN_NAME, PRIVATE_KEY_FAUCET } from '../../config';
import {
  CEP18Client,
  EVENTS_MODE,
  TransactionParams,
  TransactionResult
} from '../../src';
import wasm from '../../src/wasm/cep18';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

let client: CEP18Client;
const name = `TEST_CEP18_E2E_${Math.floor(Math.random() * 1000000)}`;

describe('CEP18Client - E2E Upgrade', () => {
  beforeEach(async () => {
    client = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);
  }, 180000);

  it('should upgrade the CEP18 contract and return valid transaction info', async () => {
    const params: TransactionParams = {
        wasm,
        sender: owner.publicKey,
        paymentAmount,
        signingKeys: [owner]
      },
      args = {
        name,
        eventsMode: EVENTS_MODE.Native
      },
      transactionResult: TransactionResult = await client.upgrade({
        params,
        args,
        waitForTransactionProcessed: false
      });

    expect(
      transactionResult.transactionInfo.transactionHash.toHex()
    ).toBeTruthy();
  });

  it('should upgrade the CEP18 contract and return valid transaction result', async () => {
    const params: TransactionParams = {
        wasm,
        sender: owner.publicKey,
        paymentAmount,
        signingKeys: [owner]
      },
      args = {
        name,
        eventsMode: EVENTS_MODE.Native
      },
      transactionResult: TransactionResult = await client.upgrade({
        params,
        args,
        waitForTransactionProcessed: true
      });

    expect(
      transactionResult.transactionInfo.transactionHash.toHex()
    ).toBeTruthy();
    expect(transactionResult.executionResult?.consumed).toBeTruthy();
    expect(transactionResult.executionResult?.errorMessage).toBeFalsy();

    const account = await getAccountInfo(RPC_URL, owner.publicKey);
    const contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_hash_${name}`
    );
    expect(contractHash).toBeDefined();
    const contractPackageHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_package_${name}`
    );
    expect(contractPackageHash).toBeDefined();
  }, 180000);
});
