tests:
	cargo test --all --exclude e2e

e2e:
  cargo test --package e2e

build:
  cargo build --all

release:
  cargo build --verbose --release --all

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

changelog:
  conventional-changelog -p angular -i CHANGELOG.md -s
