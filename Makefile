.PHONY: test clean build

# OpenAPIファイルからRustサーバコードを生成し、全テストを実行する
test:
	cargo test -- --nocapture

# OpenAPIファイルから各言語のコードを生成してout/に出力する
generate: build
	@mkdir -p out
	@for f in openapi/*.yaml openapi/*.json; do \
		[ -f "$$f" ] || continue; \
		case "$$f" in *.tsp) continue;; esac; \
		name=$$(basename "$$f"); \
		echo "=== $$name → rs ==="; \
		cargo run -- -i "$$f" -t RUST_AXUM -o "out/$$name.rs"; \
		echo "=== $$name → ts ==="; \
		cargo run -- -i "$$f" -t TYPESCRIPT_HONO -o "out/$$name.ts"; \
	done
	@echo "=== All generation complete ==="

build:
	cargo build

clean:
	rm -rf out/
	cargo clean
