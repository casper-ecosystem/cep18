import {
  FAUCET_PRIVATE_KEY,
  SSE_URL,
  USER_1_PRIVATE_KEY,
  USER_2_PRIVATE_KEY
} from 'tests/config';
import {
  CEP18Client,
  TransferArgs,
  TransferFromArgs,
  ApproveArgs,
  TransactionParams
} from '../dist';
import { CHAIN_NAME, RPC_URL } from '../tests/config';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey
} from '../tests/utils';

// Here you can check examples how to check balance, approve tokens, transfer tokens, and transfer tokens by allowance

if (!FAUCET_PRIVATE_KEY) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!USER_1_PRIVATE_KEY) {
  throw new Error('USER_1_PRIVATE_KEY environment variable is not set.');
}
if (!USER_2_PRIVATE_KEY) {
  throw new Error('USER_2_PRIVATE_KEY environment variable is not set.');
}

const name = 'TEST CEP18',
  owner = getSigningKey(FAUCET_PRIVATE_KEY),
  ali = getSigningKey(USER_1_PRIVATE_KEY),
  bob = getSigningKey(USER_2_PRIVATE_KEY),
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
  console.info(` Contract Hash: ${cep18.contractHash.toPrefixedString()}`);

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
    amount: String(10_000_000_000)
  };

  const transferResult = await cep18.transfer({
    params,
    args: transferArgs,
    waitForTransactionProcessed
  });

  console.info(
    `Token transfer transaction hash: ${transferResult.transactionInfo.transactionHash}`
  );

  const aliBalance = await cep18.balanceOf(ali.publicKey);
  console.info(`Ali's balance: ${aliBalance.toString()}`);

  // Approve tokens
  const approveArgs: ApproveArgs = {
    spender: ali.publicKey,
    amount: String(50_000_000_000)
  };

  const approveResult = await cep18.approve({
    params,
    args: approveArgs,
    waitForTransactionProcessed
  });

  console.info(
    `Token approval transaction hash: ${approveResult.transactionInfo.transactionHash}`
  );

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
    amount: String(20_000_000_000)
  };

  const transferFromResult = await cep18.transferFrom({
    params,
    args: transferFromArgs,
    waitForTransactionProcessed
  });

  console.info(
    `Token transferFrom deploy hash: ${transferFromResult.transactionInfo.transactionHash}`
  );

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
