generate:
	cargo install wasm-pack
	bash -c "cd demo && npm install && npm run generate"
tree:
	cargo tree
crate-next-app:
	npx create-next-app@latest demo --no-tailwind --no-turbopack --yes