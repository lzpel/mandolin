create:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes
generate: generate-wasm generate-frontend
	echo '*' | install -D /dev/stdin ./out/.gitignore
	cargo tree && cargo fmt
generate-frontend:
	cd frontend && npm install
	o=frontend/output ; mkdir -p $$o && cp -r openapi $$o
generate-wasm:
	cd frontend/wasm && cargo tree && cargo fmt
	cargo install wasm-pack
	wasm-pack build frontend/wasm -d ../output
test:
	@: openapiから生成できることのテスト
	cargo test
	@: cliのテスト
	@: cd mandolin-cli && find ../openapi -name '*.yaml' | xargs -IZ bash -c "cargo run -- -i Z -t RUST_AXUM > out.rs;cat Z | cargo run -- -o out.rs -t RUST_AXUM;"
	@: フロントエンドビルドのテスト
	cd frontend && npm run build
run:
	cd frontend && npm run dev
deploy:

clean:
	bash -c "cd frontend && npm cache clean --force"
compile:
	bash -c "cd frontend && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
cli:
	cd mandolin-cli && cargo run -- -h
search-%:
	@git grep --color -r --text -n '$*' .