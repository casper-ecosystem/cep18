prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p contract --target wasm32-unknown-unknown

test:
	cargo test -p logic
	cargo test -p tests

lint:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints
	cargo fmt

clean:
	cargo clean
