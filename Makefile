
help:
	@cat Makefile

all: test release.native release.wasm

test.dummy:
	@cargo test --no-default-features -- --show-output
test.real:
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
