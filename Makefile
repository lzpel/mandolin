generate:
	cargo install wasm-pack
	bash -c "cd demo && npm install && npm run generate"
run:
	bash -c "cd demo && npm run dev"
deploy:
	node -v
	npm -v
	bash -c "cd demo && npm run build"
tree:
	cargo tree
crate-next-app:
	npx create-next-app@latest demo --no-tailwind --no-turbopack --yes