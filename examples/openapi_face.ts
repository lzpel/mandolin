

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type Device = {
	memory_size_megabytes:number,
	memory_used_megabytes:number,
	name:string,
}

type Job = {
	id:Uuid,
	id_root:Uuid,
	name:string,
	status:Task[],
}

type JobRequest = {
	archive:string,
	image:string,
	json:string,
}

type Task = {
	code:number,
	path:string[],
	target:string,
	time_end:number,
	time_run:number,
}

type Uuid=string;

type User = {
	auth_email:string,
	auth_email_password:string,
	id:Uuid,
	name:string,
	picture:string,
}


// Request type for authApiUserGet
type AuthApiUserGetRequest = {
}
// Response type for authApiUserGet
type AuthApiUserGetResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 403;}
	| { code: 404;}
// Request type for authApiOut
type AuthApiOutRequest = {
}
// Response type for authApiOut
type AuthApiOutResponse =
	| { code: 204;}
// Request type for authApiSignin
type AuthApiSigninRequest = {
	body:PathsAuthSigninPostRequestBodyContentApplicationJsonSchema,
}
// Response type for authApiSignin
type AuthApiSigninResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 404;}
// Request type for authApiSignup
type AuthApiSignupRequest = {
	body:PathsAuthSignupPostRequestBodyContentApplicationJsonSchema,
}
// Response type for authApiSignup
type AuthApiSignupResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 403;}
// Request type for authApiUpdate
type AuthApiUpdateRequest = {
	body:PathsAuthUpdatePostRequestBodyContentApplicationJsonSchema,
}
// Response type for authApiUpdate
type AuthApiUpdateResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 403;}
// Request type for backgroundCron
type BackgroundCronRequest = {
}
// Response type for backgroundCron
type BackgroundCronResponse =
	| { code: 204;}
// Request type for imagesList
type ImagesListRequest = {
}
// Response type for imagesList
type ImagesListResponse =
	| { code: 200; body:string[]}
	| { code: 400; body:string}
// Request type for jobsList
type JobsListRequest = {
}
// Response type for jobsList
type JobsListResponse =
	| { code: 200; body:Job[]}
	| { code: 400; body:string}
// Request type for jobsPush
type JobsPushRequest = {
	body:PathsJobPostRequestBodyContentMultipartFormDataSchema,
}
// Response type for jobsPush
type JobsPushResponse =
	| { code: 200; body:Job}
	| { code: 400; body:string}
// Request type for jobsDelete
type JobsDeleteRequest = {
	id:Uuid,
}
// Response type for jobsDelete
type JobsDeleteResponse =
	| { code: 204;}
	| { code: 400; body:string}
	| { code: 404;}
// Request type for jobsFileCat
type JobsFileCatRequest = {
	id:Uuid,
	path:string,
	limit:number|undefined,
}
// Response type for jobsFileCat
type JobsFileCatResponse =
	| { code: 200; body:ArrayBuffer}
	| { code: 400; body:string}
	| { code: 404;}
// Request type for jobsFileLs
type JobsFileLsRequest = {
	id:Uuid,
	path:string|undefined,
}
// Response type for jobsFileLs
type JobsFileLsResponse =
	| { code: 200; body:string[]}
	| { code: 400; body:string}
	| { code: 404;}
// Request type for jobsTasklist
type JobsTasklistRequest = {
	id:Uuid,
}
// Response type for jobsTasklist
type JobsTasklistResponse =
	| { code: 200; body:Task[]}
	| { code: 400; body:string}
	| { code: 404;}
// Request type for statusInterfaceStatus
type StatusInterfaceStatusRequest = {
}
// Response type for statusInterfaceStatus
type StatusInterfaceStatusResponse =
	| { code: 200; body:Device[]}
	| { code: 400; body:string}




type PathsAuthSignupPostRequestBodyContentApplicationJsonSchema = {
	email:string,
	name:string,
	password:string,
}

type PathsAuthSigninPostRequestBodyContentApplicationJsonSchema = {
	email:string,
	password:string,
}

type PathsJobPostRequestBodyContentMultipartFormDataSchema = {
	config:string|undefined,
	files:ArrayBuffer[],
	image:string,
	paths:string[],
}

type PathsAuthUpdatePostRequestBodyContentApplicationJsonSchema = {
	name:string,
	password:string,
	password_new:string,
}

/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /auth
	authApiUserGet?(request: AuthApiUserGetRequest): Promise<AuthApiUserGetResponse>
	// GET /auth/out
	authApiOut?(request: AuthApiOutRequest): Promise<AuthApiOutResponse>
	// POST /auth/signin
	authApiSignin?(request: AuthApiSigninRequest): Promise<AuthApiSigninResponse>
	// POST /auth/signup
	authApiSignup?(request: AuthApiSignupRequest): Promise<AuthApiSignupResponse>
	// POST /auth/update
	authApiUpdate?(request: AuthApiUpdateRequest): Promise<AuthApiUpdateResponse>
	// GET /cron
	backgroundCron?(request: BackgroundCronRequest): Promise<BackgroundCronResponse>
	// GET /image
	imagesList?(request: ImagesListRequest): Promise<ImagesListResponse>
	// GET /job
	jobsList?(request: JobsListRequest): Promise<JobsListResponse>
	// POST /job
	jobsPush?(request: JobsPushRequest): Promise<JobsPushResponse>
	// DELETE /job/{id}
	jobsDelete?(request: JobsDeleteRequest): Promise<JobsDeleteResponse>
	// GET /job/{id}/cat
	jobsFileCat?(request: JobsFileCatRequest): Promise<JobsFileCatResponse>
	// GET /job/{id}/ls
	jobsFileLs?(request: JobsFileLsRequest): Promise<JobsFileLsResponse>
	// GET /job/{id}/task
	jobsTasklist?(request: JobsTasklistRequest): Promise<JobsTasklistResponse>
	// GET /status
	statusInterfaceStatus?(request: StatusInterfaceStatusRequest): Promise<StatusInterfaceStatusResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/auth', async (c) => {
		if (implement.authApiUserGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiUserGetRequest> = {}
		{
		}
		const response = await implement.authApiUserGet(request as AuthApiUserGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.text("Access is forbidden.")
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.get('/auth/out', async (c) => {
		if (implement.authApiOut===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiOutRequest> = {}
		{
		}
		const response = await implement.authApiOut(request as AuthApiOutRequest)
		switch (response.code){
			case 204:
				return c.text("There is no content to send for this request, but the headers may be useful. ")
		}
	})
	app.post('/auth/signin', async (c) => {
		if (implement.authApiSignin===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiSigninRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsAuthSigninPostRequestBodyContentApplicationJsonSchema
		const response = await implement.authApiSignin(request as AuthApiSigninRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.post('/auth/signup', async (c) => {
		if (implement.authApiSignup===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiSignupRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsAuthSignupPostRequestBodyContentApplicationJsonSchema
		const response = await implement.authApiSignup(request as AuthApiSignupRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.text("Access is forbidden.")
		}
	})
	app.post('/auth/update', async (c) => {
		if (implement.authApiUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiUpdateRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsAuthUpdatePostRequestBodyContentApplicationJsonSchema
		const response = await implement.authApiUpdate(request as AuthApiUpdateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.text("Access is forbidden.")
		}
	})
	app.get('/cron', async (c) => {
		if (implement.backgroundCron===undefined)return c.text("not yet implemented", 500)
		const request: Partial<BackgroundCronRequest> = {}
		{
		}
		const response = await implement.backgroundCron(request as BackgroundCronRequest)
		switch (response.code){
			case 204:
				return c.text("There is no content to send for this request, but the headers may be useful. ")
		}
	})
	app.get('/image', async (c) => {
		if (implement.imagesList===undefined)return c.text("not yet implemented", 500)
		const request: Partial<ImagesListRequest> = {}
		{
		}
		const response = await implement.imagesList(request as ImagesListRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/job', async (c) => {
		if (implement.jobsList===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsListRequest> = {}
		{
		}
		const response = await implement.jobsList(request as JobsListRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/job', async (c) => {
		if (implement.jobsPush===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsPushRequest> = {}
		{
		}
		request.body =c.req.arrayBuffer()
		const response = await implement.jobsPush(request as JobsPushRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.delete('/job/:id', async (c) => {
		if (implement.jobsDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsDeleteRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.jobsDelete(request as JobsDeleteRequest)
		switch (response.code){
			case 204:
				return c.text("There is no content to send for this request, but the headers may be useful. ")
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.get('/job/:id/cat', async (c) => {
		if (implement.jobsFileCat===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsFileCatRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
			let path = c.req.query("path")
			if(path===undefined)return c.text("required parameter 'path' is not in 'query'", 400)
			request.path = path;
			let limit = c.req.query("limit")
			request.limit = limit;
		}
		const response = await implement.jobsFileCat(request as JobsFileCatRequest)
		switch (response.code){
			case 200:
				return c.body(response.body, 200, {'Content-Type': 'application/octet-stream'})
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.get('/job/:id/ls', async (c) => {
		if (implement.jobsFileLs===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsFileLsRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
			let path = c.req.query("path")
			request.path = path;
		}
		const response = await implement.jobsFileLs(request as JobsFileLsRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.get('/job/:id/task', async (c) => {
		if (implement.jobsTasklist===undefined)return c.text("not yet implemented", 500)
		const request: Partial<JobsTasklistRequest> = {}
		{
			let id = c.req.param("id")
			if(id===undefined)return c.text("required parameter 'id' is not in 'path'", 400)
			request.id = id;
		}
		const response = await implement.jobsTasklist(request as JobsTasklistRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text("The server cannot find the requested resource.")
		}
	})
	app.get('/status', async (c) => {
		if (implement.statusInterfaceStatus===undefined)return c.text("not yet implemented", 500)
		const request: Partial<StatusInterfaceStatusRequest> = {}
		{
		}
		const response = await implement.statusInterfaceStatus(request as StatusInterfaceStatusRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
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
