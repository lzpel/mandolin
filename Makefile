.PHONY: test clean build


# OpenAPIファイルからRustサーバコードを生成し、全テストを実行する
test:
	cargo test -- --nocapture
	bash -c "$${MAKE_RECURSIVE}"

# OpenAPIファイルから各言語のコードを生成してout/に出力する
generate:
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
	bash -c "$${MAKE_RECURSIVE}"

run:
	bash -c "$${MAKE_RECURSIVE}"

deploy: generate
	bash -c "$${MAKE_RECURSIVE}"

clean:
	rm -rf out/
	cargo clean
	bash -c "$${MAKE_RECURSIVE}"

MAKE_RECURSIVE_DIRS := frontend frontend/wasm

# 複数のディレクトリそれぞれでmake。環境変数MAKE_RECURSIVE_PARALLELが設定されていたら並列実行 MAKE_RECURSIVE_PARALLEL=1 bash -c "$${MAKE_RECURSIVE}"
define MAKE_RECURSIVE
time printf '%s\n' $(MAKE_RECURSIVE_DIRS) | xargs -IX sh -c '$(MAKE) -C X $@'
endef
export
