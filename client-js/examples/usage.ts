import { CEP18Client, InstallArgs } from '../src';
import { findKeyFromAccountNamedKeys, getAccountInfo } from '../tests/utils';
import { TRANSACTION_TIMEOUT, CHAIN_NAME, RPC_URL } from './config';

// Here you can check examples how to check balance, approve tokens, transfer tokens, and transfer tokens by allowance

const usage = async () => {
  const cep18 = new CEP18Client(RPC_URL, CHAIN_NAME);

  const owner = FAUCET_KEY;
  const ali = USER_1_KEY;
  const bob = USER_2_KEY;

  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: String(200_000_000_000)
  };
  const accountInfo = await getAccountInfo(RPC_URL, owner.publicKey);

  const contractHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_hash_${tokenInfo.name}`
  );

  const contractPackageHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_package_${tokenInfo.name}`
  );

  cep18.setContractHash(contractHash, contractPackageHash);
  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);

  // Fetch token info
  const name = await cep18.name();
  const symbol = await cep18.symbol();
  const decimals = await cep18.decimals();
  const totalSupply = await cep18.totalSupply();

  console.log('tokenInfo: ', {
    name,
    symbol,
    decimals: decimals.toString(),
    totalSupply: totalSupply.toString()
  });

  // Fetch token balance
  const balance = await cep18.balanceOf(owner.publicKey);
  console.log('...Owner token balance: ', balance.toString());

  // Transfer tokens
  const transferDeploy = cep18.transfer(
    { recipient: ali.publicKey, amount: String(10_000_000_000) },
    String(5_000_000_000),
    owner.publicKey,
    [owner],
    CHAIN_NAME
  );
  const transferDeployHash = await transferDeploy.send(RPC_URL);
  console.log(`...Token transfer deploy hash: ${transferDeployHash}`);

  await client.waitForDeploy(transferDeploy, TRANSACTION_TIMEOUT);

  const aliBalance = await cep18.balanceOf(ali.publicKey);
  console.log(`...Ali's balance: ${aliBalance.toString()}`);

  // Approve tokens
  const approveDeploy = cep18.approve(
    { spender: ali.publicKey, amount: String(50_000_000_000) },
    5_000_000_000,
    owner.publicKey,
    CHAIN_NAME,
    [owner]
  );
  const approveDeployHash = await approveDeploy.send(RPC_URL);
  console.log(`...Token approve deploy hash: ${approveDeployHash}`);
  await client.waitForDeploy(approveDeploy, TRANSACTION_TIMEOUT);

  // Get allowances
  const allowances = await cep18.allowances(owner.publicKey, ali.publicKey);
  console.log(
    `...Allowances from ${owner.publicKey.toHex()} to ${ali.publicKey.toHex()} : ${allowances.toString()}`
  );

  // Transfer tokens by allowances
  const transferFromDeploy = cep18.transferFrom(
    {
      owner: owner.publicKey,
      recipient: bob.publicKey,
      amount: String(20_000_000_000)
    },
    String(5_000_000_000),
    ali.publicKey,
    [ali],
    CHAIN_NAME
  );

  const transferFromDeployHash = await transferFromDeploy.send(RPC_URL);
  console.log(`...Token transferFrom deploy hash: ${transferFromDeployHash}`);
  await client.waitForDeploy(transferFromDeploy, TRANSACTION_TIMEOUT);

  const bobBalance = await cep18.balanceOf(bob.publicKey);
  console.log(`...Bob's balance: ${bobBalance.toString()}`);
};

usage()
  .then(() => {
    console.log('Usage completed successfully.');
  })
  .catch(error => {
    console.error('Usage failed:', error);
  });
