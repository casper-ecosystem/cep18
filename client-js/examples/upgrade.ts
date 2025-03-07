import {
  CEP18Client,
  ContractWASM as wasm,
  EVENTS_MODE,
  type TransactionParams,
  type TransactionResult,
  type UpgradeArgs
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

const name = 'TEST_CEP18',
  eventsMode = EVENTS_MODE.NoEvents,
  waitForTransactionProcessed = true,
  sender = getSigningKey(FAUCET_PRIVATE_KEY),
  paymentAmount = String(350_000_000_000);

const upgrade = async () => {
  const cep18 = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME),
    params: TransactionParams = {
      wasm,
      sender: sender.publicKey,
      paymentAmount,
      signingKeys: [sender]
    },
    args: UpgradeArgs = {
      name,
      eventsMode
    },
    transactionResult: TransactionResult = await cep18.upgrade({
      params,
      args,
      waitForTransactionProcessed
    });

  if (!transactionResult.transactionInfo.transactionHash) {
    throw Error('Invalid transaction hash');
  }
  return transactionResult;
};

upgrade()
  .then(async transactionResult => {
    const { transactionInfo, executionResult } = transactionResult;
    console.info(
      `Contract upgrade transaction hash: ${transactionInfo.transactionHash}`
    );

    if (executionResult) {
      if (executionResult?.errorMessage) {
        throw new Error(
          `Error during upgrade.\n${executionResult?.errorMessage.toString()}`
        );
      } else {
        console.info(
          `Contract upgrade cost consumed: ${executionResult?.consumed}`
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
    console.error('Error:', error);
  });
