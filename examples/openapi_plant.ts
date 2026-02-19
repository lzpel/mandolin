

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type Apikey = {
	expiration:number,
	user:User,
}

type Device = {
	key:string,
	key_user:string,
	latitude:number,
	longitude:number,
	name:string,
}

type Position = {
	latitude:number,
	longitude:number,
}

type Record = {
	bbox:number[],
	key:string,
	key_device:string,
	label:string[],
	latitude:number,
	longitude:number,
}

type User = {
	auth:string[],
	email:string,
	key:string,
	name:string,
}


// Request type for operationClean
type OperationCleanRequest = {
}
// Response type for operationClean
type OperationCleanResponse =
	| { code: 200; body:string}
	| { code: 400; body:string}
// Request type for credentialGet
type CredentialGetRequest = {
}
// Response type for credentialGet
type CredentialGetResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for credentialSignIn
type CredentialSignInRequest = {
	body:PathsCredentialSignInPostRequestBodyContentApplicationJsonSchema,
}
// Response type for credentialSignIn
type CredentialSignInResponse =
	| { code: 200; body:string}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for credentialSignUp
type CredentialSignUpRequest = {
	body:User,
}
// Response type for credentialSignUp
type CredentialSignUpResponse =
	| { code: 200; body:string}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for credentialValidate
type CredentialValidateRequest = {
	body:string,
}
// Response type for credentialValidate
type CredentialValidateResponse =
	| { code: 200; body:Apikey}
	| { code: 400; body:string}
// Request type for deviceListGet
type DeviceListGetRequest = {
	limit:number|undefined,
	order:string|undefined,
	max_key:string|undefined,
	min_key:string|undefined,
}
// Response type for deviceListGet
type DeviceListGetResponse =
	| { code: 200; body:Device[]}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for deviceListAdd
type DeviceListAddRequest = {
	body:Device,
}
// Response type for deviceListAdd
type DeviceListAddResponse =
	| { code: 200; body:Device}
	| { code: 400; body:string}
// Request type for deviceGet
type DeviceGetRequest = {
	key:string,
}
// Response type for deviceGet
type DeviceGetResponse =
	| { code: 200; body:Device}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for deviceUpdate
type DeviceUpdateRequest = {
	key:string,
	body:Device,
}
// Response type for deviceUpdate
type DeviceUpdateResponse =
	| { code: 200; body:Device}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for deviceDelete
type DeviceDeleteRequest = {
	key:string,
}
// Response type for deviceDelete
type DeviceDeleteResponse =
	| { code: 200;}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for operationKey
type OperationKeyRequest = {
	key:string,
	seconds:number|undefined,
}
// Response type for operationKey
type OperationKeyResponse =
	| { code: 200; body:string}
	| { code: 400; body:string}
// Request type for mlImage
type MlImageRequest = {
	body:ArrayBuffer,
}
// Response type for mlImage
type MlImageResponse =
	| { code: 200; body:ArrayBuffer}
// Request type for mlJson
type MlJsonRequest = {
	body:ArrayBuffer,
}
// Response type for mlJson
type MlJsonResponse =
	| { code: 200; body:string}
// Request type for recordListGet
type RecordListGetRequest = {
	limit:number|undefined,
	order:string|undefined,
	key_device:string,
	max_key:string|undefined,
	min_key:string|undefined,
}
// Response type for recordListGet
type RecordListGetResponse =
	| { code: 200; body:Record[]}
	| { code: 400; body:string}
// Request type for recordListAdd
type RecordListAddRequest = {
	key_device:string,
	latitude:number,
	longitude:number,
	seconds:number|undefined,
	body:ArrayBuffer,
}
// Response type for recordListAdd
type RecordListAddResponse =
	| { code: 200; body:Record}
	| { code: 400; body:string}
// Request type for recordGet
type RecordGetRequest = {
	key:string,
}
// Response type for recordGet
type RecordGetResponse =
	| { code: 200; body:Record}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for recordUpdate
type RecordUpdateRequest = {
	key:string,
	body:Record,
}
// Response type for recordUpdate
type RecordUpdateResponse =
	| { code: 200; body:Record}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for recordDelete
