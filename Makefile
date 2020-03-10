build-contract:
	cargo build --release -p contract

test:
	cargo test -p erc20-logic
	cargo test -p tests

lint:
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints
