

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type Error = {
	code:number,
	message:string,
}

type Inner = {
	latitude:number,
	longitude:number,
}

type InnerUpdate = {
	latitude:number|undefined,
	longitude:number|undefined,
}

type Widget = {
	color:string,
	id:string,
	pos:Inner,
	weight:number,
}

type WidgetCollectionWithNextLink = {
	nextLink:string|undefined,
	value:Widget[],
}

type WidgetCreate = {
	color:string,
	pos:Inner,
	weight:number,
}

type WidgetUpdate = {
	color:string|undefined,
	pos:InnerUpdate|undefined,
	weight:number|undefined,
}


// Request type for widgetServiceList
type WidgetServiceListRequest = {
}
// Response type for widgetServiceList
type WidgetServiceListResponse =
	| { code: 200; body:WidgetCollectionWithNextLink}
	| { code: 500; body:Error}
// Request type for widgetServiceCreate
type WidgetServiceCreateRequest = {
	body:WidgetCreate,
}
// Response type for widgetServiceCreate
type WidgetServiceCreateResponse =
	| { code: 200; body:Widget}
	| { code: 201; body:Widget}
	| { code: 500; body:Error}
// Request type for widgetServiceCustomGet
type WidgetServiceCustomGetRequest = {
}
// Response type for widgetServiceCustomGet
type WidgetServiceCustomGetResponse =
	| { code: 200; body:Widget}
// Request type for widgetServiceGet
type WidgetServiceGetRequest = {
	id:string,
}
// Response type for widgetServiceGet
type WidgetServiceGetResponse =
	| { code: 200; body:Widget}
	| { code: 500; body:Error}
// Request type for widgetServiceDelete
type WidgetServiceDeleteRequest = {
	id:string,
}
// Response type for widgetServiceDelete
type WidgetServiceDeleteResponse =
	| { code: 200;}
	| { code: 500; body:Error}
// Request type for widgetServiceUpdate
type WidgetServiceUpdateRequest = {
	id:string,
	body:WidgetUpdate,
}
// Response type for widgetServiceUpdate
type WidgetServiceUpdateResponse =
	| { code: 200; body:Widget}
	| { code: 500; body:Error}




/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /
	widgetServiceList?(request: WidgetServiceListRequest): Promise<WidgetServiceListResponse>
	// POST /
	widgetServiceCreate?(request: WidgetServiceCreateRequest): Promise<WidgetServiceCreateResponse>
	// GET /customGet
	widgetServiceCustomGet?(request: WidgetServiceCustomGetRequest): Promise<WidgetServiceCustomGetResponse>
	// GET /{id}
	widgetServiceGet?(request: WidgetServiceGetRequest): Promise<WidgetServiceGetResponse>
	// DELETE /{id}
	widgetServiceDelete?(request: WidgetServiceDeleteRequest): Promise<WidgetServiceDeleteResponse>
	// PATCH /{id}
	widgetServiceUpdate?(request: WidgetServiceUpdateRequest): Promise<WidgetServiceUpdateResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/', async (c) => {
		if (implement.widgetServiceList===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceListRequest> = {}
		{
		}
		const response = await implement.widgetServiceList(request as WidgetServiceListRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.post('/', async (c) => {
		if (implement.widgetServiceCreate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceCreateRequest> = {}
		{
		}
		request.body =(await c.req.json()) as WidgetCreate
		const response = await implement.widgetServiceCreate(request as WidgetServiceCreateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 201:
				return c.json(response.body, 201)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.get('/customGet', async (c) => {
		if (implement.widgetServiceCustomGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceCustomGetRequest> = {}
		{
		}
		const response = await implement.widgetServiceCustomGet(request as WidgetServiceCustomGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
		}
	})
	app.get('/:id', async (c) => {
		if (implement.widgetServiceGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceGetRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.widgetServiceGet(request as WidgetServiceGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.delete('/:id', async (c) => {
		if (implement.widgetServiceDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceDeleteRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.widgetServiceDelete(request as WidgetServiceDeleteRequest)
		switch (response.code){
			case 200:
				return c.text("Resource deleted successfully.")
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.patch('/:id', async (c) => {
		if (implement.widgetServiceUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<WidgetServiceUpdateRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		request.body =(await c.req.json()) as WidgetUpdate
		const response = await implement.widgetServiceUpdate(request as WidgetServiceUpdateRequest)
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
