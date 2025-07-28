import { expect, describe, it, beforeEach } from 'vitest';
import { RPC_URL, SSE_URL, CHAIN_NAME } from '../../config';
import { CEP18Client, TransactionParams, TransactionResult } from '../../src';
import wasm from '../../src/wasm/cep18';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';
import {
  owner,
  paymentAmount,
  symbol,
  decimals,
  totalSupply,
  eventsMode,
  enableMintAndBurn,
  install
} from './helpers';

let client: CEP18Client;

describe('CEP18Client - E2E Install', () => {
  beforeEach(() => {
    client = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
  });

  it('should install the CEP18 contract and return valid transaction info', async () => {
    const name = `TEST_CEP18_E2E_${Math.floor(Math.random() * 1000000)}`,
      params: TransactionParams = {
        wasm,
        sender: owner.publicKey,
        paymentAmount,
        signingKeys: [owner]
      },
      args = {
        name,
        symbol,
        decimals,
        totalSupply,
        eventsMode,
        enableMintAndBurn
      },
      transactionResult: TransactionResult = await client.install({
        params,
        args,
        waitForTransactionProcessed: false
      });

    expect(
      transactionResult.transactionInfo.transactionHash.toHex()
    ).toBeTruthy();
  });

  it('should install the CEP18 contract and return valid transaction result', async () => {
    const name = `TEST_CEP18_E2E_${Math.floor(Math.random() * 1000000)}`,
      transactionResult: TransactionResult = await install(client, name);

    expect(
      transactionResult.transactionInfo.transactionHash.toHex()
    ).toBeTruthy();
    expect(transactionResult.executionResult?.consumed).toBeTruthy();
    expect(transactionResult.executionResult?.errorMessage).toBeFalsy();

    // Check for the contract hash after installation
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
