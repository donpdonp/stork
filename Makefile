all:
	cargo build

run: all
	RUST_BACKTRACE=0 ./target/debug/stork

runbt: all
	RUST_BACKTRACE=1 ./target/debug/stork

format: 
	rustfmt src/main.rs
