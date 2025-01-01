import type { NextConfig } from "next";

const nextConfig: NextConfig = {
    output: "standalone",
	webpack: (config, options) => {
		config.experiments = {
			asyncWebAssembly: true,
			syncWebAssembly: true,
			layers: true,
		};
		config.output.webassemblyModuleFilename = (options.isServer ? '../' : '') + 'static/wasm/webassembly.wasm';
		return config;
	}
};

export default nextConfig;
