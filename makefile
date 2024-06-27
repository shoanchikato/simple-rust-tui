run:
	RUST_BACKTRACE=1 cargo run
test:
	cargo test
build:
	cargo build
release:
	cargo build --release
fmt:
	cargo fmt
clean:
	cargo clean
clip:
	cargo clippy
