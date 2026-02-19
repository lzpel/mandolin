












































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
	// DELETE /clean
	fn operation_clean(&self, _req: OperationCleanRequest) -> impl Future<Output = OperationCleanResponse> + Send{async{Default::default()}}
	// GET /credential/iam
	fn credential_get(&self, _req: CredentialGetRequest) -> impl Future<Output = CredentialGetResponse> + Send{async{Default::default()}}
	// POST /credential/sign_in
	fn credential_sign_in(&self, _req: CredentialSignInRequest) -> impl Future<Output = CredentialSignInResponse> + Send{async{Default::default()}}
	// POST /credential/sign_up
	fn credential_sign_up(&self, _req: CredentialSignUpRequest) -> impl Future<Output = CredentialSignUpResponse> + Send{async{Default::default()}}
	// GET /credential/validate
	fn credential_validate(&self, _req: CredentialValidateRequest) -> impl Future<Output = CredentialValidateResponse> + Send{async{Default::default()}}
	// GET /device
	fn device_list_get(&self, _req: DeviceListGetRequest) -> impl Future<Output = DeviceListGetResponse> + Send{async{Default::default()}}
	// POST /device
	fn device_list_add(&self, _req: DeviceListAddRequest) -> impl Future<Output = DeviceListAddResponse> + Send{async{Default::default()}}
	// GET /device/{key}
	fn device_get(&self, _req: DeviceGetRequest) -> impl Future<Output = DeviceGetResponse> + Send{async{Default::default()}}
	// POST /device/{key}
	fn device_update(&self, _req: DeviceUpdateRequest) -> impl Future<Output = DeviceUpdateResponse> + Send{async{Default::default()}}
	// DELETE /device/{key}
	fn device_delete(&self, _req: DeviceDeleteRequest) -> impl Future<Output = DeviceDeleteResponse> + Send{async{Default::default()}}
	// GET /key/{key}
	fn operation_key(&self, _req: OperationKeyRequest) -> impl Future<Output = OperationKeyResponse> + Send{async{Default::default()}}
	// POST /ml/image
	fn ml_image(&self, _req: MlImageRequest) -> impl Future<Output = MlImageResponse> + Send{async{Default::default()}}
	// POST /ml/json
	fn ml_json(&self, _req: MlJsonRequest) -> impl Future<Output = MlJsonResponse> + Send{async{Default::default()}}
	// GET /record
	fn record_list_get(&self, _req: RecordListGetRequest) -> impl Future<Output = RecordListGetResponse> + Send{async{Default::default()}}
	// POST /record
	fn record_list_add(&self, _req: RecordListAddRequest) -> impl Future<Output = RecordListAddResponse> + Send{async{Default::default()}}
	// GET /record/{key}
	fn record_get(&self, _req: RecordGetRequest) -> impl Future<Output = RecordGetResponse> + Send{async{Default::default()}}
	// POST /record/{key}
	fn record_update(&self, _req: RecordUpdateRequest) -> impl Future<Output = RecordUpdateResponse> + Send{async{Default::default()}}
	// DELETE /record/{key}
	fn record_delete(&self, _req: RecordDeleteRequest) -> impl Future<Output = RecordDeleteResponse> + Send{async{Default::default()}}
	// GET /test
	fn test_b(&self, _req: TestBRequest) -> impl Future<Output = TestBResponse> + Send{async{Default::default()}}
	// PUT /test
	fn test_d(&self, _req: TestDRequest) -> impl Future<Output = TestDResponse> + Send{async{Default::default()}}
	// POST /test
	fn test_c(&self, _req: TestCRequest) -> impl Future<Output = TestCResponse> + Send{async{Default::default()}}
	// DELETE /test
	fn test_a(&self, _req: TestARequest) -> impl Future<Output = TestAResponse> + Send{async{Default::default()}}
	// GET /user
	fn user_list_get(&self, _req: UserListGetRequest) -> impl Future<Output = UserListGetResponse> + Send{async{Default::default()}}
	// GET /user/{key}
	fn user_get(&self, _req: UserGetRequest) -> impl Future<Output = UserGetResponse> + Send{async{Default::default()}}
	// POST /user/{key}
	fn user_update(&self, _req: UserUpdateRequest) -> impl Future<Output = UserUpdateResponse> + Send{async{Default::default()}}
	// DELETE /user/{key}
	fn user_delete(&self, _req: UserDeleteRequest) -> impl Future<Output = UserDeleteResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for operation_clean
