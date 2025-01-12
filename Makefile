generate:
	@: コマンドプロンプトでmake generate する可能性を考えたら#からコメントを始めるよりmake nativeな@:を利用した方がいい
	@: To avoid rustwasm/wasm-bindgen#4211
	rustup default 1.81
	@: the scope of change directory is just the line.
	cd demo && npm install
	cargo install wasm-pack
	wasm-pack build demo/wasm -d ../lib
run:
	cd demo && npm run dev
deploy:
	cd demo && npm run build
standalone:
	node demo/.next/standalone/server.js
tree:
	cargo tree
crate-next-app:
	npx create-next-app@latest demo --no-tailwind --no-turbopack --yes