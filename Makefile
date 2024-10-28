.PHONY: build rel test rt

build:
	rm -f ./euishi_build
	rm -f ./euishi
	cargo build
	cp ./target/debug/euishi ./euishi_build

rel: 
	cargo build --release
	cp ./target/release/euishi ./euishi

test: 
	cargo test


# rerun test
rt:
	cargo test --bin euishi

# windows
w:
	cargo build --release
	.\target\release\euishi.exe


