clean:
	bash -c "cd frontend && npm cache clean --force"
generate: generate-wasm generate-frontend
	mkdir -p frontend/output
	cp -r openapi frontend/output/
generate-frontend:
	cd frontend && npm install
generate-wasm:
	cargo install wasm-pack
	@: To avoid rustwasm/wasm-bindgen#4211
	@: rustup default 1.81
	wasm-pack build frontend/wasm -d ../output
	@: rustup default stable
run:
	cd frontend && npm run dev
deploy:
	cd frontend && npm run build
tree:
	cargo tree
fmt:
	cargo fmt
compile:
	bash -c "cd frontend && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
crate-next-app:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes