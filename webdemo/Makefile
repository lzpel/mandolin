build:
	wasm-pack build --target bundler --out-dir frontend/public
tree:
	cargo tree
generate:
	bash -c "cd frontend && npm install"
clean:
	bash -c "cd frontend && npm run cache clean --force"
run:
	bash -c "cd frontend && npm run dev"
crate-next-app:
	npx create-next-app@latest frontend --no-tailwind --no-turbopack --yes