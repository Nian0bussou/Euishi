.PHONY: build rel test

build:
	rm -f ./euishi_build
	cargo build
	cp ./target/debug/euishi ./euishi_build

rel:
	rm -f ./euishi
	cargo build --release
	cp ./target/release/euishi ./euishi

test:
	cargo test
