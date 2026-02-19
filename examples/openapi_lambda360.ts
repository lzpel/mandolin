

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type FileExists = {
	exists:boolean,
	expiresAt:string|undefined,
	uploadUrl:string|undefined,
}

type IntersectNode = {
	a:ShapeNode,
	b:ShapeNode,
	op:string,
}

type NumberOrExpr =number |string;

type RotateNode = {
	axis:NumberOrExpr[],
	deg:NumberOrExpr,
	op:string,
	shape:ShapeNode,
}

type ScaleNode = {
	factor:NumberOrExpr,
	op:string,
	shape:ShapeNode,
}

type ShapeNode =StepNode |UnionShapeNode |IntersectNode |SubtractNode |ScaleNode |TranslateNode |RotateNode |StretchNode;

type ShapeNodeBase = {
	op:string,
}

type StepNode = {
	content_hash:string|undefined,
	op:string,
	path:string,
}

type StretchNode = {
	cut:NumberOrExpr[],
	delta:NumberOrExpr[],
	op:string,
	shape:ShapeNode,
}

type SubtractNode = {
	a:ShapeNode,
	b:ShapeNode,
	op:string,
}

type TranslateNode = {
	op:string,
	shape:ShapeNode,
	xyz:NumberOrExpr[],
}

type UnionShapeNode = {
	a:ShapeNode,
	b:ShapeNode,
	op:string,
}


// Request type for helloSayHello
type HelloSayHelloRequest = {
}
// Response type for helloSayHello
type HelloSayHelloResponse =
	| { code: 200; body:string}
// Request type for shapeCompute
type ShapeComputeRequest = {
	body:ShapeNode,
}
// Response type for shapeCompute
type ShapeComputeResponse =
	| { code: 200; body:ArrayBuffer}
// Request type for stepExists
type StepExistsRequest = {
	sha256:string,
}
// Response type for stepExists
type StepExistsResponse =
	| { code: 200; body:FileExists}
// Request type for viewerView
type ViewerViewRequest = {
	sha256:string,
}
// Response type for viewerView
type ViewerViewResponse =
	| { code: 200; body:ArrayBuffer}




/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /hello
	helloSayHello?(request: HelloSayHelloRequest): Promise<HelloSayHelloResponse>
	// POST /shape
	shapeCompute?(request: ShapeComputeRequest): Promise<ShapeComputeResponse>
	// GET /step/{sha256}
	stepExists?(request: StepExistsRequest): Promise<StepExistsResponse>
	// GET /view
	viewerView?(request: ViewerViewRequest): Promise<ViewerViewResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/hello', async (c) => {
		if (implement.helloSayHello===undefined)return c.text("not yet implemented", 500)
		const request: Partial<HelloSayHelloRequest> = {}
		{
		}
		const response = await implement.helloSayHello(request as HelloSayHelloRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/shape', async (c) => {
		if (implement.shapeCompute===undefined)return c.text("not yet implemented", 500)
		const request: Partial<ShapeComputeRequest> = {}
		{
		}
		request.body =(await c.req.json()) as ShapeNode
		const response = await implement.shapeCompute(request as ShapeComputeRequest)
		switch (response.code){
			case 200:
				return c.body(response.body, 200, {'Content-Type': 'model/gltf-binary'})
		}
	})
	app.get('/step/:sha256', async (c) => {
		if (implement.stepExists===undefined)return c.text("not yet implemented", 500)
		const request: Partial<StepExistsRequest> = {}
		{
			let sha256 = c.req.param("sha256")
			if(sha256===undefined)return c.text("required parameter 'sha256' is not in 'path'", 400)
			request.sha256 = sha256;
		}
		const response = await implement.stepExists(request as StepExistsRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
		}
	})
	app.get('/view', async (c) => {
		if (implement.viewerView===undefined)return c.text("not yet implemented", 500)
		const request: Partial<ViewerViewRequest> = {}
		{
			let sha256 = c.req.query("sha256")
			if(sha256===undefined)return c.text("required parameter 'sha256' is not in 'query'", 400)
			request.sha256 = sha256;
		}
		const response = await implement.viewerView(request as ViewerViewRequest)
		switch (response.code){
			case 200:
				return c.body(response.body, 200, {'Content-Type': 'model/gltf-binary'})
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
