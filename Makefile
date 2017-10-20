build:
	cargo build --manifest-path cli/Cargo.toml

run:
	cargo run --manifest-path cli/Cargo.toml

all: build
