# CasperLabs ERC20

Implementation of ERC20 token for the CasperLabs platform.

## Install
Install `wasm32-unknown-unknown` and the Rust toolchain.

```bash
rustup install $(cat rust-toolchain)
rustup target add --toolchain=$(cat rust-toolchain) wasm32-unknown-unknown
```
or 
```bash
$ make prepare
```

## Build Smart Contract
```bash
$ make build-contract
```

## Test
Test logic and smart contract.
```bash
$ make test
```
