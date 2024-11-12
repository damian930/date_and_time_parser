test:
	cargo test

clean: 
	cargo clean

parse:
	cargo run "<DateTime>"

format:
	cargo format

clippy:
	cargo clippy -- -Dwarnings

all_commands:
	cargo run help