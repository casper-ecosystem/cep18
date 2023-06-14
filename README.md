# Casper Fungible Tokens (CEP-18 Standard)

A library for developing fungible tokens (CEP-18 Tokens) tokens for the Casper Network.

The main functionality is provided via the CEP-18 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

## Development

Make sure the `wasm32-unknown-unknown` Rust target is installed.

```
make prepare
```

## Build Smart Contracts

To build the example fungible token contract and supporting test contracts:

```
make build-contracts
```

## Locating Smart Contract Wasm

The Wasm for your new project can be found in the following local directory:

```
casper/cep18/target/wasm32-unknown-unknown/release/cep18_token.wasm
```

## Test

```
make test
```

## JavaScript Client SDK

A [JavaScript client SDK](https://github.com/casper-ecosystem/erc20/tree/master/client-js#readme) can be used to interact with the fungible token contract.

## Documentation

For more information, visit the below guides:

- [Casper Fungible Token Tutorial](/docs/full-tutorial.md) - An illustrated guide on how to implement, deploy, and test an fungible token contract.
- [Casper Fungible Token How-To Guide](/docs/1-quickstart-guide.md) - An example driven quick start guide for launching a CEP-18 token on a Casper network using the Casper CLI client.
- [Exploring the CEP18 Contracts](/docs/2-query.md) - A guide for finding the necessary values to interact with a CEP-18 instance after installation in global state.
- [CEP-18 Token Transfers and Allowances](/docs/3-transfer.md) - A guide to transferring CEP-18 tokens and approving allowances.
