import type { NextConfig } from "next";

const nextConfig: NextConfig = {
	swcMinify: false,
	webpack: (config, options) => {
		config.output.webassemblyModuleFilename =
			options.isServer && !options.dev
				? "../static/wasm/[modulehash].wasm"
				: "static/wasm/[modulehash].wasm";

		config.experiments = {
			...config.experiments,
			asyncWebAssembly: true,
			syncWebAssembly: true,
			layers: true,
		};
		return config;
	}
};

export default nextConfig;
