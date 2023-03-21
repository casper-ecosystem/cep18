import { Keys } from 'casper-js-sdk';
// eslint-disable-next-line import/no-extraneous-dependencies
import { config } from 'dotenv';

config();

export const NODE_URL = process.env.NODE_URL || 'http://localhost:11101/rpc';
export const EVENT_STREAM_ADDRESS =
  process.env.EVENT_STREAM_ADDRESS || 'http://localhost:18101/events/main';

export const DEPLOY_TIMEOUT = parseInt(
  process.env.DEPLOY_TIMEOUT || '1200000',
  10
);

export enum AVAILABLE_NETWORKS {
  NCTL = 'casper-net-1',
  TESTNET = 'casper-net',
  MAINNET = 'casper'
}

export type AVALIABLE_NETWORKS_TYPE = keyof typeof AVAILABLE_NETWORKS;

export const NETWORK_NAME = process.env.NETWORK_NAME || AVAILABLE_NETWORKS.NCTL;

export const users = [
  'MC4CAQAwBQYDK2VwBCIEIHRZr1HEgKVbgchuatwA7dCWDWB7QZe+bpDb5dguIyLE',
  'MC4CAQAwBQYDK2VwBCIEIJ3WEDyVs7vJpLbBtrsqSeOBAZaX9q0lCiGKYtGzqXgF',
  'MC4CAQAwBQYDK2VwBCIEIMqlVjSlScu3qCZcUevH6G5GjOO+sVwcqAE3c3mMtIJS',
  'MC4CAQAwBQYDK2VwBCIEIKEXtOWiDAzLLxByEUW3vVQTpV3/K+xCT/uYsdef1XaM'
].map(key => Keys.getKeysFromHexPrivKey(key, Keys.SignatureAlgorithm.Ed25519));
