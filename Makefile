
.DELETE_ON_ERROR:

precompiles-ed25519.wasm: $(shell find precompiles-ed25519 -type f)
	@rm -f superposition_precompiles.wasm
	@cargo build --release --target wasm32-unknown-unknown --bin precompiles-ed25519
	@./wasm-post.sh \
		target/wasm32-unknown-unknown/release/precompiles-ed25519.wasm \
		precompiles-ed25519.wasm
	@./check-codesize.sh precompiles-ed25519.wasm
