test:
	cargo test

clean: 
	cargo clean

parse:
	cargo run parse "<DateTime>"

format:
	cargo fmt

clippy:
	cargo clippy

all_commands:
	cargo run help
