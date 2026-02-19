

























import { Hono } from 'hono'
import { serve } from '@hono/node-server'

// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin





type Address = {
	city:string|undefined,
	state:string|undefined,
	street:string|undefined,
	zip:string|undefined,
}

type ApiResponse = {
	code:number|undefined,
	message:string|undefined,
	type:string|undefined,
}

type Category = {
	id:number|undefined,
	name:string|undefined,
}

type Customer = {
	address:Address[]|undefined,
	id:number|undefined,
	username:string|undefined,
}

type Order = {
	complete:boolean|undefined,
	id:number|undefined,
	petId:number|undefined,
	quantity:number|undefined,
	shipDate:string|undefined,
	status:string|undefined,
}

type Pet = {
	category:Category|undefined,
	id:number|undefined,
	name:string,
	photoUrls:string[],
	status:string|undefined,
	tags:Tag[]|undefined,
}

type Tag = {
	id:number|undefined,
	name:string|undefined,
}

type User = {
	email:string|undefined,
	firstName:string|undefined,
	id:number|undefined,
	lastName:string|undefined,
	password:string|undefined,
	phone:string|undefined,
	userStatus:number|undefined,
	username:string|undefined,
}


// Request type for updatePet
type UpdatePetRequest = {
	body:Pet,
}
// Response type for updatePet
type UpdatePetResponse =
	| { code: 200; body:Pet}
	| { code: 400;}
	| { code: 404;}
	| { code: 422;}
// Request type for addPet
type AddPetRequest = {
	body:Pet,
}
// Response type for addPet
type AddPetResponse =
	| { code: 200; body:Pet}
	| { code: 400;}
	| { code: 422;}
// Request type for findPetsByStatus
type FindPetsByStatusRequest = {
	status:string|undefined,
}
// Response type for findPetsByStatus
type FindPetsByStatusResponse =
	| { code: 200; body:Pet[]}
	| { code: 400;}
// Request type for findPetsByTags
type FindPetsByTagsRequest = {
}
// Response type for findPetsByTags
type FindPetsByTagsResponse =
	| { code: 200; body:Pet[]}
	| { code: 400;}
// Request type for getPetById
type GetPetByIdRequest = {
	petId:number,
}
// Response type for getPetById
type GetPetByIdResponse =
	| { code: 200; body:Pet}
	| { code: 400;}
	| { code: 404;}
// Request type for updatePetWithForm
type UpdatePetWithFormRequest = {
	petId:number,
	name:string|undefined,
	status:string|undefined,
}
// Response type for updatePetWithForm
type UpdatePetWithFormResponse =
	| { code: 400;}
// Request type for deletePet
type DeletePetRequest = {
	api_key:string|undefined,
	petId:number,
}
// Response type for deletePet
type DeletePetResponse =
	| { code: 400;}
// Request type for uploadFile
type UploadFileRequest = {
	petId:number,
	additionalMetadata:string|undefined,
	body:ArrayBuffer,
}
// Response type for uploadFile
type UploadFileResponse =
	| { code: 200; body:ApiResponse}
// Request type for getInventory
type GetInventoryRequest = {
}
// Response type for getInventory
type GetInventoryResponse =
	| { code: 200; body:{ [key: string]: number }}
// Request type for placeOrder
type PlaceOrderRequest = {
	body:Order,
}
// Response type for placeOrder
type PlaceOrderResponse =
	| { code: 200; body:Order}
	| { code: 400;}
	| { code: 422;}
// Request type for getOrderById
type GetOrderByIdRequest = {
	orderId:number,
}
// Response type for getOrderById
type GetOrderByIdResponse =
	| { code: 200; body:Order}
	| { code: 400;}
	| { code: 404;}
// Request type for deleteOrder
type DeleteOrderRequest = {
	orderId:number,
}
// Response type for deleteOrder
type DeleteOrderResponse =
	| { code: 400;}
	| { code: 404;}
// Request type for createUser
type CreateUserRequest = {
	body:User,
}
// Response type for createUser
type CreateUserResponse =
	| { code: 500; body:User}
