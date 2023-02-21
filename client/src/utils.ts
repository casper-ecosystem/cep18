import {
  BigNumber,
  type BigNumberish,
  formatFixed
} from '@ethersproject/bignumber';
import {
  CasperServiceByJsonRPC,
  type CLPublicKey,
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
  await new Promise(resolve => setTimeout(resolve, ms));
};

export const toCSPR = (motes: BigNumberish): number =>
  parseFloat(formatFixed(BigNumber.from(motes), 9));
