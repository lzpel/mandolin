

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type NotFoundResponse = {
	body:,
}

type Page = {
	content:string,
	id:Uuid,
	id_root:Uuid,
	name:string,
	path_image:string[],
	path_model:string,
	progress:number,
	script:string,
	view_image:string[],
}

type Plan = {
	currency:string,
	description:string|undefined,
	id:Uuid,
	name:string,
	priceMonthly:number,
}

type Subscription = {
	id:Uuid,
	id_node:Uuid,
	id_root:Uuid,
	status:string,
}

type Uuid=string;

type User = {
	auth_email:string,
	auth_email_password:string,
	auth_google:string,
	id:Uuid,
	is_active:boolean,
	name:string,
	picture:string,
}


// Request type for authApiCallbackOauth
type AuthApiCallbackOauthRequest = {
	code:string,
	state:string,
}
// Response type for authApiCallbackOauth
type AuthApiCallbackOauthResponse =
	| { code: 200; body:string}
	| { code: 400; body:string}
// Request type for authApiEmail
type AuthApiEmailRequest = {
	body:PathsAuthEmailPostRequestBodyContentApplicationJsonSchema,
}
// Response type for authApiEmail
type AuthApiEmailResponse =
	| { code: 204; body:}
	| { code: 400; body:string}
// Request type for authApiGoogle
type AuthApiGoogleRequest = {
}
// Response type for authApiGoogle
type AuthApiGoogleResponse =
	| { code: 200; body:string}
// Request type for authApiOut
type AuthApiOutRequest = {
}
// Response type for authApiOut
type AuthApiOutResponse =
	| { code: 204; body:}
// Request type for pageApiGet
type PageApiGetRequest = {
}
// Response type for pageApiGet
type PageApiGetResponse =
	| { code: 200; body:Page[]}
	| { code: 400; body:string}
	| { code: 403; body:}
// Request type for pageApiPush
type PageApiPushRequest = {
	body:PathsPagePostRequestBodyContentApplicationJsonSchema,
}
// Response type for pageApiPush
type PageApiPushResponse =
	| { code: 200; body:Page}
	| { code: 400; body:string}
	| { code: 403; body:}
// Request type for pageApiUpload
type PageApiUploadRequest = {
	fileName:string,
	expiresIn:number|undefined,
}
// Response type for pageApiUpload
type PageApiUploadResponse =
	| { code: 200; body:string[]}
	| { code: 400; body:string}
// Request type for userApiUserGet
type UserApiUserGetRequest = {
}
// Response type for userApiUserGet
type UserApiUserGetResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 403; body:}
// Request type for userApiUserSet
type UserApiUserSetRequest = {
	body:PathsUserPostRequestBodyContentApplicationJsonSchema,
}
// Response type for userApiUserSet
type UserApiUserSetResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 403; body:}
// Request type for userApiUserPop
type UserApiUserPopRequest = {
}
// Response type for userApiUserPop
type UserApiUserPopResponse =
	| { code: 204; body:}
	| { code: 400; body:string}
	| { code: 403; body:}




type PathsAuthEmailPostRequestBodyContentApplicationJsonSchema = {
	email:string,
}

type PathsUserPostRequestBodyContentApplicationJsonSchema = {
	user:User,
}

type PathsPagePostRequestBodyContentApplicationJsonSchema = {
	path_image:string[],
	script:string,
	view_image:string[],
}

/// API Interface: Define handlers for each operation
interface ApiInterface{
	// GET /auth/callback_oauth
	authApiCallbackOauth?(request: AuthApiCallbackOauthRequest): Promise<AuthApiCallbackOauthResponse>
	// POST /auth/email
	authApiEmail?(request: AuthApiEmailRequest): Promise<AuthApiEmailResponse>
	// GET /auth/google
	authApiGoogle?(request: AuthApiGoogleRequest): Promise<AuthApiGoogleResponse>
	// GET /auth/out
	authApiOut?(request: AuthApiOutRequest): Promise<AuthApiOutResponse>
	// GET /page
	pageApiGet?(request: PageApiGetRequest): Promise<PageApiGetResponse>
	// POST /page
	pageApiPush?(request: PageApiPushRequest): Promise<PageApiPushResponse>
	// POST /page/upload
	pageApiUpload?(request: PageApiUploadRequest): Promise<PageApiUploadResponse>
	// GET /user
	userApiUserGet?(request: UserApiUserGetRequest): Promise<UserApiUserGetResponse>
	// POST /user
	userApiUserSet?(request: UserApiUserSetRequest): Promise<UserApiUserSetResponse>
	// DELETE /user
	userApiUserPop?(request: UserApiUserPopRequest): Promise<UserApiUserPopResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.get('/auth/callback_oauth', async (c) => {
		if (implement.authApiCallbackOauth===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiCallbackOauthRequest> = {}
		{
			let code = c.req.query("code")
			if(code===undefined)return c.text("required parameter 'code' is not in 'query'", 400)
			request.code = code;
			let state = c.req.query("state")
			if(state===undefined)return c.text("required parameter 'state' is not in 'query'", 400)
			request.state = state;
		}
		const response = await implement.authApiCallbackOauth(request as AuthApiCallbackOauthRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/auth/email', async (c) => {
		if (implement.authApiEmail===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiEmailRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsAuthEmailPostRequestBodyContentApplicationJsonSchema
		const response = await implement.authApiEmail(request as AuthApiEmailRequest)
		switch (response.code){
			case 204:
				return c.json(response.body, 204)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/auth/google', async (c) => {
		if (implement.authApiGoogle===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AuthApiGoogleRequest> = {}
		{
		}
		const response = await implement.authApiGoogle(request as AuthApiGoogleRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
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
				return c.json(response.body, 204)
		}
	})
	app.get('/page', async (c) => {
		if (implement.pageApiGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<PageApiGetRequest> = {}
		{
		}
		const response = await implement.pageApiGet(request as PageApiGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.json(response.body, 403)
		}
	})
	app.post('/page', async (c) => {
		if (implement.pageApiPush===undefined)return c.text("not yet implemented", 500)
		const request: Partial<PageApiPushRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsPagePostRequestBodyContentApplicationJsonSchema
		const response = await implement.pageApiPush(request as PageApiPushRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.json(response.body, 403)
		}
	})
	app.post('/page/upload', async (c) => {
		if (implement.pageApiUpload===undefined)return c.text("not yet implemented", 500)
		const request: Partial<PageApiUploadRequest> = {}
		{
			let file_name = c.req.query("fileName")
			if(file_name===undefined)return c.text("required parameter 'fileName' is not in 'query'", 400)
			request.fileName = file_name;
			let expires_in = c.req.query("expiresIn")
			request.expiresIn = expires_in;
		}
		const response = await implement.pageApiUpload(request as PageApiUploadRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/user', async (c) => {
		if (implement.userApiUserGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserApiUserGetRequest> = {}
		{
		}
		const response = await implement.userApiUserGet(request as UserApiUserGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.json(response.body, 403)
		}
	})
	app.post('/user', async (c) => {
		if (implement.userApiUserSet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserApiUserSetRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsUserPostRequestBodyContentApplicationJsonSchema
		const response = await implement.userApiUserSet(request as UserApiUserSetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.json(response.body, 403)
		}
	})
	app.delete('/user', async (c) => {
		if (implement.userApiUserPop===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserApiUserPopRequest> = {}
		{
		}
		const response = await implement.userApiUserPop(request as UserApiUserPopRequest)
		switch (response.code){
			case 204:
				return c.json(response.body, 204)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 403:
				return c.json(response.body, 403)
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
