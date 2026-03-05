#!/bin/sh -eu

# Post process so Rust doesn't sneak any new features into the wasm
# blob that the node doesn't support.

f="$(mktemp)"

wasm-opt \
	--dce \
	--rse \
	--signature-pruning \
	--strip-debug \
	--enable-bulk-memory \
	--strip \
	-Oz \
	"$1" \
	-o $f.wasm1

wasm2wat -o $f.wat $f.wasm1

wat2wasm -o $2 $f.wat
