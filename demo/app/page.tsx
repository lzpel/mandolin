import * as fs from "node:fs";
import path from "node:path";
import Compare from "@/app/compare";
import {Metadata} from "next";

export default async function Home() {
	const yamlFilePath = path.join(process.cwd(), 'openapi', 'openapi_petstore.yaml');
	const openapi = fs.readFileSync(yamlFilePath, 'utf8');
	return (
		<>
			<h2>mandolin demo</h2>
			<Compare openapi={openapi}/>
		</>
	)
}