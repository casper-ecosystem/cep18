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
casper/cep18/target/wasm32-unknown-unknown/release/cep18.wasm
```

## A JavaScript Client SDK

A [JavaScript client SDK](https://github.com/casper-ecosystem/cep18/tree/master/client-js#readme) has been provided to interact with the fungible token contract.

## Tutorials

For more information, visit the links below:

- [Casper Fungible Token Tutorial](/docs/full-tutorial.md) - An illustrated guide on how to implement, deploy, and test an fungible token contract.
- [Casper Fungible Token How-To Guide](/docs/1-quickstart-guide.md) - An example driven quick start guide for launching a CEP-18 token on a Casper network using the Casper CLI client.
- [Exploring the CEP18 Contracts](/docs/2-query.md) - A guide for finding the necessary values to interact with a CEP-18 instance after installation in global state.
- [CEP-18 Token Transfers and Allowances](/docs/3-transfer.md) - A guide to transferring CEP-18 tokens and approving allowances.
