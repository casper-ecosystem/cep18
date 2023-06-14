import { CasperServiceByJsonRPC } from 'casper-js-sdk';

import { CEP18Client, ContractWASM, InstallArgs } from '../src';
import { DEPLOY_TIMEOUT, NETWORK_NAME, NODE_URL, users } from '../tests/config';
import {
  expectDeployResultToSuccess,
  findKeyFromAccountNamedKeys,
  getAccountInfo
} from '../tests/utils';

const install = async () => {
  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  const client = new CasperServiceByJsonRPC(NODE_URL);

  const owner = users[0];

  const tokenInfo: InstallArgs = {
    name: 'TEST CEP18',
    symbol: 'TFT',
    decimals: 9,
    totalSupply: 200_000_000_000
  };

  const deploy = cep18.install(
    ContractWASM,
    tokenInfo,
    150_000_000_000,
    owner.publicKey,
    NETWORK_NAME,
    [owner]
  );

  const deployHash = await deploy.send(NODE_URL);

  console.log(`... Contract installation deployHash: ${deployHash}`);

  const result = await client.waitForDeploy(deploy, DEPLOY_TIMEOUT);

  expectDeployResultToSuccess(result);

  console.log(`... Contract installed successfully.`);

  const accountInfo = await getAccountInfo(NODE_URL, owner.publicKey);

  const contractHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_hash_${tokenInfo.name}`
  ) as `hash-${string}`;

  const contractPackageHash = findKeyFromAccountNamedKeys(
    accountInfo,
    `cep18_contract_package_hash_${tokenInfo.name}`
  ) as `hash-${string}`;

  console.log(`... Contract Hash: ${contractHash}`);
  console.log(`... Contract Package Hash: ${contractPackageHash}`);
};

// eslint-disable-next-line @typescript-eslint/no-floating-promises
install();
