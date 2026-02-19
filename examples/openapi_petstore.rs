












































// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin

/* Cargo.toml to build this server

[dependencies]
serde= { version="*", features = ["derive"] }
serde_json= "*"
axum = { version = "*", features = ["multipart"] }
tokio = { version = "*", features = ["rt", "rt-multi-thread", "macros", "signal"] }
# optional
uuid = { version = "*", features = ["serde"] }
chrono = { version = "*", features = ["serde"] }
*/

use std::collections::HashMap;
use serde;
use std::future::Future;

/// API Interface Trait
/// Define server logic by implementing methods corresponding to each operation
pub trait ApiInterface{
	/// Authentication process: Generate AuthContext from request
	fn authorize(&self, _req: axum::http::Request<axum::body::Body>) -> impl Future<Output = Result<AuthContext, String>> + Send{async { Ok(Default::default()) } }
	// PUT /pet
	fn update_pet(&self, _req: UpdatePetRequest) -> impl Future<Output = UpdatePetResponse> + Send{async{Default::default()}}
	// POST /pet
	fn add_pet(&self, _req: AddPetRequest) -> impl Future<Output = AddPetResponse> + Send{async{Default::default()}}
	// GET /pet/findByStatus
	fn find_pets_by_status(&self, _req: FindPetsByStatusRequest) -> impl Future<Output = FindPetsByStatusResponse> + Send{async{Default::default()}}
	// GET /pet/findByTags
	fn find_pets_by_tags(&self, _req: FindPetsByTagsRequest) -> impl Future<Output = FindPetsByTagsResponse> + Send{async{Default::default()}}
	// GET /pet/{petId}
	fn get_pet_by_id(&self, _req: GetPetByIdRequest) -> impl Future<Output = GetPetByIdResponse> + Send{async{Default::default()}}
	// POST /pet/{petId}
	fn update_pet_with_form(&self, _req: UpdatePetWithFormRequest) -> impl Future<Output = UpdatePetWithFormResponse> + Send{async{Default::default()}}
	// DELETE /pet/{petId}
	fn delete_pet(&self, _req: DeletePetRequest) -> impl Future<Output = DeletePetResponse> + Send{async{Default::default()}}
	// POST /pet/{petId}/uploadImage
	fn upload_file(&self, _req: UploadFileRequest) -> impl Future<Output = UploadFileResponse> + Send{async{Default::default()}}
	// GET /store/inventory
	fn get_inventory(&self, _req: GetInventoryRequest) -> impl Future<Output = GetInventoryResponse> + Send{async{Default::default()}}
	// POST /store/order
	fn place_order(&self, _req: PlaceOrderRequest) -> impl Future<Output = PlaceOrderResponse> + Send{async{Default::default()}}
	// GET /store/order/{orderId}
	fn get_order_by_id(&self, _req: GetOrderByIdRequest) -> impl Future<Output = GetOrderByIdResponse> + Send{async{Default::default()}}
	// DELETE /store/order/{orderId}
	fn delete_order(&self, _req: DeleteOrderRequest) -> impl Future<Output = DeleteOrderResponse> + Send{async{Default::default()}}
	// POST /user
	fn create_user(&self, _req: CreateUserRequest) -> impl Future<Output = CreateUserResponse> + Send{async{Default::default()}}
	// POST /user/createWithList
	fn create_users_with_list_input(&self, _req: CreateUsersWithListInputRequest) -> impl Future<Output = CreateUsersWithListInputResponse> + Send{async{Default::default()}}
	// GET /user/login
	fn login_user(&self, _req: LoginUserRequest) -> impl Future<Output = LoginUserResponse> + Send{async{Default::default()}}
	// GET /user/logout
	fn logout_user(&self, _req: LogoutUserRequest) -> impl Future<Output = LogoutUserResponse> + Send{async{Default::default()}}
	// GET /user/{username}
	fn get_user_by_name(&self, _req: GetUserByNameRequest) -> impl Future<Output = GetUserByNameResponse> + Send{async{Default::default()}}
	// PUT /user/{username}
	fn update_user(&self, _req: UpdateUserRequest) -> impl Future<Output = UpdateUserResponse> + Send{async{Default::default()}}
	// DELETE /user/{username}
	fn delete_user(&self, _req: DeleteUserRequest) -> impl Future<Output = DeleteUserResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for update_pet
#[derive(Debug)]
pub struct UpdatePetRequest{
	pub body: Pet,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for UpdatePetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for update_pet
#[derive(Debug)]
pub enum UpdatePetResponse{
	Status200(Pet),
	Status400,
	Status404,
	Status422,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UpdatePetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for add_pet
#[derive(Debug)]
pub struct AddPetRequest{
	pub body: Pet,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for AddPetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for add_pet
#[derive(Debug)]
pub enum AddPetResponse{
	Status200(Pet),
	Status400,
	Status422,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AddPetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for find_pets_by_status
#[derive(Debug)]
pub struct FindPetsByStatusRequest{
	pub status:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for FindPetsByStatusRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for find_pets_by_status
#[derive(Debug)]
pub enum FindPetsByStatusResponse{
	Status200(Vec<Pet>),
	Status400,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for FindPetsByStatusResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for find_pets_by_tags
#[derive(Debug)]
pub struct FindPetsByTagsRequest{
	pub tags:Option<Vec<String>>,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for FindPetsByTagsRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for find_pets_by_tags
#[derive(Debug)]
pub enum FindPetsByTagsResponse{
	Status200(Vec<Pet>),
	Status400,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for FindPetsByTagsResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for get_pet_by_id
#[derive(Debug)]
pub struct GetPetByIdRequest{
	pub petId:i64,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"api_key": []}, {"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for GetPetByIdRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for get_pet_by_id
#[derive(Debug)]
pub enum GetPetByIdResponse{
	Status200(Pet),
	Status400,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for GetPetByIdResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for update_pet_with_form
#[derive(Debug)]
pub struct UpdatePetWithFormRequest{
	pub petId:i64,
	pub name:Option<String>,
	pub status:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for UpdatePetWithFormRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for update_pet_with_form
#[derive(Debug)]
pub enum UpdatePetWithFormResponse{
	Status400,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UpdatePetWithFormResponse{
	fn default() -> Self{
		Self::Status400
	}
}
// Request type for delete_pet
#[derive(Debug)]
pub struct DeletePetRequest{
	pub api_key:Option<String>,
	pub petId:i64,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeletePetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for delete_pet
#[derive(Debug)]
pub enum DeletePetResponse{
	Status400,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeletePetResponse{
	fn default() -> Self{
		Self::Status400
	}
}
// Request type for upload_file
#[derive(Debug)]
pub struct UploadFileRequest{
	pub petId:i64,
	pub additionalMetadata:Option<String>,
	pub body: Vec<u8>,
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"petstore_auth": ["write:pets", "read:pets"]}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for UploadFileRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for upload_file
#[derive(Debug)]
pub enum UploadFileResponse{
	Status200(ApiResponse),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UploadFileResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for get_inventory
#[derive(Debug)]
pub struct GetInventoryRequest{
	pub request: axum::http::Request<axum::body::Body>,
	pub security: AuthContext, /*[{"api_key": []}]*/
}
impl AsRef<axum::http::Request<axum::body::Body>> for GetInventoryRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for get_inventory
#[derive(Debug)]
pub enum GetInventoryResponse{
	Status200(HashMap<String,i32>),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for GetInventoryResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for place_order
#[derive(Debug)]
pub struct PlaceOrderRequest{
	pub body: Order,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for PlaceOrderRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for place_order
#[derive(Debug)]
pub enum PlaceOrderResponse{
	Status200(Order),
	Status400,
	Status422,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for PlaceOrderResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for get_order_by_id
#[derive(Debug)]
pub struct GetOrderByIdRequest{
	pub orderId:i64,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for GetOrderByIdRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for get_order_by_id
#[derive(Debug)]
pub enum GetOrderByIdResponse{
	Status200(Order),
	Status400,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for GetOrderByIdResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for delete_order
#[derive(Debug)]
pub struct DeleteOrderRequest{
	pub orderId:i64,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeleteOrderRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for delete_order
#[derive(Debug)]
pub enum DeleteOrderResponse{
	Status400,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeleteOrderResponse{
	fn default() -> Self{
		Self::Status400
	}
}
// Request type for create_user
#[derive(Debug)]
pub struct CreateUserRequest{
	pub body: User,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CreateUserRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for create_user
#[derive(Debug)]
pub enum CreateUserResponse{
	StatusDefault(User),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CreateUserResponse{
	fn default() -> Self{
		Self::StatusDefault(Default::default())
	}
}
// Request type for create_users_with_list_input
#[derive(Debug)]
pub struct CreateUsersWithListInputRequest{
	pub body: Vec<User>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CreateUsersWithListInputRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for create_users_with_list_input
#[derive(Debug)]
pub enum CreateUsersWithListInputResponse{
	Status200(User),
	StatusDefault,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CreateUsersWithListInputResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for login_user
#[derive(Debug)]
pub struct LoginUserRequest{
	pub username:Option<String>,
	pub password:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for LoginUserRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for login_user
#[derive(Debug)]
pub enum LoginUserResponse{
	Status200(String),
	Status400,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for LoginUserResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for logout_user
#[derive(Debug)]
pub struct LogoutUserRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for LogoutUserRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for logout_user
#[derive(Debug)]
pub enum LogoutUserResponse{
	StatusDefault,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for LogoutUserResponse{
	fn default() -> Self{
		Self::StatusDefault
	}
}
// Request type for get_user_by_name
#[derive(Debug)]
pub struct GetUserByNameRequest{
	pub username:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for GetUserByNameRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for get_user_by_name
#[derive(Debug)]
pub enum GetUserByNameResponse{
	Status200(User),
	Status400,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for GetUserByNameResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for update_user
#[derive(Debug)]
pub struct UpdateUserRequest{
	pub username:String,
	pub body: User,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for UpdateUserRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for update_user
#[derive(Debug)]
pub enum UpdateUserResponse{
	StatusDefault,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UpdateUserResponse{
	fn default() -> Self{
		Self::StatusDefault
	}
}
// Request type for delete_user
#[derive(Debug)]
pub struct DeleteUserRequest{
	pub username:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeleteUserRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for delete_user
#[derive(Debug)]
pub enum DeleteUserResponse{
	Status400,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeleteUserResponse{
	fn default() -> Self{
		Self::Status400
	}
}





#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Address{
	pub r#city:Option<String>,
	pub r#state:Option<String>,
	pub r#street:Option<String>,
	pub r#zip:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct ApiResponse{
	pub r#code:Option<i32>,
	pub r#message:Option<String>,
	pub r#type:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Category{
	pub r#id:Option<i64>,
	pub r#name:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Customer{
	pub r#address:Option<Vec<Address>>,
	pub r#id:Option<i64>,
	pub r#username:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Order{
	pub r#complete:Option<bool>,
	pub r#id:Option<i64>,
	pub r#petId:Option<i64>,
	pub r#quantity:Option<i32>,
	pub r#shipDate:Option<chrono::DateTime<chrono::Utc>>,
	pub r#status:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Pet{
	pub r#category:Option<Category>,
	pub r#id:Option<i64>,
	pub r#name:String,
	pub r#photoUrls:Vec<String>,
	pub r#status:Option<String>,
	pub r#tags:Option<Vec<Tag>>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Tag{
	pub r#id:Option<i64>,
	pub r#name:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct User{
	pub r#email:Option<String>,
	pub r#firstName:Option<String>,
	pub r#id:Option<i64>,
	pub r#lastName:Option<String>,
	pub r#password:Option<String>,
	pub r#phone:Option<String>,
	pub r#userStatus:Option<i32>,
	pub r#username:Option<String>,
}




use axum;
use axum::extract::FromRequest;

/// Helper function to generate text responses
fn text_response(code: axum::http::StatusCode, body: String)->axum::response::Response{
	axum::response::Response::builder()
		.status(code)
		.header(axum::http::header::CONTENT_TYPE, "text/plain")
		.body(axum::body::Body::from(body))
		.unwrap()
}

/// Returns axum::Router with root handlers for all operations registered
pub fn axum_router_operations<S: ApiInterface + Sync + Send + 'static>(instance :std::sync::Arc<S>)->axum::Router{
	let router = axum::Router::new();
	let i = instance.clone();
	let router = router.route("/pet", axum::routing::put(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::update_pet(i.as_ref(), UpdatePetRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			UpdatePetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			UpdatePetResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UpdatePetResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UpdatePetResponse::Status422=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(422).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UpdatePetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::add_pet(i.as_ref(), AddPetRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			AddPetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			AddPetResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			AddPetResponse::Status422=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(422).unwrap()).body(axum::body::Body::empty()).unwrap(),
			AddPetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/findByStatus", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::find_pets_by_status(i.as_ref(), FindPetsByStatusRequest{
			r#status:{let v=query.get("status").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			FindPetsByStatusResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			FindPetsByStatusResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			FindPetsByStatusResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/findByTags", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::find_pets_by_tags(i.as_ref(), FindPetsByTagsRequest{
			r#tags:{let v=query.get("tags").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			FindPetsByTagsResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			FindPetsByTagsResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			FindPetsByTagsResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/{petId}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::get_pet_by_id(i.as_ref(), GetPetByIdRequest{
			r#petId:{let v=path.get("petId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: petId in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			GetPetByIdResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			GetPetByIdResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetPetByIdResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetPetByIdResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/{petId}", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::update_pet_with_form(i.as_ref(), UpdatePetWithFormRequest{
			r#petId:{let v=path.get("petId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: petId in path={:?}", path))}},
			r#name:{let v=query.get("name").and_then(|v| v.parse().ok());v},
			r#status:{let v=query.get("status").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			UpdatePetWithFormResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UpdatePetWithFormResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/{petId}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::delete_pet(i.as_ref(), DeletePetRequest{
			r#api_key:{let v=header.get("api_key").map(|v| v.to_str().unwrap_or_default()).and_then(|v| v.parse().ok());v},
			r#petId:{let v=path.get("petId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: petId in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			DeletePetResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeletePetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/pet/{petId}/uploadImage", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::upload_file(i.as_ref(), UploadFileRequest{
			r#petId:{let v=path.get("petId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: petId in path={:?}", path))}},
			r#additionalMetadata:{let v=query.get("additionalMetadata").and_then(|v| v.parse().ok());v},
			body:match axum::body::to_bytes(body, usize::MAX).await{Ok(v)=>v.into(),Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST,format!("{v:?}"))},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			UploadFileResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			UploadFileResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/store/inventory", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::get_inventory(i.as_ref(), GetInventoryRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
			security: match i.as_ref().authorize(axum::http::Request::from_parts(parts.clone(), Default::default())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(axum::http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		match ret{
			GetInventoryResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			GetInventoryResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/store/order", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::place_order(i.as_ref(), PlaceOrderRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			PlaceOrderResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			PlaceOrderResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			PlaceOrderResponse::Status422=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(422).unwrap()).body(axum::body::Body::empty()).unwrap(),
			PlaceOrderResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/store/order/{orderId}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::get_order_by_id(i.as_ref(), GetOrderByIdRequest{
			r#orderId:{let v=path.get("orderId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: orderId in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			GetOrderByIdResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			GetOrderByIdResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetOrderByIdResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetOrderByIdResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/store/order/{orderId}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::delete_order(i.as_ref(), DeleteOrderRequest{
			r#orderId:{let v=path.get("orderId").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: orderId in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeleteOrderResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeleteOrderResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeleteOrderResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::create_user(i.as_ref(), CreateUserRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CreateUserResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CreateUserResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/createWithList", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::create_users_with_list_input(i.as_ref(), CreateUsersWithListInputRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CreateUsersWithListInputResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CreateUsersWithListInputResponse::StatusDefault=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).body(axum::body::Body::empty()).unwrap(),
			CreateUsersWithListInputResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/login", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::login_user(i.as_ref(), LoginUserRequest{
			r#username:{let v=query.get("username").and_then(|v| v.parse().ok());v},
			r#password:{let v=query.get("password").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			LoginUserResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			LoginUserResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			LoginUserResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/logout", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::logout_user(i.as_ref(), LogoutUserRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			LogoutUserResponse::StatusDefault=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).body(axum::body::Body::empty()).unwrap(),
			LogoutUserResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{username}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::get_user_by_name(i.as_ref(), GetUserByNameRequest{
			r#username:{let v=path.get("username").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: username in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			GetUserByNameResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			GetUserByNameResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetUserByNameResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			GetUserByNameResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{username}", axum::routing::put(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::update_user(i.as_ref(), UpdateUserRequest{
			r#username:{let v=path.get("username").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: username in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			UpdateUserResponse::StatusDefault=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UpdateUserResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{username}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::delete_user(i.as_ref(), DeleteUserRequest{
			r#username:{let v=path.get("username").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: username in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeleteUserResponse::Status400=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeleteUserResponse::Status404=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeleteUserResponse::Raw(v)=>v,
		}
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"requestBodies":{"Pet":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"Pet object that needs to be added to the store"},"UserArray":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/User"},"type":"array"}}},"description":"List of user object"}},"schemas":{"Address":{"properties":{"city":{"example":"Palo Alto","type":"string"},"state":{"example":"CA","type":"string"},"street":{"example":"437 Lytton","type":"string"},"zip":{"example":"94301","type":"string"}},"type":"object"},"ApiResponse":{"properties":{"code":{"format":"int32","type":"integer"},"message":{"type":"string"},"type":{"type":"string"}},"type":"object"},"Category":{"properties":{"id":{"example":1,"format":"int64","type":"integer"},"name":{"example":"Dogs","type":"string"}},"type":"object"},"Customer":{"properties":{"address":{"items":{"$ref":"#/components/schemas/Address"},"type":"array"},"id":{"example":100000,"format":"int64","type":"integer"},"username":{"example":"fehguy","type":"string"}},"type":"object"},"Order":{"properties":{"complete":{"type":"boolean"},"id":{"example":10,"format":"int64","type":"integer"},"petId":{"example":198772,"format":"int64","type":"integer"},"quantity":{"example":7,"format":"int32","type":"integer"},"shipDate":{"format":"date-time","type":"string"},"status":{"description":"Order Status","enum":["placed","approved","delivered"],"example":"approved","type":"string"}},"type":"object"},"Pet":{"properties":{"category":{"$ref":"#/components/schemas/Category"},"id":{"example":10,"format":"int64","type":"integer"},"name":{"example":"doggie","type":"string"},"photoUrls":{"items":{"type":"string"},"type":"array"},"status":{"description":"pet status in the store","enum":["available","pending","sold"],"type":"string"},"tags":{"items":{"$ref":"#/components/schemas/Tag"},"type":"array"}},"required":["name","photoUrls"],"type":"object"},"Tag":{"properties":{"id":{"format":"int64","type":"integer"},"name":{"type":"string"}},"type":"object"},"User":{"properties":{"email":{"example":"john@email.com","type":"string"},"firstName":{"example":"John","type":"string"},"id":{"example":10,"format":"int64","type":"integer"},"lastName":{"example":"James","type":"string"},"password":{"example":"12345","type":"string"},"phone":{"example":"12345","type":"string"},"userStatus":{"description":"User Status","example":1,"format":"int32","type":"integer"},"username":{"example":"theUser","type":"string"}},"type":"object"}},"securitySchemes":{"api_key":{"in":"header","name":"api_key","type":"apiKey"},"petstore_auth":{"flows":{"implicit":{"authorizationUrl":"https://petstore3.swagger.io/oauth/authorize","scopes":{"read:pets":"read your pets","write:pets":"modify pets in your account"}}},"type":"oauth2"}}},"externalDocs":{"description":"Find out more about Swagger","url":"http://swagger.io"},"info":{"contact":{"email":"apiteam@swagger.io"},"description":"This is a sample Pet Store Server based on the OpenAPI 3.0 specification.  You can find out more about\nSwagger at [https://swagger.io](https://swagger.io). In the third iteration of the pet store, we\u0027ve switched to the design first approach!\nYou can now help us improve the API whether it\u0027s by making changes to the definition itself or to the code.\nThat way, with time, we can improve the API in general, and expose some of the new features in OAS3.\n\n_If you\u0027re looking for the Swagger 2.0/OAS 2.0 version of Petstore, then click [here](https://editor.swagger.io/?url=https://petstore.swagger.io/v2/swagger.yaml). Alternatively, you can load via the `Edit \u003e Load Petstore OAS 2.0` menu option!_\n\nSome useful links:\n- [The Pet Store repository](https://github.com/swagger-api/swagger-petstore)\n- [The source API definition for the Pet Store](https://github.com/swagger-api/swagger-petstore/blob/master/src/main/resources/openapi.yaml)","license":{"name":"Apache 2.0","url":"http://www.apache.org/licenses/LICENSE-2.0.html"},"termsOfService":"http://swagger.io/terms/","title":"Swagger Petstore - OpenAPI 3.0","version":"1.0.11"},"openapi":"3.0.3","paths":{"/pet":{"post":{"description":"Add a new pet to the store","operationId":"addPet","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/x-www-form-urlencoded":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"Create a new pet in the store","required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"Successful operation"},"400":{"description":"Invalid input"},"422":{"description":"Validation exception"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Add a new pet to the store","tags":["pet"]},"put":{"description":"Update an existing pet by Id","operationId":"updatePet","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/x-www-form-urlencoded":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"Update an existent pet in the store","required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"Successful operation"},"400":{"description":"Invalid ID supplied"},"404":{"description":"Pet not found"},"422":{"description":"Validation exception"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Update an existing pet","tags":["pet"]}},"/pet/findByStatus":{"get":{"description":"Multiple status values can be provided with comma separated strings","operationId":"findPetsByStatus","parameters":[{"description":"Status values that need to be considered for filter","explode":true,"in":"query","name":"status","schema":{"default":"available","enum":["available","pending","sold"],"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Pet"},"type":"array"}},"application/xml":{"schema":{"items":{"$ref":"#/components/schemas/Pet"},"type":"array"}}},"description":"successful operation"},"400":{"description":"Invalid status value"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Finds Pets by status","tags":["pet"]}},"/pet/findByTags":{"get":{"description":"Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.","operationId":"findPetsByTags","parameters":[{"description":"Tags to filter by","explode":true,"in":"query","name":"tags","schema":{"items":{"type":"string"},"type":"array"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Pet"},"type":"array"}},"application/xml":{"schema":{"items":{"$ref":"#/components/schemas/Pet"},"type":"array"}}},"description":"successful operation"},"400":{"description":"Invalid tag value"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Finds Pets by tags","tags":["pet"]}},"/pet/{petId}":{"delete":{"description":"delete a pet","operationId":"deletePet","parameters":[{"description":"","in":"header","name":"api_key","schema":{"type":"string"},"style":"simple"},{"description":"Pet id to delete","in":"path","name":"petId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"}],"responses":{"400":{"description":"Invalid pet value"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Deletes a pet","tags":["pet"]},"get":{"description":"Returns a single pet","operationId":"getPetById","parameters":[{"description":"ID of pet to return","in":"path","name":"petId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Pet"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Pet"}}},"description":"successful operation"},"400":{"description":"Invalid ID supplied"},"404":{"description":"Pet not found"}},"security":[{"api_key":[]},{"petstore_auth":["write:pets","read:pets"]}],"summary":"Find pet by ID","tags":["pet"]},"post":{"description":"","operationId":"updatePetWithForm","parameters":[{"description":"ID of pet that needs to be updated","in":"path","name":"petId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"},{"description":"Name of pet that needs to be updated","in":"query","name":"name","schema":{"type":"string"},"style":"form"},{"description":"Status of pet that needs to be updated","in":"query","name":"status","schema":{"type":"string"},"style":"form"}],"responses":{"400":{"description":"Invalid input"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"Updates a pet in the store with form openapi","tags":["pet"]}},"/pet/{petId}/uploadImage":{"post":{"description":"","operationId":"uploadFile","parameters":[{"description":"ID of pet to update","in":"path","name":"petId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"},{"description":"Additional Metadata","in":"query","name":"additionalMetadata","schema":{"type":"string"},"style":"form"}],"requestBody":{"content":{"application/octet-stream":{"schema":{"format":"binary","type":"string"}}}},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/ApiResponse"}}},"description":"successful operation"}},"security":[{"petstore_auth":["write:pets","read:pets"]}],"summary":"uploads an image","tags":["pet"]}},"/store/inventory":{"get":{"description":"Returns a map of status codes to quantities","operationId":"getInventory","responses":{"200":{"content":{"application/json":{"schema":{"additionalProperties":{"format":"int32","type":"integer"},"type":"object"}}},"description":"successful operation"}},"security":[{"api_key":[]}],"summary":"Returns pet inventories by status","tags":["store"]}},"/store/order":{"post":{"description":"Place a new order in the store","operationId":"placeOrder","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Order"}},"application/x-www-form-urlencoded":{"schema":{"$ref":"#/components/schemas/Order"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Order"}}}},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Order"}}},"description":"successful operation"},"400":{"description":"Invalid input"},"422":{"description":"Validation exception"}},"summary":"Place an order for a pet","tags":["store"]}},"/store/order/{orderId}":{"delete":{"description":"For valid response try integer IDs with value \u003c 1000. Anything above 1000 or nonintegers will generate API errors","operationId":"deleteOrder","parameters":[{"description":"ID of the order that needs to be deleted","in":"path","name":"orderId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"}],"responses":{"400":{"description":"Invalid ID supplied"},"404":{"description":"Order not found"}},"summary":"Delete purchase order by ID","tags":["store"]},"get":{"description":"For valid response try integer IDs with value \u003c= 5 or \u003e 10. Other values will generate exceptions.","operationId":"getOrderById","parameters":[{"description":"ID of order that needs to be fetched","in":"path","name":"orderId","required":true,"schema":{"format":"int64","type":"integer"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Order"}},"application/xml":{"schema":{"$ref":"#/components/schemas/Order"}}},"description":"successful operation"},"400":{"description":"Invalid ID supplied"},"404":{"description":"Order not found"}},"summary":"Find purchase order by ID","tags":["store"]}},"/user":{"post":{"description":"This can only be done by the logged in user.","operationId":"createUser","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}},"application/x-www-form-urlencoded":{"schema":{"$ref":"#/components/schemas/User"}},"application/xml":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"Created user object"},"responses":{"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}},"application/xml":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"successful operation"}},"summary":"Create user","tags":["user"]}},"/user/createWithList":{"post":{"description":"Creates list of users with given input array","operationId":"createUsersWithListInput","requestBody":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/User"},"type":"array"}}}},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}},"application/xml":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"Successful operation"},"default":{"description":"successful operation"}},"summary":"Creates list of users with given input array","tags":["user"]}},"/user/login":{"get":{"description":"","operationId":"loginUser","parameters":[{"description":"The user name for login","in":"query","name":"username","schema":{"type":"string"},"style":"form"},{"description":"The password for login in clear text","in":"query","name":"password","schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}},"application/xml":{"schema":{"type":"string"}}},"description":"successful operation","headers":{"X-Expires-After":{"description":"date in UTC when token expires","schema":{"format":"date-time","type":"string"},"style":"simple"},"X-Rate-Limit":{"description":"calls per hour allowed by the user","schema":{"format":"int32","type":"integer"},"style":"simple"}}},"400":{"description":"Invalid username/password supplied"}},"summary":"Logs user into the system","tags":["user"]}},"/user/logout":{"get":{"description":"","operationId":"logoutUser","responses":{"default":{"description":"successful operation"}},"summary":"Logs out current logged in user session","tags":["user"]}},"/user/{username}":{"delete":{"description":"This can only be done by the logged in user.","operationId":"deleteUser","parameters":[{"description":"The name that needs to be deleted","in":"path","name":"username","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"400":{"description":"Invalid username supplied"},"404":{"description":"User not found"}},"summary":"Delete user","tags":["user"]},"get":{"description":"","operationId":"getUserByName","parameters":[{"description":"The name that needs to be fetched. Use user1 for testing. ","in":"path","name":"username","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}},"application/xml":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"successful operation"},"400":{"description":"Invalid username supplied"},"404":{"description":"User not found"}},"summary":"Get user by user name","tags":["user"]},"put":{"description":"This can only be done by the logged in user.","operationId":"updateUser","parameters":[{"description":"name that need to be deleted","in":"path","name":"username","required":true,"schema":{"type":"string"},"style":"simple"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}},"application/x-www-form-urlencoded":{"schema":{"$ref":"#/components/schemas/User"}},"application/xml":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"Update an existent user in the store"},"responses":{"default":{"description":"successful operation"}},"summary":"Update user","tags":["user"]}}},"servers":[{"url":"https://petstore3.swagger.io/api/v3"}],"tags":[{"description":"Everything about your Pets","externalDocs":{"description":"Find out more","url":"http://swagger.io"},"name":"pet"},{"description":"Access to Petstore orders","externalDocs":{"description":"Find out more about our store","url":"http://swagger.io"},"name":"store"},{"description":"Operations about user","name":"user"}]}"###
		}))
		.route("/ui", axum::routing::get(|| async move{
			axum::response::Html(r###"
			<html lang="en">
			<head>
			  <meta charset="utf-8" />
			  <meta name="viewport" content="width=device-width, initial-scale=1" />
			  <meta name="description" content="SwaggerUI" />
			  <title>SwaggerUI</title>
			  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css" />
			</head>
			<body>
			<div id="swagger-ui"></div>
			<script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-bundle.js" crossorigin></script>
			<script>
			  window.onload = () => {
				window.ui = SwaggerUIBundle({
				  url: location.href.replace("/ui","/openapi.json"),
				  dom_id: '#swagger-ui',
				});
			  };
			</script>
			</body>
			</html>
			"###)
		}));
	return router;
}

/// Mount the router to the server's URL prefix with nest_service
pub fn axum_router<S: ApiInterface + Sync + Send + 'static>(instance: S)->axum::Router{
	let instance_arc=std::sync::Arc::new(instance);
	let mut router = axum::Router::new();
	router = router.nest_service("/api/v3", axum_router_operations(instance_arc.clone()));
	router
}

/// Display the server URL list to standard output
pub fn print_axum_router(port:u16){
	println!("http://localhost:{}/api/v3/ui", port);
}

/// Test server implementation (all methods return default values)
pub struct TestServer{}
impl ApiInterface for TestServer{
	// Implement required methods here
	// PUT /pet
	// async fn update_pet(&self, _req: UpdatePetRequest) -> UpdatePetResponse{Default::default()}
	// POST /pet
	// async fn add_pet(&self, _req: AddPetRequest) -> AddPetResponse{Default::default()}
	// GET /pet/findByStatus
	// async fn find_pets_by_status(&self, _req: FindPetsByStatusRequest) -> FindPetsByStatusResponse{Default::default()}
	// GET /pet/findByTags
	// async fn find_pets_by_tags(&self, _req: FindPetsByTagsRequest) -> FindPetsByTagsResponse{Default::default()}
	// GET /pet/{petId}
	// async fn get_pet_by_id(&self, _req: GetPetByIdRequest) -> GetPetByIdResponse{Default::default()}
	// POST /pet/{petId}
	// async fn update_pet_with_form(&self, _req: UpdatePetWithFormRequest) -> UpdatePetWithFormResponse{Default::default()}
	// DELETE /pet/{petId}
	// async fn delete_pet(&self, _req: DeletePetRequest) -> DeletePetResponse{Default::default()}
	// POST /pet/{petId}/uploadImage
	// async fn upload_file(&self, _req: UploadFileRequest) -> UploadFileResponse{Default::default()}
	// GET /store/inventory
	// async fn get_inventory(&self, _req: GetInventoryRequest) -> GetInventoryResponse{Default::default()}
	// POST /store/order
	// async fn place_order(&self, _req: PlaceOrderRequest) -> PlaceOrderResponse{Default::default()}
	// GET /store/order/{orderId}
	// async fn get_order_by_id(&self, _req: GetOrderByIdRequest) -> GetOrderByIdResponse{Default::default()}
	// DELETE /store/order/{orderId}
	// async fn delete_order(&self, _req: DeleteOrderRequest) -> DeleteOrderResponse{Default::default()}
	// POST /user
	// async fn create_user(&self, _req: CreateUserRequest) -> CreateUserResponse{Default::default()}
	// POST /user/createWithList
	// async fn create_users_with_list_input(&self, _req: CreateUsersWithListInputRequest) -> CreateUsersWithListInputResponse{Default::default()}
	// GET /user/login
	// async fn login_user(&self, _req: LoginUserRequest) -> LoginUserResponse{Default::default()}
	// GET /user/logout
	// async fn logout_user(&self, _req: LogoutUserRequest) -> LogoutUserResponse{Default::default()}
	// GET /user/{username}
	// async fn get_user_by_name(&self, _req: GetUserByNameRequest) -> GetUserByNameResponse{Default::default()}
	// PUT /user/{username}
	// async fn update_user(&self, _req: UpdateUserRequest) -> UpdateUserResponse{Default::default()}
	// DELETE /user/{username}
	// async fn delete_user(&self, _req: DeleteUserRequest) -> DeleteUserResponse{Default::default()}
}

/// Estimates the origin URL (scheme://host) from an HTTP request
/// Priority: Forwarded > X-Forwarded-* > Host
pub fn origin_from_request<B>(req: &axum::http::Request<B>) -> Option<String> {
	fn first_csv(s: &str) -> &str {
		s.split(',').next().unwrap_or(s).trim()
	}
	fn unquote(s: &str) -> &str {
		let s = s.trim();
		if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
			&s[1..s.len() - 1]
		} else {
			s
		}
	}
	fn guess_scheme(host: &str) -> &'static str {
		let hostname = host
			.trim_start_matches('[')
			.split(']')
			.next()
			.unwrap_or(host)
			.split(':')
			.next()
			.unwrap_or(host);
		match hostname {
			"localhost" | "127.0.0.1" | "::1" => "http",
			_ => "https",
		}
	}
	fn mk_origin(proto: Option<String>, host: String) -> String {
		let proto = proto.unwrap_or_else(|| guess_scheme(&host).to_string());
		format!("{proto}://{host}")
	}

	let headers = req.headers();

	// 0) Check URI authority (for absolute URIs)
	if let Some(auth) = req.uri().authority() {
		let host = auth.as_str().to_string();
		return Some(mk_origin(None, host));
	}

	// 1) Forwarded (RFC 7239)
	if let Some(raw) = headers
		.get(axum::http::header::FORWARDED)
		.and_then(|v| v.to_str().ok())
	{
		let first = first_csv(raw);
		let mut proto: Option<String> = None;
		let mut host: Option<String> = None;

		for part in first.split(';') {
			let mut it = part.trim().splitn(2, '=');
			let k = it.next().unwrap_or("").trim().to_ascii_lowercase();
			let v = unquote(it.next().unwrap_or(""));

			match k.as_str() {
				"proto" if !v.is_empty() => proto = Some(v.to_ascii_lowercase()),
				"host" if !v.is_empty() => host = Some(v.to_string()),
				_ => {}
			}
		}

		if let Some(host) = host {
			return Some(mk_origin(proto, host));
		}
	}

	// 2) X-Forwarded-*
	if let Some(mut host) = headers
		.get("x-forwarded-host")
		.and_then(|v| v.to_str().ok())
		.map(first_csv)
		.filter(|s| !s.is_empty())
		.map(str::to_string)
	{
		if !host.contains(':') {
			if let Some(port) = headers
				.get("x-forwarded-port")
				.and_then(|v| v.to_str().ok())
				.map(str::trim)
				.filter(|s| !s.is_empty())
			{
				host = format!("{host}:{port}");
			}
		}

		let proto = headers
			.get("x-forwarded-proto")
			.and_then(|v| v.to_str().ok())
			.map(first_csv)
			.map(|s| s.to_ascii_lowercase())
			.filter(|s| !s.is_empty());

		return Some(mk_origin(proto, host));
	}

	// 3) Fallback to Host header
	let host = headers
		.get(axum::http::header::HOST)
		.and_then(|h| h.to_str().ok())
		.map(str::trim)
		.filter(|s| !s.is_empty())?
		.to_string();

	Some(format!("{}://{}", guess_scheme(&host), host))
}

#[tokio::main]
async fn main() {
	let port:u16 = std::env::var("PORT").unwrap_or("8080".to_string()).parse().expect("PORT should be integer");
	print_axum_router(port);
	let api = TestServer{};
	let app = axum_router(api).layer(axum::extract::DefaultBodyLimit::disable());
	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
	axum::serve(listener, app)
		.with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
		.await
		.unwrap();
}
