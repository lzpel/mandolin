

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type Error = {
	code:number,
	message:string,
}

type Widget = {
	color:string,
	id:string,
	weight:number,
}

type WidgetCreate = {
	color:string,
	weight:number,
}

type WidgetUpdate = {
	color:string|undefined,
	weight:number|undefined,
}


// Request type for widgetsList
type WidgetsListRequest = {
}
// Response type for widgetsList
type WidgetsListResponse =
	| { code: 200; body:Widget[]}
	| { code: 500; body:Error}
// Request type for widgetsCreate
type WidgetsCreateRequest = {
	body:WidgetCreate,
}
// Response type for widgetsCreate
type WidgetsCreateResponse =
	| { code: 200; body:Widget}
	| { code: 500; body:Error}
// Request type for widgetsRead
type WidgetsReadRequest = {
	id:string,
}
// Response type for widgetsRead
type WidgetsReadResponse =
	| { code: 200; body:Widget}
	| { code: 500; body:Error}
// Request type for widgetsDelete
type WidgetsDeleteRequest = {
	id:string,
}
// Response type for widgetsDelete
type WidgetsDeleteResponse =
	| { code: 204;}
	| { code: 500; body:Error}
// Request type for widgetsUpdate
type WidgetsUpdateRequest = {
	id:string,
	body:WidgetUpdate,
}
// Response type for widgetsUpdate
type WidgetsUpdateResponse =
	| { code: 200; body:Widget}
	| { code: 500; body:Error}
// Request type for widgetsAnalyze
type WidgetsAnalyzeRequest = {
	id:string,
}
// Response type for widgetsAnalyze
type WidgetsAnalyzeResponse =
	| { code: 200; body:string}
	| { code: 500; body:Error}




/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /widgets
	widgetsList?(request: WidgetsListRequest): Promise<WidgetsListResponse>
	// POST /widgets
	widgetsCreate?(request: WidgetsCreateRequest): Promise<WidgetsCreateResponse>
	// GET /widgets/{id}
	widgetsRead?(request: WidgetsReadRequest): Promise<WidgetsReadResponse>
	// DELETE /widgets/{id}
	widgetsDelete?(request: WidgetsDeleteRequest): Promise<WidgetsDeleteResponse>
	// PATCH /widgets/{id}
	widgetsUpdate?(request: WidgetsUpdateRequest): Promise<WidgetsUpdateResponse>
	// POST /widgets/{id}/analyze
	widgetsAnalyze?(request: WidgetsAnalyzeRequest): Promise<WidgetsAnalyzeResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/widgets', async (c) => {
		if (implement.widgetsList===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsListRequest> = {}
		{
		}
		const response = await implement.widgetsList(request as WidgetsListRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.post('/widgets', async (c) => {
		if (implement.widgetsCreate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsCreateRequest> = {}
		{
		}
		request.body =(await c.req.json()) as WidgetCreate
		const response = await implement.widgetsCreate(request as WidgetsCreateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.get('/widgets/:id', async (c) => {
		if (implement.widgetsRead===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsReadRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.widgetsRead(request as WidgetsReadRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.delete('/widgets/:id', async (c) => {
		if (implement.widgetsDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsDeleteRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.widgetsDelete(request as WidgetsDeleteRequest)
		switch (response.code){
			case 204:
				return c.text("There is no content to send for this request, but the headers may be useful. ")
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.patch('/widgets/:id', async (c) => {
		if (implement.widgetsUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsUpdateRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		request.body =(await c.req.json()) as WidgetUpdate
		const response = await implement.widgetsUpdate(request as WidgetsUpdateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.post('/widgets/:id/analyze', async (c) => {
		if (implement.widgetsAnalyze===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetsAnalyzeRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.widgetsAnalyze(request as WidgetsAnalyzeRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
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
