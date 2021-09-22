# Casper ERC20

A library for developing ERC20 tokens for the Casper network.

The main functionality is provided via the ERC20 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper network.

## Usage

To create an example ERC20 contract which uses this library, use the `cargo-casper` tool:

```
cargo install cargo-casper
cargo casper --erc20 <PATH TO NEW PROJECT>
```

This command will generate a new project structure with an example token contract based on [example project](example/erc20-token/src/main)

## Development

Make sure the `wasm32-unknown-unknown` Rust target is installed.

```
make prepare
```

## Build Smart Contracts
To build the example ERC20 contract and supporting test contracts:

```
make build-contracts
```

## Test

```
make test
```
