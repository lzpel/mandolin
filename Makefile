clean:
	bash -c "cd demo && npm cache clean --force"
generate:
	bash -c "cd demo && npm install"
	# To avloid rustwasm/wasm-bindgen#4211
	rustup default 1.81
	cargo install wasm-pack
	wasm-pack build demo/wasm -d ../lib
	rm -rf demo/openapi
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
compile:
	bash -c "cd demo && find ../openapi/ -name '*.tsp' | xargs -IX npx tsp compile X --emit @typespec/openapi3"
crate-next-app:
	npx create-next-app@latest demo --no-tailwind --no-turbopack --yes