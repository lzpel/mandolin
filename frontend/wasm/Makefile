%:
	@echo "Unknown target '$@' skipping"
generate:
	cargo tree && cargo fmt
	cargo install wasm-pack
	wasm-pack build . -d ../output
