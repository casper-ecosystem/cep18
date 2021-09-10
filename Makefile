ALL_CONTRACTS = erc20-token erc20-test erc20-test-call
CONTRACT_TARGET_DIR = target/wasm32-unknown-unknown/release

prepare:
	rustup target add wasm32-unknown-unknown

build-contract/%:
	cargo build --release -p $* --target wasm32-unknown-unknown
	wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$*).wasm

.PHONY:	build-contracts
build-contracts: build-contract/erc20-token build-contract/erc20-test build-contract/erc20-test-call

.PHONY: test-only

test/%:
	cargo test -p $*

test-all: test/erc20-tests test/tests
	cargo test -p tests -- --ignored

test: build-contracts test-all

clippy:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all
