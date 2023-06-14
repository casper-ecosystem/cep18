import { Keys } from 'casper-js-sdk';
import { config } from 'dotenv';

config();

const { MASTER_KEY_PAIR_PATH, USER1_KEY_PAIR_PATH, USER2_KEY_PAIR_PATH } =
  process.env;

export const FAUCET_KEY = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

export const USER1_KEY = Keys.Ed25519.parseKeyFiles(
  `${USER1_KEY_PAIR_PATH}/public_key.pem`,
  `${USER1_KEY_PAIR_PATH}/secret_key.pem`
);

export const USER2_KEY = Keys.Ed25519.parseKeyFiles(
  `${USER2_KEY_PAIR_PATH}/public_key.pem`,
  `${USER2_KEY_PAIR_PATH}/secret_key.pem`
);

export const NODE_URL = process.env.NODE_URL || 'http://localhost:11101/rpc';
export const EVENT_STREAM_ADDRESS =
  process.env.EVENT_STREAM_ADDRESS || 'http://localhost:18101/events/main';

export const NETWORK_NAME = process.env.NETWORK_NAME || 'casper-net-1';

export const DEPLOY_TIMEOUT = parseInt(
  process.env.DEPLOY_TIMEOUT || '1200000',
  10
);
