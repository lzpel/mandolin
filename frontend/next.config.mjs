// next.config.tsはworkflowが読み込めない。mjsが回避策
const nextConfig = {
	output: 'export', // これは動的なサイトを生成してしまうので間違いでexportが正解。そしてmjsにするならばexportも書かなくてよい
	basePath: process.env.NEXT_PUBLIC_REPO ? `/${process.env.NEXT_PUBLIC_REPO}` : undefined,
	assetPrefix: process.env.NEXT_PUBLIC_REPO ? `/${process.env.NEXT_PUBLIC_REPO}/` : undefined,
	env: {
		NEXT_PUBLIC_PREFIX: process.env.NEXT_PUBLIC_REPO ? `/${process.env.NEXT_PUBLIC_REPO}` : "",
	},
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