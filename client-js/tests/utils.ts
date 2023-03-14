import {
  CasperServiceByJsonRPC,
  type CLPublicKey,
  type GetDeployResult,
  type StoredValue
} from 'casper-js-sdk';

type Defined<Value> = Exclude<Value, null | undefined>;
type Account = Defined<StoredValue['Account']>;

export const getAccountInfo = async (
  nodeAddress: string,
  publicKey: CLPublicKey
): Promise<Account> => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const stateRootHash = await client.getStateRootHash();
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  const account = blockState.Account;
  if (!account) throw Error('Not found account');
  return account;
};

export const findKeyFromAccountNamedKeys = (
  account: Account,
  name: string
): string => {
  const key = account.namedKeys.find(namedKey => namedKey.name === name)?.key;

  if (!key) throw Error(`Not found key: ${name}`);

  return key;
};

export const sleep = async (ms: number): Promise<void> => {
  // eslint-disable-next-line no-promise-executor-return
  await new Promise(resolve => setTimeout(resolve, ms));
};

export const expectDeployResultToSuccess = (result: GetDeployResult): void => {
  expect(result.execution_results[0].result.Failure).toBeUndefined();
  expect(result.execution_results[0].result.Success).toBeDefined();
};
