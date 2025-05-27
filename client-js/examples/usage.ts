import {
  PRIVATE_KEY_FAUCET,
  SSE_URL,
  PRIVATE_KEY_USER_1,
  PRIVATE_KEY_USER_2,
  CHAIN_NAME,
  RPC_URL
} from '../config';
import {
  CEP18Client,
  TransferArgs,
  TransferFromArgs,
  ApproveArgs,
  TransactionParams
} from '../dist';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey
} from '../tests/utils';

// Here you can check examples how to check balance, approve tokens, transfer tokens, and transfer tokens by allowance

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}
if (!PRIVATE_KEY_USER_2) {
  throw new Error('PRIVATE_KEY_USER_2 environment variable is not set.');
}

const name = 'TEST_CEP18',
  owner = getSigningKey(PRIVATE_KEY_FAUCET),
  ali = getSigningKey(PRIVATE_KEY_USER_1),
  bob = getSigningKey(PRIVATE_KEY_USER_2),
  waitForTransactionProcessed = true;

const usage = async () => {
  const accountInfo = await getAccountInfo(RPC_URL, owner.publicKey),
    contractHash = findKeyFromAccountNamedKeys(
      accountInfo,
      `cep18_contract_hash_${name}`
    );

  const cep18 = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME).setContractHash(
    contractHash
  );
  console.info(`Contract Hash: ${cep18.contractHash.toPrefixedString()}`);

  // Fetch token info
  const token_name = await cep18.name(),
    symbol = await cep18.symbol(),
    decimals = await cep18.decimals(),
    totalSupply = await cep18.totalSupply();

  console.info('tokenInfo: ', {
    token_name,
    symbol,
    decimals: decimals.toString(),
    totalSupply: totalSupply.toString()
  });

  // Fetch token balance
  const balance = await cep18.balanceOf(owner.publicKey);
  console.info('Owner token balance: ', balance.toString());

  // Transfer tokens
  let params: TransactionParams = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner]
  };

  const transferArgs: TransferArgs = {
    recipient: ali.publicKey,
    amount: String(1_000_000_000)
  };

  let { transactionInfo, executionResult } = await cep18.transfer({
    params,
    args: transferArgs,
    waitForTransactionProcessed
  });

  if (executionResult?.errorMessage) {
    throw new Error(
      `Error during transfer.\n${executionResult?.errorMessage.toString()}`
    );
  } else {
    console.info(
      `Token transfer transaction hash: ${transactionInfo.transactionHash.toHex()}`
    );
    console.info(`Transfer cost consumed: ${executionResult?.consumed}`);
  }

  const aliBalance = await cep18.balanceOf(ali.publicKey);
  console.info(`Ali's balance: ${aliBalance.toString()}`);

  // Approve tokens
  const approveArgs: ApproveArgs = {
    spender: ali.publicKey,
    amount: String(5_000_000_000)
  };

  ({ transactionInfo, executionResult } = await cep18.approve({
    params,
    args: approveArgs,
    waitForTransactionProcessed
  }));

  if (executionResult?.errorMessage) {
    throw new Error(
      `Error during approval.\n${executionResult?.errorMessage.toString()}`
    );
  } else {
    console.info(
      `Token approval transaction hash: ${transactionInfo.transactionHash.toHex()}`
    );
    console.info(`Approval cost consumed: ${executionResult?.consumed}`);
  }

  // Get allowances
  const allowances = await cep18.allowances(owner.publicKey, ali.publicKey);
  console.info(
    `Allowances from ${owner.publicKey.toHex()} to ${ali.publicKey.toHex()} : ${allowances.toString()}`
  );

  // Transfer tokens by allowances
  params = {
    sender: ali.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [ali]
  };

  const transferFromArgs: TransferFromArgs = {
    owner: owner.publicKey,
    recipient: bob.publicKey,
    amount: String(2_000_000_000)
  };

  ({ transactionInfo, executionResult } = await cep18.transferFrom({
    params,
    args: transferFromArgs,
    waitForTransactionProcessed
  }));

  if (executionResult?.errorMessage) {
    throw new Error(
      `Error during transferFrom.\n${executionResult?.errorMessage.toString()}`
    );
  } else {
    console.info(`TransferFrom cost consumed: ${executionResult?.consumed}`);
  }

  const bobBalance = await cep18.balanceOf(bob.publicKey);
  console.info(`Bob's balance: ${bobBalance.toString()}`);
};

usage()
  .then(() => {
    console.info('Usage completed successfully.');
  })
  .catch(error => {
    console.error('Usage failed:', error);
  });
