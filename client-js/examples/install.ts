import { CasperServiceByJsonRPC } from 'casper-js-sdk';

import { CEP18Client, ContractWASM, EVENTS_MODE, InstallArgs } from '../src';
import { findKeyFromAccountNamedKeys, getAccountInfo } from '../tests/utils';
import { DEPLOY_TIMEOUT, FAUCET_KEY, NETWORK_NAME, NODE_URL } from './common';

const install = async () => {
  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  const client = new CasperServiceByJsonRPC(NODE_URL);

  const owner = FAUCET_KEY;

  // The events mode is disabled by default, to enable CES events you should set `eventsMode`.
  // Mint and burn is also disabled by default, if you need to enable burn and mint you should set `enableMintAndBurn` as true.
  // You couldn't change this once the token is deployed.
  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 200_000_000_000,
    eventsMode: EVENTS_MODE.CES,
    enableMintAndBurn: true
  };

  const deploy = cep18.install(
    ContractWASM,
    tokenInfo,
    250_000_000_000,
    owner.publicKey,
    NETWORK_NAME,
    [owner]
  );

  const deployHash = await deploy.send(NODE_URL);

  console.log(`... Contract installation deployHash: ${deployHash}`);

  await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);

  console.log(`... Contract installed successfully.`);

  const accountInfo = await getAccountInfo(NODE_URL, owner.publicKey);

  const contractHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_hash_${tokenInfo.name}`
  ) as `hash-${string}`;

  const contractPackageHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_package_${tokenInfo.name}`
  ) as `hash-${string}`;

  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
install();
