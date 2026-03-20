
.DELETE_ON_ERROR:

.PHONY: all clean

all:

precompiles-ed25519.wasm: $(shell find precompiles-ed25519 -type f)
	@cd precompiles-ed25519 && \
		cargo build --release --target wasm32-unknown-unknown --bin precompiles-ed25519
	@./wasm-post.sh \
		precompiles-ed25519/target/wasm32-unknown-unknown/release/precompiles-ed25519.wasm \
		precompiles-ed25519.wasm
	@./check-codesize.sh precompiles-ed25519.wasm

precompiles-muldiv.wasm: $(shell find precompiles-muldiv -type f)
	@cd precompiles-muldiv && \
		cargo build --release --target wasm32-unknown-unknown --bin precompiles-muldiv
	@./wasm-post.sh \
		precompiles-muldiv/target/wasm32-unknown-unknown/release/precompiles-muldiv.wasm \
		precompiles-muldiv.wasm
	@./check-codesize.sh precompiles-muldiv.wasm

precompiles-root.wasm: $(shell find precompiles-root -type f)
	@cd precompiles-root && \
		cargo build --release --target wasm32-unknown-unknown --bin precompiles-root
	@./wasm-post.sh \
		precompiles-root/target/wasm32-unknown-unknown/release/precompiles-root.wasm \
		precompiles-root.wasm
	@./check-codesize.sh precompiles-root.wasm

clean:
	@rm -rf precompiles-ed25519/target precompiles-muldiv/target
