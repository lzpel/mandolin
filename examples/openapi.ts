

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin






// Request type for pathsHelloGet
type PathsHelloGetRequest = {
}
// Response type for pathsHelloGet
type PathsHelloGetResponse =
	| { code: 200; body:PathsHelloGetResponses200ContentApplicationJsonSchema}




type PathsHelloGetResponses200ContentApplicationJsonSchema = {
	message:string|undefined,
}

/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /hello
	pathsHelloGet?(request: PathsHelloGetRequest): Promise<PathsHelloGetResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/hello', async (c) => {
		if (implement.pathsHelloGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<PathsHelloGetRequest> = {}
		{
		}
		const response = await implement.pathsHelloGet(request as PathsHelloGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
		}
	})
}

/// Test server implementation (all methods not yet implemented)
class TestServer implements ApiInterface{}

/// Entry point to start the server
export function main(){
	const app = new Hono()
	addHonoOperations(app,new TestServer());
	serve({
		fetch: app.fetch,
		port: 3000
	}, (info) => {
		console.log(`Server is running on http://localhost:${info.port}`)
	})
}