#[derive(Debug)]
pub struct OperationCleanRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for OperationCleanRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for operation_clean
#[derive(Debug)]
pub enum OperationCleanResponse{
	Status200(String),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for OperationCleanResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for credential_get
#[derive(Debug)]
pub struct CredentialGetRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CredentialGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for credential_get
#[derive(Debug)]
pub enum CredentialGetResponse{
	Status200(User),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CredentialGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for credential_sign_in
#[derive(Debug)]
pub struct CredentialSignInRequest{
	pub body: PathsCredentialSignInPostRequestBodyContentApplicationJsonSchema,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CredentialSignInRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for credential_sign_in
#[derive(Debug)]
pub enum CredentialSignInResponse{
	Status200(String),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CredentialSignInResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for credential_sign_up
#[derive(Debug)]
pub struct CredentialSignUpRequest{
	pub body: User,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CredentialSignUpRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for credential_sign_up
#[derive(Debug)]
pub enum CredentialSignUpResponse{
	Status200(String),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CredentialSignUpResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for credential_validate
#[derive(Debug)]
pub struct CredentialValidateRequest{
	pub body: String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for CredentialValidateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for credential_validate
#[derive(Debug)]
pub enum CredentialValidateResponse{
	Status200(Apikey),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for CredentialValidateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for device_list_get
#[derive(Debug)]
pub struct DeviceListGetRequest{
	pub limit:Option<i32>,
	pub order:Option<String>,
	pub max_key:Option<String>,
	pub min_key:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeviceListGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for device_list_get
#[derive(Debug)]
pub enum DeviceListGetResponse{
	Status200(Vec<Device>),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeviceListGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for device_list_add
#[derive(Debug)]
pub struct DeviceListAddRequest{
	pub body: Device,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeviceListAddRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for device_list_add
#[derive(Debug)]
pub enum DeviceListAddResponse{
	Status200(Device),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeviceListAddResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for device_get
#[derive(Debug)]
pub struct DeviceGetRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeviceGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for device_get
#[derive(Debug)]
pub enum DeviceGetResponse{
	Status200(Device),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeviceGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for device_update
#[derive(Debug)]
pub struct DeviceUpdateRequest{
	pub key:String,
	pub body: Device,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeviceUpdateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for device_update
#[derive(Debug)]
pub enum DeviceUpdateResponse{
	Status200(Device),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeviceUpdateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for device_delete
#[derive(Debug)]
pub struct DeviceDeleteRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for DeviceDeleteRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for device_delete
#[derive(Debug)]
pub enum DeviceDeleteResponse{
	Status200,
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for DeviceDeleteResponse{
	fn default() -> Self{
		Self::Status200
	}
}
// Request type for operation_key
#[derive(Debug)]
pub struct OperationKeyRequest{
	pub key:String,
	pub seconds:Option<i64>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for OperationKeyRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for operation_key
#[derive(Debug)]
pub enum OperationKeyResponse{
	Status200(String),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for OperationKeyResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for ml_image
#[derive(Debug)]
pub struct MlImageRequest{
	pub body: Vec<u8>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for MlImageRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for ml_image
#[derive(Debug)]
pub enum MlImageResponse{
	Status200(Vec<u8>),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for MlImageResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for ml_json
#[derive(Debug)]
pub struct MlJsonRequest{
	pub body: Vec<u8>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for MlJsonRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for ml_json
#[derive(Debug)]
pub enum MlJsonResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for MlJsonResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for record_list_get
#[derive(Debug)]
pub struct RecordListGetRequest{
	pub limit:Option<i32>,
	pub order:Option<String>,
	pub key_device:String,
	pub max_key:Option<String>,
	pub min_key:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for RecordListGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for record_list_get
#[derive(Debug)]
pub enum RecordListGetResponse{
	Status200(Vec<Record>),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for RecordListGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for record_list_add
#[derive(Debug)]
pub struct RecordListAddRequest{
	pub key_device:String,
	pub latitude:f64,
	pub longitude:f64,
	pub seconds:Option<u64>,
	pub body: Vec<u8>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for RecordListAddRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for record_list_add
#[derive(Debug)]
pub enum RecordListAddResponse{
	Status200(Record),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for RecordListAddResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for record_get
#[derive(Debug)]
pub struct RecordGetRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for RecordGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for record_get
#[derive(Debug)]
pub enum RecordGetResponse{
	Status200(Record),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for RecordGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for record_update
#[derive(Debug)]
pub struct RecordUpdateRequest{
	pub key:String,
	pub body: Record,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for RecordUpdateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for record_update
#[derive(Debug)]
pub enum RecordUpdateResponse{
	Status200(Record),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for RecordUpdateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for record_delete
#[derive(Debug)]
pub struct RecordDeleteRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for RecordDeleteRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for record_delete
#[derive(Debug)]
pub enum RecordDeleteResponse{
	Status200,
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for RecordDeleteResponse{
	fn default() -> Self{
		Self::Status200
	}
}
// Request type for test_b
#[derive(Debug)]
pub struct TestBRequest{
	pub limit:Option<i32>,
	pub cursor:Option<i32>,
	pub order:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for TestBRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for test_b
#[derive(Debug)]
pub enum TestBResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for TestBResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for test_d
#[derive(Debug)]
pub struct TestDRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for TestDRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for test_d
#[derive(Debug)]
pub enum TestDResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for TestDResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for test_c
#[derive(Debug)]
pub struct TestCRequest{
	pub limit:Option<i32>,
	pub cursor:Option<i32>,
	pub order:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for TestCRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for test_c
#[derive(Debug)]
pub enum TestCResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for TestCResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for test_a
#[derive(Debug)]
pub struct TestARequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for TestARequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for test_a
#[derive(Debug)]
pub enum TestAResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for TestAResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for user_list_get
#[derive(Debug)]
pub struct UserListGetRequest{
	pub limit:Option<i32>,
	pub order:Option<String>,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for UserListGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for user_list_get
#[derive(Debug)]
pub enum UserListGetResponse{
	Status200(Vec<User>),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UserListGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for user_get
#[derive(Debug)]
pub struct UserGetRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for UserGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for user_get
#[derive(Debug)]
pub enum UserGetResponse{
	Status200(User),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UserGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for user_update
#[derive(Debug)]
pub struct UserUpdateRequest{
	pub key:String,
	pub body: User,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for UserUpdateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for user_update
#[derive(Debug)]
pub enum UserUpdateResponse{
	Status200(User),
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UserUpdateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for user_delete
#[derive(Debug)]
pub struct UserDeleteRequest{
	pub key:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for UserDeleteRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for user_delete
#[derive(Debug)]
pub enum UserDeleteResponse{
	Status200,
	Status400(String),
	Status404(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for UserDeleteResponse{
	fn default() -> Self{
		Self::Status200
	}
}





#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Apikey{
	pub r#expiration:u64,
	pub r#user:User,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Device{
	pub r#key:String,
	pub r#key_user:String,
	pub r#latitude:f64,
	pub r#longitude:f64,
	pub r#name:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Position{
	pub r#latitude:f64,
	pub r#longitude:f64,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Record{
	pub r#bbox:Vec<u64>,
	pub r#key:String,
	pub r#key_device:String,
	pub r#label:Vec<String>,
	pub r#latitude:f64,
	pub r#longitude:f64,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct User{
	pub r#auth:Vec<String>,
	pub r#email:String,
	pub r#key:String,
	pub r#name:String,
}



#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsCredentialSignInPostRequestBodyContentApplicationJsonSchema{
	pub r#has_auth:String,
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
	let router = router.route("/clean", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::operation_clean(i.as_ref(), OperationCleanRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			OperationCleanResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			OperationCleanResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			OperationCleanResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/credential/iam", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::credential_get(i.as_ref(), CredentialGetRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CredentialGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CredentialGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/credential/sign_in", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::credential_sign_in(i.as_ref(), CredentialSignInRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CredentialSignInResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CredentialSignInResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialSignInResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialSignInResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/credential/sign_up", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::credential_sign_up(i.as_ref(), CredentialSignUpRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CredentialSignUpResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CredentialSignUpResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialSignUpResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialSignUpResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/credential/validate", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::credential_validate(i.as_ref(), CredentialValidateRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			CredentialValidateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			CredentialValidateResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			CredentialValidateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/device", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::device_list_get(i.as_ref(), DeviceListGetRequest{
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			r#order:{let v=query.get("order").and_then(|v| v.parse().ok());v},
			r#max_key:{let v=query.get("max_key").and_then(|v| v.parse().ok());v},
			r#min_key:{let v=query.get("min_key").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeviceListGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			DeviceListGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceListGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceListGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/device", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::device_list_add(i.as_ref(), DeviceListAddRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeviceListAddResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			DeviceListAddResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceListAddResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/device/{key}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::device_get(i.as_ref(), DeviceGetRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeviceGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			DeviceGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/device/{key}", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::device_update(i.as_ref(), DeviceUpdateRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeviceUpdateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			DeviceUpdateResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceUpdateResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceUpdateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/device/{key}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::device_delete(i.as_ref(), DeviceDeleteRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			DeviceDeleteResponse::Status200=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).body(axum::body::Body::empty()).unwrap(),
			DeviceDeleteResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceDeleteResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			DeviceDeleteResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/key/{key}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::operation_key(i.as_ref(), OperationKeyRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			r#seconds:{let v=query.get("seconds").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			OperationKeyResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			OperationKeyResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			OperationKeyResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/ml/image", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::ml_image(i.as_ref(), MlImageRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await{Ok(v)=>v.into(),Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST,format!("{v:?}"))},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			MlImageResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "image/png").body(axum::body::Body::from(v)).unwrap(),
			MlImageResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/ml/json", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::ml_json(i.as_ref(), MlJsonRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await{Ok(v)=>v.into(),Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST,format!("{v:?}"))},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			MlJsonResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			MlJsonResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/record", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::record_list_get(i.as_ref(), RecordListGetRequest{
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			r#order:{let v=query.get("order").and_then(|v| v.parse().ok());v},
			r#key_device:{let v=query.get("key_device").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key_device in query={:?}", query))}},
			r#max_key:{let v=query.get("max_key").and_then(|v| v.parse().ok());v},
			r#min_key:{let v=query.get("min_key").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			RecordListGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			RecordListGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordListGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/record", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::record_list_add(i.as_ref(), RecordListAddRequest{
			r#key_device:{let v=query.get("key_device").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key_device in query={:?}", query))}},
			r#latitude:{let v=query.get("latitude").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: latitude in query={:?}", query))}},
			r#longitude:{let v=query.get("longitude").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: longitude in query={:?}", query))}},
			r#seconds:{let v=query.get("seconds").and_then(|v| v.parse().ok());v},
			body:match axum::body::to_bytes(body, usize::MAX).await{Ok(v)=>v.into(),Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST,format!("{v:?}"))},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			RecordListAddResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			RecordListAddResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordListAddResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/record/{key}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::record_get(i.as_ref(), RecordGetRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			RecordGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			RecordGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/record/{key}", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::record_update(i.as_ref(), RecordUpdateRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			RecordUpdateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			RecordUpdateResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordUpdateResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordUpdateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/record/{key}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::record_delete(i.as_ref(), RecordDeleteRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			RecordDeleteResponse::Status200=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).body(axum::body::Body::empty()).unwrap(),
			RecordDeleteResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordDeleteResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			RecordDeleteResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/test", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::test_b(i.as_ref(), TestBRequest{
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			r#cursor:{let v=query.get("cursor").and_then(|v| v.parse().ok());v},
			r#order:{let v=query.get("order").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: order in query={:?}", query))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			TestBResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			TestBResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/test", axum::routing::put(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::test_d(i.as_ref(), TestDRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			TestDResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			TestDResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/test", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::test_c(i.as_ref(), TestCRequest{
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			r#cursor:{let v=query.get("cursor").and_then(|v| v.parse().ok());v},
			r#order:{let v=query.get("order").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: order in query={:?}", query))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			TestCResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			TestCResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/test", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::test_a(i.as_ref(), TestARequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			TestAResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			TestAResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::user_list_get(i.as_ref(), UserListGetRequest{
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			r#order:{let v=query.get("order").and_then(|v| v.parse().ok());v},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			UserListGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			UserListGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserListGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserListGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{key}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::user_get(i.as_ref(), UserGetRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			UserGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			UserGetResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserGetResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{key}", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::user_update(i.as_ref(), UserUpdateRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			UserUpdateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			UserUpdateResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserUpdateResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserUpdateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/user/{key}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::user_delete(i.as_ref(), UserDeleteRequest{
			r#key:{let v=path.get("key").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: key in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			UserDeleteResponse::Status200=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).body(axum::body::Body::empty()).unwrap(),
			UserDeleteResponse::Status400(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(400).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserDeleteResponse::Status404(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(404).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			UserDeleteResponse::Raw(v)=>v,
		}
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"schemas":{"Apikey":{"properties":{"expiration":{"format":"uint64","type":"integer"},"user":{"$ref":"#/components/schemas/User"}},"required":["user","expiration"],"type":"object"},"Device":{"properties":{"key":{"type":"string"},"key_user":{"type":"string"},"latitude":{"type":"number"},"longitude":{"type":"number"},"name":{"type":"string"}},"required":["key","key_user","name","latitude","longitude"],"type":"object"},"Position":{"properties":{"latitude":{"type":"number"},"longitude":{"type":"number"}},"required":["latitude","longitude"],"type":"object"},"Record":{"properties":{"bbox":{"items":{"format":"uint64","type":"integer"},"type":"array"},"key":{"type":"string"},"key_device":{"type":"string"},"label":{"items":{"type":"string"},"type":"array"},"latitude":{"type":"number"},"longitude":{"type":"number"}},"required":["key","key_device","label","bbox","latitude","longitude"],"type":"object"},"User":{"properties":{"auth":{"items":{"type":"string"},"type":"array"},"email":{"type":"string"},"key":{"type":"string"},"name":{"type":"string"}},"required":["key","name","email","auth"],"type":"object"}}},"info":{"title":"plant mimamori","version":"0.0.0"},"openapi":"3.0.0","paths":{"/clean":{"delete":{"operationId":"operation_clean","responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/credential/iam":{"get":{"operationId":"credential_get","responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/credential/sign_in":{"post":{"operationId":"credential_sign_in","requestBody":{"content":{"application/json":{"schema":{"properties":{"has_auth":{"type":"string"}},"required":["has_auth"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/credential/sign_up":{"post":{"operationId":"credential_sign_up","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/credential/validate":{"get":{"operationId":"credential_validate","requestBody":{"content":{"application/json":{"schema":{"type":"string"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Apikey"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/device":{"get":{"operationId":"device_list_get","parameters":[{"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"order","schema":{"type":"string"},"style":"form"},{"in":"query","name":"max_key","schema":{"type":"string"},"style":"form"},{"in":"query","name":"min_key","schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Device"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"post":{"operationId":"device_list_add","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Device"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Device"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/device/{key}":{"delete":{"operationId":"device_delete","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"get":{"operationId":"device_get","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Device"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"post":{"operationId":"device_update","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Device"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Device"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/key/{key}":{"get":{"operationId":"operation_key","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"},{"in":"query","name":"seconds","schema":{"format":"int64","type":"integer"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/ml/image":{"post":{"operationId":"ml_image","requestBody":{"content":{"image/*":{"schema":{"format":"binary","type":"string"}}},"required":true},"responses":{"200":{"content":{"image/png":{"schema":{"format":"binary","type":"string"}}},"description":"The request has succeeded."}}}},"/ml/json":{"post":{"operationId":"ml_json","requestBody":{"content":{"image/*":{"schema":{"format":"binary","type":"string"}}},"required":true},"responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}}},"/record":{"get":{"operationId":"record_list_get","parameters":[{"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"order","schema":{"type":"string"},"style":"form"},{"in":"query","name":"key_device","required":true,"schema":{"type":"string"},"style":"form"},{"in":"query","name":"max_key","schema":{"type":"string"},"style":"form"},{"in":"query","name":"min_key","schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Record"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}},"post":{"operationId":"record_list_add","parameters":[{"in":"query","name":"key_device","required":true,"schema":{"type":"string"},"style":"form"},{"in":"query","name":"latitude","required":true,"schema":{"type":"number"},"style":"form"},{"in":"query","name":"longitude","required":true,"schema":{"type":"number"},"style":"form"},{"in":"query","name":"seconds","schema":{"format":"uint64","type":"integer"},"style":"form"}],"requestBody":{"content":{"image/*":{"schema":{"format":"binary","type":"string"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Record"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/record/{key}":{"delete":{"operationId":"record_delete","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"get":{"operationId":"record_get","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Record"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"post":{"operationId":"record_update","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Record"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Record"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/test":{"delete":{"operationId":"test_a","responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}},"get":{"operationId":"test_b","parameters":[{"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"cursor","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"order","required":true,"schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}},"post":{"operationId":"test_c","parameters":[{"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"cursor","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"order","required":true,"schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}},"put":{"operationId":"test_d","responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}}},"/user":{"get":{"operationId":"user_list_get","parameters":[{"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"},{"in":"query","name":"order","schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/User"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}},"/user/{key}":{"delete":{"operationId":"user_delete","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"get":{"operationId":"user_get","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}},"post":{"operationId":"user_update","parameters":[{"in":"path","name":"key","required":true,"schema":{"type":"string"},"style":"simple"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server cannot find the requested resource."}}}}},"servers":[{"description":"開発用","url":"/api","variables":{}}]}"###
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
	router = router.nest_service("/api", axum_router_operations(instance_arc.clone()));
	router
}

/// Display the server URL list to standard output
pub fn print_axum_router(port:u16){
	println!("http://localhost:{}/api/ui", port);
}

/// Test server implementation (all methods return default values)
pub struct TestServer{}
impl ApiInterface for TestServer{
	// Implement required methods here
	// DELETE /clean
	// async fn operation_clean(&self, _req: OperationCleanRequest) -> OperationCleanResponse{Default::default()}
	// GET /credential/iam
	// async fn credential_get(&self, _req: CredentialGetRequest) -> CredentialGetResponse{Default::default()}
	// POST /credential/sign_in
	// async fn credential_sign_in(&self, _req: CredentialSignInRequest) -> CredentialSignInResponse{Default::default()}
	// POST /credential/sign_up
	// async fn credential_sign_up(&self, _req: CredentialSignUpRequest) -> CredentialSignUpResponse{Default::default()}
	// GET /credential/validate
	// async fn credential_validate(&self, _req: CredentialValidateRequest) -> CredentialValidateResponse{Default::default()}
	// GET /device
	// async fn device_list_get(&self, _req: DeviceListGetRequest) -> DeviceListGetResponse{Default::default()}
	// POST /device
	// async fn device_list_add(&self, _req: DeviceListAddRequest) -> DeviceListAddResponse{Default::default()}
	// GET /device/{key}
	// async fn device_get(&self, _req: DeviceGetRequest) -> DeviceGetResponse{Default::default()}
	// POST /device/{key}
	// async fn device_update(&self, _req: DeviceUpdateRequest) -> DeviceUpdateResponse{Default::default()}
	// DELETE /device/{key}
	// async fn device_delete(&self, _req: DeviceDeleteRequest) -> DeviceDeleteResponse{Default::default()}
	// GET /key/{key}
	// async fn operation_key(&self, _req: OperationKeyRequest) -> OperationKeyResponse{Default::default()}
	// POST /ml/image
	// async fn ml_image(&self, _req: MlImageRequest) -> MlImageResponse{Default::default()}
	// POST /ml/json
	// async fn ml_json(&self, _req: MlJsonRequest) -> MlJsonResponse{Default::default()}
	// GET /record
	// async fn record_list_get(&self, _req: RecordListGetRequest) -> RecordListGetResponse{Default::default()}
	// POST /record
	// async fn record_list_add(&self, _req: RecordListAddRequest) -> RecordListAddResponse{Default::default()}
	// GET /record/{key}
	// async fn record_get(&self, _req: RecordGetRequest) -> RecordGetResponse{Default::default()}
	// POST /record/{key}
	// async fn record_update(&self, _req: RecordUpdateRequest) -> RecordUpdateResponse{Default::default()}
	// DELETE /record/{key}
	// async fn record_delete(&self, _req: RecordDeleteRequest) -> RecordDeleteResponse{Default::default()}
	// GET /test
	// async fn test_b(&self, _req: TestBRequest) -> TestBResponse{Default::default()}
	// PUT /test
	// async fn test_d(&self, _req: TestDRequest) -> TestDResponse{Default::default()}
	// POST /test
	// async fn test_c(&self, _req: TestCRequest) -> TestCResponse{Default::default()}
	// DELETE /test
	// async fn test_a(&self, _req: TestARequest) -> TestAResponse{Default::default()}
	// GET /user
	// async fn user_list_get(&self, _req: UserListGetRequest) -> UserListGetResponse{Default::default()}
	// GET /user/{key}
	// async fn user_get(&self, _req: UserGetRequest) -> UserGetResponse{Default::default()}
	// POST /user/{key}
	// async fn user_update(&self, _req: UserUpdateRequest) -> UserUpdateResponse{Default::default()}
	// DELETE /user/{key}
	// async fn user_delete(&self, _req: UserDeleteRequest) -> UserDeleteResponse{Default::default()}
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
