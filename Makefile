build:
	cargo build
	mv ./target/debug/euishi ./euishi

test: 
	cargo test
