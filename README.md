# Casper ERC20

A library and example implementation of ERC20 token for the Casper network.

## Install
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

## Javascript client SDK

A javascript client SDK can be used to interact with the ERC20 contract. It is available in it's own [repository](https://github.com/casper-network/casper-contracts-js-clients/tree/master/packages/erc20-client).
