import * as fs from "node:fs";
import path from "node:path";
import Compare from "@/app/compare";

export default async function Home() {
	const files = readTextFilesInDirectory(path.join(process.cwd(), 'out'))
	return (
		<>
			<h2>mandolin</h2>
			<a href="https://crates.io/crates/mandolin"><img alt="crates.io" src="https://img.shields.io/crates/v/mandolin.svg?style=for-the-badge&logo=rust" height="20"/></a>
			Generate openapi-based server code
			<small>mandolin converts OpenAPI.yaml file into Rust server code. This demo runs with WebAssembly.</small>
			<hr/>
			<Compare files={files}/>
		</>
	)
}
function readTextFilesInDirectory(dirPath: string): { [key: string]: string } {
	const files = fs.readdirSync(dirPath);
	const result: { [key: string]: string } = {};

	files.forEach(file => {
		const filePath = path.join(dirPath, file);
		if (fs.statSync(filePath).isFile() && path.extname(file) === '.yaml') {
			const content = fs.readFileSync(filePath, 'utf-8');
			result[file] = content;
		}
	});

	return result;
}