
makeit:
	cargo build
	mv ./target/debug/rustmove ./rustmove

test: 
	cargo test
