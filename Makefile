build:
	cargo build --manifest-path cli/Cargo.toml

run:
	export RUST_BACKTRACE=1; cargo run --manifest-path cli/Cargo.toml

all: build