// Request type for createUsersWithListInput
type CreateUsersWithListInputRequest = {
	body:User[],
}
// Response type for createUsersWithListInput
type CreateUsersWithListInputResponse =
	| { code: 200; body:User}
	| { code: 500;}
// Request type for loginUser
type LoginUserRequest = {
	username:string|undefined,
	password:string|undefined,
}
// Response type for loginUser
type LoginUserResponse =
	| { code: 200; body:string}
	| { code: 400;}
// Request type for logoutUser
type LogoutUserRequest = {
}
// Response type for logoutUser
type LogoutUserResponse =
	| { code: 500;}
// Request type for getUserByName
type GetUserByNameRequest = {
	username:string,
}
// Response type for getUserByName
type GetUserByNameResponse =
	| { code: 200; body:User}
	| { code: 400;}
	| { code: 404;}
// Request type for updateUser
type UpdateUserRequest = {
	username:string,
	body:User,
}
// Response type for updateUser
type UpdateUserResponse =
	| { code: 500;}
// Request type for deleteUser
type DeleteUserRequest = {
	username:string,
}
// Response type for deleteUser
type DeleteUserResponse =
	| { code: 400;}
	| { code: 404;}




/// API Interface: Define handlers for each operation
interface ApiInterface{
	// PUT /pet
	updatePet?(request: UpdatePetRequest): Promise<UpdatePetResponse>
	// POST /pet
	addPet?(request: AddPetRequest): Promise<AddPetResponse>
	// GET /pet/findByStatus
	findPetsByStatus?(request: FindPetsByStatusRequest): Promise<FindPetsByStatusResponse>
	// GET /pet/findByTags
	findPetsByTags?(request: FindPetsByTagsRequest): Promise<FindPetsByTagsResponse>
	// GET /pet/{petId}
	getPetById?(request: GetPetByIdRequest): Promise<GetPetByIdResponse>
	// POST /pet/{petId}
	updatePetWithForm?(request: UpdatePetWithFormRequest): Promise<UpdatePetWithFormResponse>
	// DELETE /pet/{petId}
	deletePet?(request: DeletePetRequest): Promise<DeletePetResponse>
	// POST /pet/{petId}/uploadImage
	uploadFile?(request: UploadFileRequest): Promise<UploadFileResponse>
	// GET /store/inventory
	getInventory?(request: GetInventoryRequest): Promise<GetInventoryResponse>
	// POST /store/order
	placeOrder?(request: PlaceOrderRequest): Promise<PlaceOrderResponse>
	// GET /store/order/{orderId}
	getOrderById?(request: GetOrderByIdRequest): Promise<GetOrderByIdResponse>
	// DELETE /store/order/{orderId}
	deleteOrder?(request: DeleteOrderRequest): Promise<DeleteOrderResponse>
	// POST /user
	createUser?(request: CreateUserRequest): Promise<CreateUserResponse>
	// POST /user/createWithList
	createUsersWithListInput?(request: CreateUsersWithListInputRequest): Promise<CreateUsersWithListInputResponse>
	// GET /user/login
	loginUser?(request: LoginUserRequest): Promise<LoginUserResponse>
	// GET /user/logout
	logoutUser?(request: LogoutUserRequest): Promise<LogoutUserResponse>
	// GET /user/{username}
	getUserByName?(request: GetUserByNameRequest): Promise<GetUserByNameResponse>
	// PUT /user/{username}
	updateUser?(request: UpdateUserRequest): Promise<UpdateUserResponse>
	// DELETE /user/{username}
	deleteUser?(request: DeleteUserRequest): Promise<DeleteUserResponse>
}

