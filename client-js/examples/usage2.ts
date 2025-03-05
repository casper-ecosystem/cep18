import { CEP18Client, InstallArgs } from '../src';
import { findKeyFromAccountNamedKeys, getAccountInfo } from '../tests/utils';
import { TRANSACTION_TIMEOUT, CHAIN_NAME, RPC_URL } from './config';

// Here you can check examples how to mint and burn tokens

const usage = async () => {
  const cep18 = new CEP18Client(RPC_URL, CHAIN_NAME);

  const owner = FAUCET_KEY;
  const ali = USER_1_KEY;

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

  // Mint tokens
  const mintDeploy = cep18.mint(
    { owner: ali.publicKey, amount: String(10_000_000_000) },
    5_000_000_000,
    owner.publicKey,
    CHAIN_NAME,
    [owner]
  );
  const mintDeployHash = await mintDeploy.send(RPC_URL);
  console.log(`...Token mint deploy hash: ${mintDeployHash}`);
  await client.waitForDeploy(mintDeploy, TRANSACTION_TIMEOUT);
  const aliBalance = await cep18.balanceOf(ali.publicKey);
  console.log(
    `...Token minted Successfully, Ali's balance: ${aliBalance.toString()}`
  );

  const isMintAndBurnEnabled = await cep18.isMintAndBurnEnabled();

  if (!isMintAndBurnEnabled) {
    console.warn(`Mint and Burn is disabled.`);
    return;
  }

  // Burn tokens
  const burnDeploy = cep18.burn(
    { owner: ali.publicKey, amount: String(10_000_000_000) },
    String(5_000_000_000),
    owner.publicKey,
    [owner],
    CHAIN_NAME
  );
  const burnDeployHash = await burnDeploy.send(RPC_URL);
  console.log(`...Token burn deploy hash: ${burnDeployHash}`);
  await client.waitForDeploy(burnDeploy, TRANSACTION_TIMEOUT);
  const newBalance = await cep18.balanceOf(ali.publicKey);
  console.log(
    `...Token burned Successfully, Ali's balance: ${newBalance.toString()}`
  );
};

usage()
  .then(() => {
    console.log('Usage completed successfully.');
  })
  .catch(error => {
    console.error('Usage failed:', error);
  });
