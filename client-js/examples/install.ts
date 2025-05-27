import { CHAIN_NAME, PRIVATE_KEY_FAUCET, RPC_URL, SSE_URL } from '../config';
import {
  CEP18Client,
  ContractWASM as wasm,
  EVENTS_MODE,
  type InstallArgs,
  type TransactionParams,
  type TransactionResult
} from '../dist';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey
} from '../tests/utils';

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}

const name = 'TEST_CEP18',
  symbol = 'CEP18',
  decimals = 9,
  totalSupply = String(200_000_000_000),
  // The events mode is disabled by default, to enable CES events you should set `eventsMode`.
  eventsMode = EVENTS_MODE.CES,
  // Mint and burn is also disabled by default, if you need to enable burn and mint you should set `enableMintAndBurn` as true.
  enableMintAndBurn = true,
  waitForTransactionProcessed = true,
  sender = getSigningKey(PRIVATE_KEY_FAUCET),
  paymentAmount = String(500_000_000_000);

const install = async () => {
  const cep18 = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);

  const params: TransactionParams = {
    wasm,
    sender: sender.publicKey,
    paymentAmount,
    signingKeys: [sender]
  };

  const args: InstallArgs = {
    name,
    symbol,
    decimals,
    totalSupply,
    eventsMode,
    enableMintAndBurn
  };

  const transactionResult: TransactionResult = await cep18.install({
    params,
    args,
    waitForTransactionProcessed
  });

  if (!transactionResult.transactionInfo.transactionHash) {
    throw Error('Invalid transaction hash');
  }
  return transactionResult;
};

install()
  .then(async transactionResult => {
    const { transactionInfo, executionResult } = transactionResult;
    console.info(
      `Contract installation transaction hash: ${transactionInfo.transactionHash.toHex()}`
    );

    if (executionResult) {
      if (executionResult?.errorMessage) {
        throw new Error(
          `Error during installation.\n${executionResult?.errorMessage.toString()}`
        );
      } else {
        console.info(
          `Contract installation cost consumed: ${executionResult?.consumed}`
        );
      }
    }

    const account = await getAccountInfo(RPC_URL, sender.publicKey),
      contractHash = findKeyFromAccountNamedKeys(
        account,
        `cep18_contract_hash_${name}`
      ),
      contractPackageHash = findKeyFromAccountNamedKeys(
        account,
        `cep18_contract_package_${name}`
      );

    console.info(`Contract Hash: ${contractHash}`);
    console.info(`Contract Package Hash: ${contractPackageHash}`);
  })
  .catch(error => {
    console.error(error);
  });
