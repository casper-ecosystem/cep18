import {
  CEP18Client,
  ContractWASM as wasm,
  EVENTS_MODE,
  InstallArgs,
  InstallParams,
  InstallResult
} from '../dist';
import {
  CHAIN_NAME,
  FAUCET_PRIVATE_KEY,
  RPC_URL,
  SSE_URL
} from '../tests/config';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey
} from '../tests/utils';

if (!FAUCET_PRIVATE_KEY) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

const name = 'TEST CEP18';
const symbol = 'TFT';
const decimals = 9;
const sender = getSigningKey(FAUCET_PRIVATE_KEY);

const install = async (): Promise<InstallResult> => {
  const params: InstallParams = {
    wasm,
    sender: sender.publicKey,
    paymentAmount: String(350_000_000_000),
    signingKeys: [sender]
  };

  // The events mode is disabled by default, to enable CES events you should set `eventsMode`.
  // Mint and burn is also disabled by default, if you need to enable burn and mint you should set `enableMintAndBurn` as true.
  // You couldn't change this once the token is deployed.
  const args: InstallArgs = {
    name,
    symbol,
    decimals,
    totalSupply: String(200_000_000_000),
    eventsMode: EVENTS_MODE.CES,
    enableMintAndBurn: true
  };

  const cep18 = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
  const waitForTransactionProcessed = true;

  const installResult: InstallResult = await cep18.install({
    params,
    args,
    waitForTransactionProcessed
  });

  if (!installResult.transactionResult.transactionHash) {
    throw Error('Invalid transaction hash');
  }

  return installResult;
};

install()
  .then(async installResult => {
    console.info(
      `... Contract installation transaction hash: ${installResult.transactionResult.transactionHash}`
    );

    if (installResult.executionResult?.errorMessage) {
      throw new Error(
        `Error during installation.\n${installResult.executionResult?.errorMessage.toString()}`
      );
    } else {
      console.info(
        `... Contract installation cost consumed: ${installResult.executionResult?.consumed}`
      );
    }

    const account = await getAccountInfo(RPC_URL, sender.publicKey);

    const contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_hash_${name}`
    );

    const contractPackageHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_package_${name}`
    );

    console.info(`... Contract Hash: ${contractHash}`);
    console.info(`... Contract Package Hash: ${contractPackageHash}`);
  })
  .catch(error => {
    console.error('... Installation failed:', error);
  });
