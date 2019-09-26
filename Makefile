all:
	cargo build

run: all
	./target/debug/stork
