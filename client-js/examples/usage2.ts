import { CasperServiceByJsonRPC } from 'casper-js-sdk';

import { CEP18Client, InstallArgs } from '../src';
import { findKeyFromAccountNamedKeys, getAccountInfo } from '../tests/utils';
import {
  DEPLOY_TIMEOUT,
  FAUCET_KEY,
  NETWORK_NAME,
  NODE_URL,
  USER1_KEY
} from './common';

// Here you can check examples how to mint and burn tokens

const run = async () => {
  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  const client = new CasperServiceByJsonRPC(NODE_URL);

  const owner = FAUCET_KEY;
  const ali = USER1_KEY;

  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 200_000_000_000
  };
  const accountInfo = await getAccountInfo(NODE_URL, owner.publicKey);

  const contractHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_hash_${tokenInfo.name}`
  ) as `hash-${string}`;

  const contractPackageHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_package_${tokenInfo.name}`
  ) as `hash-${string}`;

  cep18.setContractHash(contractHash, contractPackageHash);
  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);

  // Mint tokens
  const mintDeploy = cep18.mint(
    { owner: ali.publicKey, amount: 10_000_000_000 },
    5_000_000_000,
    owner.publicKey,
    NETWORK_NAME,
    [owner]
  );
  const mintDeployHash = await mintDeploy.send(NODE_URL);
  console.log(`...Token mint deploy hash: ${mintDeployHash}`);
  await client.waitForDeploy(mintDeploy, DEPLOY_TIMEOUT);
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
    { owner: ali.publicKey, amount: 10_000_000_000 },
    5_000_000_000,
    owner.publicKey,
    NETWORK_NAME,
    [owner]
  );
  const burnDeployHash = await burnDeploy.send(NODE_URL);
  console.log(`...Token burn deploy hash: ${burnDeployHash}`);
  await client.waitForDeploy(burnDeploy, DEPLOY_TIMEOUT);
  const newBalance = await cep18.balanceOf(ali.publicKey);
  console.log(
    `...Token burned Successfully, Ali's balance: ${newBalance.toString()}`
  );
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
run();
