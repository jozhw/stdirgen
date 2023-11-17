lint:
	echo "===> Linting"
	cargo clippy

fmt: 
	echo "===> Formatting"
	cargo fmt

build:
	echo "===> Building"
	cargo build --release

test:
	echo "===> Testing"
	cargo test

pr-ready: lint fmt build test
