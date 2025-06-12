import {
  Account,
  AccountIdentifier,
  HttpHandler,
  type PublicKey,
  PrivateKey,
  RpcClient
} from 'casper-js-sdk';
import fs from 'fs';
import { SECRET_KEY_ALGO, SECRET_KEY_NAME } from '../config';

export const getAccountInfo = async (
  rpcUrl: string,
  publicKey: PublicKey
): Promise<Account> => {
  const rpcHandler = new HttpHandler(rpcUrl);
  const client = new RpcClient(rpcHandler);

  const accountIdentifier: AccountIdentifier = new AccountIdentifier(
    undefined,
    publicKey
  );
  const { account } = await client.getAccountInfo(null, accountIdentifier);
  if (!account) throw Error('Account not found');
  return account;
};

export const findKeyFromAccountNamedKeys = (
  account: Account,
  name: string
): string => {
  const keysArray = Array.isArray(account.namedKeys)
    ? (account.namedKeys as { name: string; key: { toString(): string } }[])
    : (Object.values(account.namedKeys) as {
        name: string;
        key: { toString(): string };
      }[]);
  const match = keysArray.find(entry => entry.name === name);
  if (!match) {
    console.error(`NamedKey not found: ${name}`);
    return '';
  }
  return match.key.toString();
};

/**
 * Reads a private key from either a file path or a raw string.
 * Ensures the key has the correct PEM format.
 *
 * @param {string} input - File path or raw private key string.
 * @returns {string} - Formatted PEM private key.
 */
export const getPrivateKey = (input: string): string => {
  if (!input) {
    return '';
  }
  let key = input;

  // If the input is a file path, read the content
  if (fs.existsSync(input)) {
    key = fs.readFileSync(input, 'utf8').trim();
  }

  // Check if it's already in PEM format
  if (key && key.includes('BEGIN PRIVATE KEY')) {
    return key;
  }

  // Format as PEM if it's a raw key string
  return `-----BEGIN PRIVATE KEY-----\n${key}\n-----END PRIVATE KEY-----`;
};

/**
 * Retrieves a `PrivateKey` instance from either a file path or a raw PEM key string.
 *
 * - If the input is a PEM string (contains "BEGIN PRIVATE KEY"), it is used directly.
 * - If the input is a file path, the function checks if the path exists:
 *     - If the path ends with `.pem`, it uses that path directly.
 *     - If the path doesn't end with `.pem`, it appends `SECRET_KEY_NAME` to form the full path.
 * - If the input is a directory path (or a file path without `.pem`), it appends `SECRET_KEY_NAME` to form the full path before reading the private key from the file.
 *
 * The function then reads the private key from the file or uses the PEM string directly, and converts it into a `PrivateKey` instance using the `ED25519` algorithm.
 *
 * @param {string} keyPathOrKey - The file path to the private key or a raw PEM private key string.
 * @returns {Promise<PrivateKey>} - A `PrivateKey` instance derived from the provided key or file.
 */
export const getSigningKey = (keyPathOrKey: string): PrivateKey => {
  if (!keyPathOrKey) {
    throw new Error('key Path Or Key in getSigningKey is not set.');
  }
  let signingKeyPem: string;

  // If it's a PEM string, use it directly
  if (keyPathOrKey.includes('BEGIN PRIVATE KEY')) {
    signingKeyPem = keyPathOrKey;
  } else if (fs.existsSync(keyPathOrKey)) {
    // If it's a valid file path, check if it ends with .pem or append SECRET_KEY_NAME if needed
    const formattedPath = keyPathOrKey.endsWith('.pem')
      ? keyPathOrKey
      : `${keyPathOrKey.replace(/\/$/, '')}/${SECRET_KEY_NAME}`;
    signingKeyPem = getPrivateKey(formattedPath);
  } else {
    // If it's not a file path, treat it as a raw PEM key
    signingKeyPem = getPrivateKey(keyPathOrKey);
  }

  // Convert the PEM string into a PrivateKey instance
  return PrivateKey.fromPem(signingKeyPem, SECRET_KEY_ALGO) as PrivateKey;
};
