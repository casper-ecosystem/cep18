import {
  FAUCET_PRIVATE_KEY,
  USER_1_PRIVATE_KEY,
  USER_2_PRIVATE_KEY
} from '../../config';
import {
  ApproveArgs,
  CEP18Client,
  EVENTS_MODE,
  TransactionParams,
  TransactionResult
} from '../../src';
import wasm from '../../src/wasm/cep18';
import { getSigningKey } from '../utils';

if (!FAUCET_PRIVATE_KEY) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!USER_1_PRIVATE_KEY) {
  throw new Error('USER_1_PRIVATE_KEY environment variable is not set.');
}
if (!USER_2_PRIVATE_KEY) {
  throw new Error('USER_2_PRIVATE_KEY environment variable is not set.');
}

export const symbol = 'CEP18';
export const decimals = 9;
export const totalSupply = String(200_000_000_000);
export const eventsMode = EVENTS_MODE.CES;
export const enableMintAndBurn = true;
export const paymentAmount = String(350_000_000_000);
export const owner = getSigningKey(FAUCET_PRIVATE_KEY);
export const ali = getSigningKey(USER_1_PRIVATE_KEY);
export const bob = getSigningKey(USER_2_PRIVATE_KEY);

export const install = async (
  client: CEP18Client,
  name: string
): Promise<TransactionResult> => {
  const params: TransactionParams = {
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
    };

  return client.install({
    params,
    args,
    waitForTransactionProcessed: true
  });
};

export const approve = async (
  client: CEP18Client
): Promise<TransactionResult> => {
  const approveArgs: ApproveArgs = {
    spender: ali.publicKey,
    amount: String(5_000_000_000)
  };

  const params: TransactionParams = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner]
  };

  return client.approve({
    params,
    args: approveArgs,
    waitForTransactionProcessed: true
  });
};

export const increaseAllowance = async (
  client: CEP18Client,
  increaseAmount: bigint
): Promise<TransactionResult> => {
  return client.increaseAllowance({
    params: {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner]
    },
    args: {
      spender: ali.publicKey,
      amount: String(increaseAmount)
    },
    waitForTransactionProcessed: true
  });
};

export const mint = async (
  client: CEP18Client,
  mintAmount: bigint,
  waitForTransactionProcessed: boolean = true
): Promise<TransactionResult> => {
  return client.mint({
    params: {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner]
    },
    args: {
      owner: ali.publicKey,
      amount: String(mintAmount)
    },
    waitForTransactionProcessed
  });
};
