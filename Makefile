
help:
	@cat Makefile

all: test release.native release.wasm

test:
	@cargo test
check:
	@cargo rustc -- -Awarnings

release.native:
	@cargo rustc --release -- -C prefer-dynamic
release.wasm:
	@cargo build --release --target wasm32-unknown-unknown


dev.test:
	@cargo test -- --show-output

clean:
	@cargo clean