/// Register operation routes to Hono app
export function addHonoOperations(app: Hono, implement: ApiInterface){
	app.put('/pet', async (c) => {
		if (implement.updatePet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UpdatePetRequest> = {}
		{
		}
		request.body =(await c.req.json()) as Pet
		const response = await implement.updatePet(request as UpdatePetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid ID supplied")
			case 404:
				return c.text("Pet not found")
			case 422:
				return c.text("Validation exception")
		}
	})
	app.post('/pet', async (c) => {
		if (implement.addPet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<AddPetRequest> = {}
		{
		}
		request.body =(await c.req.json()) as Pet
		const response = await implement.addPet(request as AddPetRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid input")
			case 422:
				return c.text("Validation exception")
		}
	})
	app.get('/pet/findByStatus', async (c) => {
		if (implement.findPetsByStatus===undefined)return c.text("not yet implemented", 500)
		const request: Partial<FindPetsByStatusRequest> = {}
		{
			let status = c.req.query("status")
			request.status = status;
		}
		const response = await implement.findPetsByStatus(request as FindPetsByStatusRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid status value")
		}
	})
	app.get('/pet/findByTags', async (c) => {
		if (implement.findPetsByTags===undefined)return c.text("not yet implemented", 500)
		const request: Partial<FindPetsByTagsRequest> = {}
		{
		}
		const response = await implement.findPetsByTags(request as FindPetsByTagsRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid tag value")
		}
	})
	app.get('/pet/:petId', async (c) => {
		if (implement.getPetById===undefined)return c.text("not yet implemented", 500)
		const request: Partial<GetPetByIdRequest> = {}
		{
			let pet_id = c.req.param("petId")
			if(pet_id===undefined)return c.text("required parameter 'petId' is not in 'path'", 400)
			request.petId = pet_id;
		}
		const response = await implement.getPetById(request as GetPetByIdRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid ID supplied")
			case 404:
				return c.text("Pet not found")
		}
	})
	app.post('/pet/:petId', async (c) => {
		if (implement.updatePetWithForm===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UpdatePetWithFormRequest> = {}
		{
			let pet_id = c.req.param("petId")
			if(pet_id===undefined)return c.text("required parameter 'petId' is not in 'path'", 400)
			request.petId = pet_id;
			let name = c.req.query("name")
			request.name = name;
			let status = c.req.query("status")
			request.status = status;
		}
		const response = await implement.updatePetWithForm(request as UpdatePetWithFormRequest)
		switch (response.code){
			case 400:
				return c.text("Invalid input")
		}
	})
	app.delete('/pet/:petId', async (c) => {
		if (implement.deletePet===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeletePetRequest> = {}
		{
			let api_key = c.req.header("api_key")
			request.api_key = api_key;
			let pet_id = c.req.param("petId")
			if(pet_id===undefined)return c.text("required parameter 'petId' is not in 'path'", 400)
			request.petId = pet_id;
		}
		const response = await implement.deletePet(request as DeletePetRequest)
		switch (response.code){
			case 400:
				return c.text("Invalid pet value")
		}
	})
	app.post('/pet/:petId/uploadImage', async (c) => {
		if (implement.uploadFile===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UploadFileRequest> = {}
		{
			let pet_id = c.req.param("petId")
			if(pet_id===undefined)return c.text("required parameter 'petId' is not in 'path'", 400)
			request.petId = pet_id;
			let additional_metadata = c.req.query("additionalMetadata")
			request.additionalMetadata = additional_metadata;
		}
		request.body =c.req.arrayBuffer()
		const response = await implement.uploadFile(request as UploadFileRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
		}
	})
	app.get('/store/inventory', async (c) => {
		if (implement.getInventory===undefined)return c.text("not yet implemented", 500)
		const request: Partial<GetInventoryRequest> = {}
		{
		}
		const response = await implement.getInventory(request as GetInventoryRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
		}
	})
	app.post('/store/order', async (c) => {
		if (implement.placeOrder===undefined)return c.text("not yet implemented", 500)
		const request: Partial<PlaceOrderRequest> = {}
		{
		}
		request.body =(await c.req.json()) as Order
		const response = await implement.placeOrder(request as PlaceOrderRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid input")
			case 422:
				return c.text("Validation exception")
		}
	})
	app.get('/store/order/:orderId', async (c) => {
		if (implement.getOrderById===undefined)return c.text("not yet implemented", 500)
		const request: Partial<GetOrderByIdRequest> = {}
		{
			let order_id = c.req.param("orderId")
			if(order_id===undefined)return c.text("required parameter 'orderId' is not in 'path'", 400)
			request.orderId = order_id;
		}
		const response = await implement.getOrderById(request as GetOrderByIdRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid ID supplied")
			case 404:
				return c.text("Order not found")
		}
	})
	app.delete('/store/order/:orderId', async (c) => {
		if (implement.deleteOrder===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeleteOrderRequest> = {}
		{
			let order_id = c.req.param("orderId")
			if(order_id===undefined)return c.text("required parameter 'orderId' is not in 'path'", 400)
			request.orderId = order_id;
		}
		const response = await implement.deleteOrder(request as DeleteOrderRequest)
		switch (response.code){
			case 400:
				return c.text("Invalid ID supplied")
			case 404:
				return c.text("Order not found")
		}
	})
	app.post('/user', async (c) => {
		if (implement.createUser===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CreateUserRequest> = {}
		{
		}
		request.body =(await c.req.json()) as User
		const response = await implement.createUser(request as CreateUserRequest)
		switch (response.code){
			case 500:
				return c.json(response.body, 500)
		}
	})
	app.post('/user/createWithList', async (c) => {
		if (implement.createUsersWithListInput===undefined)return c.text("not yet implemented", 500)
		const request: Partial<CreateUsersWithListInputRequest> = {}
		{
		}
		request.body =(await c.req.json()) as User[]
		const response = await implement.createUsersWithListInput(request as CreateUsersWithListInputRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 500:
				return c.text("successful operation")
		}
	})
	app.get('/user/login', async (c) => {
		if (implement.loginUser===undefined)return c.text("not yet implemented", 500)
		const request: Partial<LoginUserRequest> = {}
		{
			let username = c.req.query("username")
			request.username = username;
			let password = c.req.query("password")
			request.password = password;
		}
		const response = await implement.loginUser(request as LoginUserRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid username/password supplied")
		}
	})
	app.get('/user/logout', async (c) => {
		if (implement.logoutUser===undefined)return c.text("not yet implemented", 500)
		const request: Partial<LogoutUserRequest> = {}
		{
		}
		const response = await implement.logoutUser(request as LogoutUserRequest)
		switch (response.code){
			case 500:
				return c.text("successful operation")
		}
	})
	app.get('/user/:username', async (c) => {
		if (implement.getUserByName===undefined)return c.text("not yet implemented", 500)
		const request: Partial<GetUserByNameRequest> = {}
		{
			let username = c.req.param("username")
			if(username===undefined)return c.text("required parameter 'username' is not in 'path'", 400)
			request.username = username;
		}
		const response = await implement.getUserByName(request as GetUserByNameRequest)
		switch (response.code){
			case 200:
				return c.json(response.body, 200)
			case 400:
				return c.text("Invalid username supplied")
			case 404:
				return c.text("User not found")
		}
	})
	app.put('/user/:username', async (c) => {
		if (implement.updateUser===undefined)return c.text("not yet implemented", 500)
		const request: Partial<UpdateUserRequest> = {}
		{
			let username = c.req.param("username")
			if(username===undefined)return c.text("required parameter 'username' is not in 'path'", 400)
			request.username = username;
		}
		request.body =(await c.req.json()) as User
		const response = await implement.updateUser(request as UpdateUserRequest)
		switch (response.code){
			case 500:
				return c.text("successful operation")
		}
	})
	app.delete('/user/:username', async (c) => {
		if (implement.deleteUser===undefined)return c.text("not yet implemented", 500)
		const request: Partial<DeleteUserRequest> = {}
		{
			let username = c.req.param("username")
			if(username===undefined)return c.text("required parameter 'username' is not in 'path'", 400)
			request.username = username;
		}
		const response = await implement.deleteUser(request as DeleteUserRequest)
		switch (response.code){
			case 400:
				return c.text("Invalid username supplied")
			case 404:
				return c.text("User not found")
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
