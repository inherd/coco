tests:
	cargo test --all

build:
    cargo build

@bench:
	cargo bench

@lint:
	rustup component add clippy
	rustup component add rustfmt
	cargo clippy -- -D warnings
	cargo clippy --tests
	cargo fmt -- --check

@fix:
    cargo fmt --all

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;
