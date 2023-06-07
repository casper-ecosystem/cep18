# Casper Fungible Tokens (CEP-18 Standard)

This repository contains a reference contract implementation and tests for fungible tokens on a Casper network, following the [CEP-18 standard](https://github.com/casper-network/ceps/pull/18).

## Preparation

Install the `wasm32-unknown-unknown` Rust target with the following command.

```
make prepare
```

## Building and Testing the Contract

To build the reference fungible token contract and supporting tests, run this command:

```
make test
```

## Locating the Contract Wasm

Find the Wasm for the contract in the following directory:

```
casper/cep18/target/wasm32-unknown-unknown/release/cep18_token.wasm
```

## A JavaScript Client SDK

A [JavaScript client SDK](https://github.com/casper-ecosystem/cep18/tree/master/client-js#readme) has been provided to interact with the fungible token contract.

## Tutorials

For more information, visit the links below:

- [The Casper Fungible Token Developer Tutorial](/docs/TUTORIAL.md) - How to implement, deploy, and test a fungible token contract.
- [The Casper Fungible Token Usage Guide](/docs/Sample-Guide.md) - An example-driven guide on how to set up, query, transfer, approve, and check the balance of a fungible token contract.
- [The Casper Fungible Token Quick Deploying Guide](/docs/Deploy-Token.md) - How to deploy the Casper fungible token to a Casper network.
