MAKE_RECURSIVE_DIRS := frontend frontend/wasm
generate:
	python markdown_import.py README.md
	cargo tree && cargo fmt
	bash -c "$${MAKE_RECURSIVE}"
run:
	bash -c "$${MAKE_RECURSIVE}"
deploy:
	bash -c "$${MAKE_RECURSIVE}"
test:
	cargo run --example example_axum_generate
	cargo build --example example_axum_generated
	cargo test
	bash -c "$${MAKE_RECURSIVE}"
clean:
	bash -c "$${MAKE_RECURSIVE}"
compile:
	bash -c "cd frontend && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
cli:
	cd mandolin-cli && cargo run -- -h
search-%:
	@git grep --color -r --text -n '$*' .
# 複数のディレクトリそれぞれでmake。環境変数MAKE_RECURSIVE_PARALLELが設定されていたら並列実行 MAKE_RECURSIVE_PARALLEL=1 bash -c "$${MAKE_RECURSIVE}"
define MAKE_RECURSIVE
if [ -n "$${MAKE_RECURSIVE_PARALLEL}" ]; then
	trap 'kill 0' EXIT INT TERM
	time printf '%s\n' $(MAKE_RECURSIVE_DIRS) | xargs -P0 -IX sh -c '$(MAKE) -C X $@'
	wait
else
	time printf '%s\n' $(MAKE_RECURSIVE_DIRS) | xargs -IX sh -c '$(MAKE) -C X $@'
fi
endef
export MAKE_RECURSIVE