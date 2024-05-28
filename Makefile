PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rust-src --toolchain ${PINNED_TOOLCHAIN}

.PHONY:	build-contract
build-contract:
	RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort -p cep18
	RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort -p cep18-test-contract
	wasm-strip target/wasm32-unknown-unknown/release/cep18.wasm
	wasm-strip target/wasm32-unknown-unknown/release/cep18_test_contract.wasm

setup-test: build-contract
	mkdir -p tests/wasm
	cp ./target/wasm32-unknown-unknown/release/cep18.wasm tests/wasm
	cp ./target/wasm32-unknown-unknown/release/cep18_test_contract.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd cep18 && cargo clippy --all-targets -- -D warnings
	cd cep18-test-contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd cep18 && cargo fmt -- --check
	cd cep18-test-contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd cep18 && cargo fmt
	cd cep18-test-contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd cep18 && cargo clean
	cd cep18-test-contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
