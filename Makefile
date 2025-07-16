export MAKEDIRS := frontend frontend/wasm
SHELL := bash
create:
	bash -c "$${create}"
generate:
	cargo tree && cargo fmt
	bash -c "$${make_dirs}"
test:
	@: openapiから生成できることのテスト
	cargo test
	@: cliのテスト
	@: cd mandolin-cli && find ../openapi -name '*.yaml' | xargs -IZ bash -c "cargo run -- -i Z -t RUST_AXUM > out.rs;cat Z | cargo run -- -o out.rs -t RUST_AXUM;"
	bash -c "$${make_dirs}"
run:
	bash -c "$${make_dirs}"
deploy:
	bash -c "$${make_dirs}"
clean:
	bash -c "$${make_dirs}"
compile:
	bash -c "$${markdown_import}"
	bash -c "cd frontend && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
cli:
	cd mandolin-cli && cargo run -- -h
search-%:
	@git grep --color -r --text -n '$*' .
define make_dirs
if [ -n "$$parallel" ]; then
	trap "kill 0" EXIT
	for dir in $$MAKEDIRS; do
		$(MAKE) -C $$dir $@ & done
	wait
else
	time echo $$MAKEDIRS | xargs -n 1 | xargs -IX sh -c "$(MAKE) -C X $@ || exit 255"
fi
endef
export make_dirs
define create
install -D /dev/stdin ./frontend/Makefile <<'EOF'
%:
	@echo "Unknown target '$$@' skipping"
create:
	npx create-next-app@latest . --ts --eslint --app --use-npm --yes
generate:
	npm install
	cp -rf ../openapi out
run:
	npm run dev
deploy:
	npm run build
clean:
	npm cache clean --force
EOF
install -D /dev/stdin ./frontend/wasm/Makefile <<'EOF'
%:
	@echo "Unknown target '$$@' skipping"
generate:
	cargo tree && cargo fmt
	cargo install wasm-pack
	wasm-pack build . -d ../output
EOF
endef
export create
define markdown_import
sed -i -n 's/^```\(.*\)/```\1/p' README.md
endef
export markdown_import