type RecordDeleteRequest = {
	key:string,
}
// Response type for recordDelete
type RecordDeleteResponse =
	| { code: 200;}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for testB
type TestBRequest = {
	limit:number|undefined,
	cursor:number|undefined,
	order:string,
}
// Response type for testB
type TestBResponse =
	| { code: 200; body:string}
// Request type for testD
type TestDRequest = {
}
// Response type for testD
type TestDResponse =
	| { code: 200; body:string}
// Request type for testC
type TestCRequest = {
	limit:number|undefined,
	cursor:number|undefined,
	order:string,
}
// Response type for testC
type TestCResponse =
	| { code: 200; body:string}
// Request type for testA
type TestARequest = {
}
// Response type for testA
type TestAResponse =
	| { code: 200; body:string}
// Request type for userListGet
type UserListGetRequest = {
	limit:number|undefined,
	order:string|undefined,
}
// Response type for userListGet
type UserListGetResponse =
	| { code: 200; body:User[]}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for userGet
type UserGetRequest = {
	key:string,
}
// Response type for userGet
type UserGetResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for userUpdate
type UserUpdateRequest = {
	key:string,
	body:User,
}
// Response type for userUpdate
type UserUpdateResponse =
	| { code: 200; body:User}
	| { code: 400; body:string}
	| { code: 404; body:string}
// Request type for userDelete
type UserDeleteRequest = {
	key:string,
}
// Response type for userDelete
type UserDeleteResponse =
	| { code: 200;}
	| { code: 400; body:string}
	| { code: 404; body:string}




type PathsCredentialSignInPostRequestBodyContentApplicationJsonSchema = {
	has_auth:string,
}

