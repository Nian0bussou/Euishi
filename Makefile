build:
	printf "\e[2J\e[HBuilding...\n\t"
	cargo build
	mv ./target/debug/rustmove ./rustmove

test: 
	printf "\e[2J\e[HTesting...\n\t"
	cargo test
