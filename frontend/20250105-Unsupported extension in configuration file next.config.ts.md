Run actions/configure-pages@v5
Warning: Unsupported extension in configuration file: demo/next.config.ts. Currently supported extensions: ".js", ".cjs", ".mjs". We will still attempt to inject the site metadata into the configuration file, but it may not work as expected.
Injecting property=output and value=export in:
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
output: "standalone",
webpack: (config, options) => {
config.experiments = {
...config.experiments,
asyncWebAssembly: true,
syncWebAssembly: true,
layers: true,
};
config.output.webassemblyModuleFilename = (options.isServer ? '../' : '') + 'static/wasm/webassembly.wasm';
return config;
}
};

export default nextConfig;

Warning: We were unable to determine how to inject the site metadata into your config. Generated URLs may be incorrect. The base URL for this site should be https://lzpel.github.io/mandolin/. Please ensure your framework is configured to generate relative links appropriately. Error: Unexpected token {