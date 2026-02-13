
## solution: downgrade rustc 1.81(temporary solution)

Ref: rustwasm/wasm-bindgen#4211

```makefile
generate:
	# To avoid rustwasm/wasm-bindgen#4211
	rustup default 1.81
```

## problem

> next build

▲ Next.js 15.1.2

Creating an optimized production build ...
<w> [webpack.cache.PackFileCacheStrategy] Skipped not serializable cache item 'Compilation/modules|webassembly/async|/home/lzpel/mandolin/demo/lib/wasm_bg.wasm|ssr': No serializer registered for CompileError
<w> while serializing webpack/lib/cache/PackFileCacheStrategy.PackContentItems -> webpack/lib/NormalModule -> webpack/lib/ModuleParseError -> CompileError
Failed to compile.

./lib/wasm_bg.wasm
Module parse failed: Internal failure: parseVec could not cast the value
You may need an appropriate loader to handle this file type, currently no loaders are configured to process this file. See https://webpack.js.org/concepts#loaders
Error: Internal failure: parseVec could not cast the value
at parseVec (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:1:119284)
at parseTypeSection (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:1:119694)
at parseSection (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:1:134162)
at Object.decode (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:1:139551)
at decode (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:1:144834)
at WebAssemblyParser.parse (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:28:1473078)
at /home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:28:410486
at processResult (/home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:28:406448)
at /home/lzpel/mandolin/demo/node_modules/next/dist/compiled/webpack/bundle5.js:28:407465
at /home/lzpel/mandolin/demo/node_modules/next/dist/compiled/loader-runner/LoaderRunner.js:1:8727

Import trace for requested module:
./lib/wasm_bg.wasm
./lib/wasm.js
./app/compare.tsx


> Build failed because of webpack errors
make: *** [Makefile:12: deploy] Error 1
