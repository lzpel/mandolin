import * as fs from "node:fs";
import path from "node:path";
import Compare from "@/app/compare";

export default async function Home() {
	const yamlFilePath = path.join(process.cwd(), 'openapi', 'openapi_petstore.yaml');
	const openapi = fs.readFileSync(yamlFilePath, 'utf8');
	return (
		<>
			<h2>mandolin demo</h2>
			<a href="https://github.com/lzpel/mandolin">https://github.com/lzpel/mandolin</a>
			<p>mandolin converts OpenAPI.yaml file into Rust server code. This demo runs with WebAssembly.</p>
			<Compare openapi={openapi}/>
		</>
	)
}