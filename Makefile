PINNED_TOOLCHAIN := $(shell cat rust-toolchain)
WASM_TARGET_DIR := ./target/wasm32-unknown-unknown/release
WASM_OUTPUT_DIR := tests/wasm
WASM_FILES := cep18.wasm cep18_test_contract.wasm
RUSTFLAGS := -C target-cpu=mvp
CARGO_BUILD_FLAGS := -Z build-std=std,panic_abort

prepare:
	rustup install $(PINNED_TOOLCHAIN)
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain $(PINNED_TOOLCHAIN)
	rustup component add rustfmt --toolchain $(PINNED_TOOLCHAIN)
	rustup component add rust-src --toolchain $(PINNED_TOOLCHAIN)

.PHONY: build-contract
build-contract:
	RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -p cep18
	wasm-strip $(WASM_TARGET_DIR)/$(word 1, $(WASM_FILES))

.PHONY: build-all-contracts
build-all-contracts: build-contract
	RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -p cep18-test-contract
	wasm-strip $(WASM_TARGET_DIR)/$(word 2, $(WASM_FILES))

.PHONY: setup-test
setup-test: build-all-contracts copy-wasm

.PHONY: copy-wasm
copy-wasm:
	mkdir -p $(WASM_OUTPUT_DIR)
	cp $(addprefix $(WASM_TARGET_DIR)/, $(WASM_FILES)) $(WASM_OUTPUT_DIR)

native-test: setup-test
	cargo test -p tests --lib should_transfer_account_to_account

test: setup-test
	cargo test -p tests --lib

clippy:
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep18 --bins --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep18 --lib --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep18 --lib --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) --no-default-features -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy -p cep18-test-contract --bins --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo +stable clippy -p tests --all-targets -- -D warnings

format:
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep18
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep18-test-contract
	cargo +stable fmt -p tests

check-lint: clippy
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep18
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep18-test-contract
	cargo +stable fmt -p tests -- --check

lint: clippy format

clean:
	cargo clean -p cep18
	cargo clean -p cep18-test-contract
	cargo clean -p tests
	rm -rf $(WASM_OUTPUT_DIR)
	rm -rf ./Cargo.lock

.PHONY: cargo-update
cargo-update:
	cargo update -p cep18
	cargo update -p cep18-test-contract
	cargo update -p tests
