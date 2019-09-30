all:
	cargo build

run: all
	RUST_BACKTRACE=0 ./target/debug/stork
