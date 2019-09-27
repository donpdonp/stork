all:
	cargo build

run: all
	RUST_BACKTRACE=1 ./target/debug/stork
