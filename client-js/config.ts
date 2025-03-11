import { KeyAlgorithm } from 'casper-js-sdk';
import { config } from 'dotenv';

config({ path: './.env' });

const DEFAULT_RPC_URL = 'http://localhost:11101/rpc';
const DEFAULT_SSE_URL = 'http://localhost:18101/events';
const DEFAULT_CHAIN_NAME = 'casper-net-1';
const DEFAULT_PRIVATE_KEY_NAME = 'secret_key.pem';
const DEFAULT_PRIVATE_KEY_ALGO: KeyAlgorithm = KeyAlgorithm.ED25519;

// Path or base64 value or full .pem key as a string
export const { PRIVATE_KEY_FAUCET, PRIVATE_KEY_USER_1, PRIVATE_KEY_USER_2 } =
  process.env;

export const SECRET_KEY_NAME: string =
  process.env.SECRET_KEY_NAME || DEFAULT_PRIVATE_KEY_NAME;
export const SECRET_KEY_ALGO: KeyAlgorithm =
  process.env.PRIVATE_KEY_FAUCET === 'SECP256K1'
    ? KeyAlgorithm.SECP256K1
    : DEFAULT_PRIVATE_KEY_ALGO;

export const RPC_URL = process.env.RPC_URL || DEFAULT_RPC_URL;
export const SSE_URL = process.env.SSE_URL || DEFAULT_SSE_URL;
export const CHAIN_NAME = process.env.CHAIN_NAME || DEFAULT_CHAIN_NAME;
