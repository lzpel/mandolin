generate: generate-wasm generate-frontend
	cargo tree && cargo fmt
	mkdir -p frontend/output
	cp -r openapi frontend/output/
generate-frontend:
	cd frontend && npm install
generate-wasm:
	cd frontend/wasm && cargo tree && cargo fmt
	cargo install wasm-pack
	wasm-pack build frontend/wasm -d ../output
test:
	mkdir -p output
	cargo test
run:
	cd frontend && npm run dev
deploy:
	cd frontend && npm run build
clean:
	bash -c "cd frontend && npm cache clean --force"
compile:
	bash -c "cd frontend && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
crate-next-app:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes