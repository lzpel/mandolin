generate:
	cargo install wasm-pack
	bash -c "cd frontend && npm install"
	wasm-pack build  --target web --out-dir frontend/lib
tree:
	cargo tree
clean:
	bash -c "cd frontend && npm run cache clean --force"
run:
	bash -c "cd frontend && npm run dev"
build:
	bash -c "cd frontend && npm run build"
crate-next-app:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes