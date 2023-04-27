# Casper Fungible Tokens (ERC-20 Standard)

A library for developing fungible tokens (ERC-20 Tokens) tokens for the Casper Network.

The main functionality is provided via the ERC-20 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

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

A [JavaScript client SDK](https://github.com/casper-network/casper-contracts-js-clients/tree/master/packages/cep18-client) can be used to interact with the fungible token contract. 


## Documentation

For more information, visit the below guides:
- [Casper Fungible Token Tutorial](/docs/TUTORIAL.md) - An illustrated guide on how to implement, deploy, and test an fungible token contract. 
- [Casper Fungible Token How-To Guide](/docs/Sample-Guide.md) - An example-driven guide on how to setup, query, transfer, approve, and check the balance of an fungible token contract.
- [Casper Fungible Token Quick Deploying Guide](/docs/Deploy-Token.md) - A quick guide on how to deploy the Casper fungible token to the Casper Network.


