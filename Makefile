generate:
	# To avoid rustwasm/wasm-bindgen#4211
	rustup default 1.81
	bash -c "cd demo && npm install"
	cargo install wasm-pack
	wasm-pack build demo/wasm -d ../lib
	cp -r openapi demo/
	echo '*' > demo/openapi/.gitignore
run:
	bash -c "cd demo && npm run dev"
deploy:
	bash -c "cd demo && npm run build"
standalone:
	node demo/.next/standalone/server.js
tree:
	cargo tree
crate-next-app:
	npx create-next-app@latest demo --no-tailwind --no-turbopack --yes