/// API Interface: Define handlers for each operation
interface ApiInterface{
	// DELETE /clean
	operationClean?(request: OperationCleanRequest): Promise<OperationCleanResponse>
	// GET /credential/iam
	credentialGet?(request: CredentialGetRequest): Promise<CredentialGetResponse>
	// POST /credential/sign_in
	credentialSignIn?(request: CredentialSignInRequest): Promise<CredentialSignInResponse>
	// POST /credential/sign_up
	credentialSignUp?(request: CredentialSignUpRequest): Promise<CredentialSignUpResponse>
	// GET /credential/validate
	credentialValidate?(request: CredentialValidateRequest): Promise<CredentialValidateResponse>
	// GET /device
	deviceListGet?(request: DeviceListGetRequest): Promise<DeviceListGetResponse>
	// POST /device
	deviceListAdd?(request: DeviceListAddRequest): Promise<DeviceListAddResponse>
	// GET /device/{key}
	deviceGet?(request: DeviceGetRequest): Promise<DeviceGetResponse>
	// POST /device/{key}
	deviceUpdate?(request: DeviceUpdateRequest): Promise<DeviceUpdateResponse>
	// DELETE /device/{key}
	deviceDelete?(request: DeviceDeleteRequest): Promise<DeviceDeleteResponse>
	// GET /key/{key}
	operationKey?(request: OperationKeyRequest): Promise<OperationKeyResponse>
	// POST /ml/image
	mlImage?(request: MlImageRequest): Promise<MlImageResponse>
	// POST /ml/json
	mlJson?(request: MlJsonRequest): Promise<MlJsonResponse>
	// GET /record
	recordListGet?(request: RecordListGetRequest): Promise<RecordListGetResponse>
	// POST /record
	recordListAdd?(request: RecordListAddRequest): Promise<RecordListAddResponse>
	// GET /record/{key}
	recordGet?(request: RecordGetRequest): Promise<RecordGetResponse>
	// POST /record/{key}
	recordUpdate?(request: RecordUpdateRequest): Promise<RecordUpdateResponse>
	// DELETE /record/{key}
	recordDelete?(request: RecordDeleteRequest): Promise<RecordDeleteResponse>
	// GET /test
	testB?(request: TestBRequest): Promise<TestBResponse>
	// PUT /test
	testD?(request: TestDRequest): Promise<TestDResponse>
	// POST /test
	testC?(request: TestCRequest): Promise<TestCResponse>
	// DELETE /test
	testA?(request: TestARequest): Promise<TestAResponse>
	// GET /user
	userListGet?(request: UserListGetRequest): Promise<UserListGetResponse>
	// GET /user/{key}
	userGet?(request: UserGetRequest): Promise<UserGetResponse>
	// POST /user/{key}
	userUpdate?(request: UserUpdateRequest): Promise<UserUpdateResponse>
	// DELETE /user/{key}
	userDelete?(request: UserDeleteRequest): Promise<UserDeleteResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.delete('/clean', async (c) => {
		if (implement.operationClean===undefined)return c.text("not yet implemented", 500)
		const request: Partial<OperationCleanRequest> = {}
		{
		}
		const response = await implement.operationClean(request as OperationCleanRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/credential/iam', async (c) => {
		if (implement.credentialGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CredentialGetRequest> = {}
		{
		}
		const response = await implement.credentialGet(request as CredentialGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/credential/sign_in', async (c) => {
		if (implement.credentialSignIn===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CredentialSignInRequest> = {}
		{
		}
		request.body =(await c.req.json()) as PathsCredentialSignInPostRequestBodyContentApplicationJsonSchema
		const response = await implement.credentialSignIn(request as CredentialSignInRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/credential/sign_up', async (c) => {
		if (implement.credentialSignUp===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CredentialSignUpRequest> = {}
		{
		}
		request.body =(await c.req.json()) as User
		const response = await implement.credentialSignUp(request as CredentialSignUpRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/credential/validate', async (c) => {
		if (implement.credentialValidate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CredentialValidateRequest> = {}
		{
		}
		request.body =(await c.req.json()) as string
		const response = await implement.credentialValidate(request as CredentialValidateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/device', async (c) => {
		if (implement.deviceListGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeviceListGetRequest> = {}
		{
			let limit = c.req.query("limit")
			request.limit = limit;
			let order = c.req.query("order")
			request.order = order;
			let max_key = c.req.query("max_key")
			request.max_key = max_key;
			let min_key = c.req.query("min_key")
			request.min_key = min_key;
		}
		const response = await implement.deviceListGet(request as DeviceListGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/device', async (c) => {
		if (implement.deviceListAdd===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeviceListAddRequest> = {}
		{
		}
		request.body =(await c.req.json()) as Device
		const response = await implement.deviceListAdd(request as DeviceListAddRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/device/:key', async (c) => {
		if (implement.deviceGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeviceGetRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.deviceGet(request as DeviceGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/device/:key', async (c) => {
		if (implement.deviceUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeviceUpdateRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		request.body =(await c.req.json()) as Device
		const response = await implement.deviceUpdate(request as DeviceUpdateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.delete('/device/:key', async (c) => {
		if (implement.deviceDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeviceDeleteRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.deviceDelete(request as DeviceDeleteRequest)
		switch (response.code){
			case 200:
				return c.text("The request has succeeded.")
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/key/:key', async (c) => {
		if (implement.operationKey===undefined)return c.text("not yet implemented", 500)
		const request: Partial<OperationKeyRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
			let seconds = c.req.query("seconds")
			request.seconds = seconds;
		}
		const response = await implement.operationKey(request as OperationKeyRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/ml/image', async (c) => {
		if (implement.mlImage===undefined)return c.text("not yet implemented", 500)
		const request: Partial<MlImageRequest> = {}
		{
		}
		request.body =c.req.arrayBuffer()
		const response = await implement.mlImage(request as MlImageRequest)
		switch (response.code){
			case 200:
				return c.body(response.body, 200, {'Content-Type': 'image/png'})
		}
	})
	app.post('/ml/json', async (c) => {
		if (implement.mlJson===undefined)return c.text("not yet implemented", 500)
		const request: Partial<MlJsonRequest> = {}
		{
		}
		request.body =c.req.arrayBuffer()
		const response = await implement.mlJson(request as MlJsonRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/record', async (c) => {
		if (implement.recordListGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<RecordListGetRequest> = {}
		{
			let limit = c.req.query("limit")
			request.limit = limit;
			let order = c.req.query("order")
			request.order = order;
			let key_device = c.req.query("key_device")
			if(key_device===undefined)return c.text("required parameter 'key_device' is not in 'query'", 400)
			request.key_device = key_device;
			let max_key = c.req.query("max_key")
			request.max_key = max_key;
			let min_key = c.req.query("min_key")
			request.min_key = min_key;
		}
		const response = await implement.recordListGet(request as RecordListGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/record', async (c) => {
		if (implement.recordListAdd===undefined)return c.text("not yet implemented", 500)
		const request: Partial<RecordListAddRequest> = {}
		{
			let key_device = c.req.query("key_device")
			if(key_device===undefined)return c.text("required parameter 'key_device' is not in 'query'", 400)
			request.key_device = key_device;
			let latitude = c.req.query("latitude")
			if(latitude===undefined)return c.text("required parameter 'latitude' is not in 'query'", 400)
			request.latitude = latitude;
			let longitude = c.req.query("longitude")
			if(longitude===undefined)return c.text("required parameter 'longitude' is not in 'query'", 400)
			request.longitude = longitude;
			let seconds = c.req.query("seconds")
			request.seconds = seconds;
		}
		request.body =c.req.arrayBuffer()
		const response = await implement.recordListAdd(request as RecordListAddRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/record/:key', async (c) => {
		if (implement.recordGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<RecordGetRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.recordGet(request as RecordGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/record/:key', async (c) => {
		if (implement.recordUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<RecordUpdateRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		request.body =(await c.req.json()) as Record
		const response = await implement.recordUpdate(request as RecordUpdateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.delete('/record/:key', async (c) => {
		if (implement.recordDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<RecordDeleteRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.recordDelete(request as RecordDeleteRequest)
		switch (response.code){
			case 200:
				return c.text("The request has succeeded.")
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/test', async (c) => {
		if (implement.testB===undefined)return c.text("not yet implemented", 500)
		const request: Partial<TestBRequest> = {}
		{
			let limit = c.req.query("limit")
			request.limit = limit;
			let cursor = c.req.query("cursor")
			request.cursor = cursor;
			let order = c.req.query("order")
			if(order===undefined)return c.text("required parameter 'order' is not in 'query'", 400)
			request.order = order;
		}
		const response = await implement.testB(request as TestBRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.put('/test', async (c) => {
		if (implement.testD===undefined)return c.text("not yet implemented", 500)
		const request: Partial<TestDRequest> = {}
		{
		}
		const response = await implement.testD(request as TestDRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/test', async (c) => {
		if (implement.testC===undefined)return c.text("not yet implemented", 500)
		const request: Partial<TestCRequest> = {}
		{
			let limit = c.req.query("limit")
			request.limit = limit;
			let cursor = c.req.query("cursor")
			request.cursor = cursor;
			let order = c.req.query("order")
			if(order===undefined)return c.text("required parameter 'order' is not in 'query'", 400)
			request.order = order;
		}
		const response = await implement.testC(request as TestCRequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.delete('/test', async (c) => {
		if (implement.testA===undefined)return c.text("not yet implemented", 500)
		const request: Partial<TestARequest> = {}
		{
		}
		const response = await implement.testA(request as TestARequest)
		switch (response.code){
			case 200:
				return c.text(response.body, 200, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/user', async (c) => {
		if (implement.userListGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserListGetRequest> = {}
		{
			let limit = c.req.query("limit")
			request.limit = limit;
			let order = c.req.query("order")
			request.order = order;
		}
		const response = await implement.userListGet(request as UserListGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.get('/user/:key', async (c) => {
		if (implement.userGet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserGetRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.userGet(request as UserGetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.post('/user/:key', async (c) => {
		if (implement.userUpdate===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserUpdateRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		request.body =(await c.req.json()) as User
		const response = await implement.userUpdate(request as UserUpdateRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
		}
	})
	app.delete('/user/:key', async (c) => {
		if (implement.userDelete===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UserDeleteRequest> = {}
		{
			let key = c.req.param("key")
			if(key===undefined)return c.text("required parameter 'key' is not in 'path'", 400)
			request.key = key;
		}
		const response = await implement.userDelete(request as UserDeleteRequest)
		switch (response.code){
			case 200:
				return c.text("The request has succeeded.")
			case 400:
				return c.text(response.body, 400, {'Content-Type': 'text/plain'})
			case 404:
				return c.text(response.body, 404, {'Content-Type': 'text/plain'})
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
