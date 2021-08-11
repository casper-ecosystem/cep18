prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc20 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20-token.wasm 2>/dev/null | true

test-only:
	cargo test -p erc20-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm erc20-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf erc20-tests/wasm/*.wasm
