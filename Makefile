build: 
	cargo build 
	cp ./target/debug/rustmove ./rustmove

test: 
	cargo test
