OS_TYPE := $(shell uname -s)
export RUST_LOG=info

build:
	cargo build

run:
	cargo run

lint:
	cargo clippy

doc:
	cargo doc --document-private-items
