












































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
	fn authorize(&self, _req: http::Request<()>) -> impl Future<Output = Result<AuthContext, String>> + Send{async { Ok(Default::default()) } }
	// GET /auth
	fn auth_api_user_get(&self, _req: AuthApiUserGetRequest) -> impl Future<Output = AuthApiUserGetResponse> + Send{async{Default::default()}}
	// GET /auth/out
	fn auth_api_out(&self, _req: AuthApiOutRequest) -> impl Future<Output = AuthApiOutResponse> + Send{async{Default::default()}}
	// POST /auth/signin
	fn auth_api_signin(&self, _req: AuthApiSigninRequest) -> impl Future<Output = AuthApiSigninResponse> + Send{async{Default::default()}}
	// POST /auth/signup
	fn auth_api_signup(&self, _req: AuthApiSignupRequest) -> impl Future<Output = AuthApiSignupResponse> + Send{async{Default::default()}}
	// POST /auth/update
	fn auth_api_update(&self, _req: AuthApiUpdateRequest) -> impl Future<Output = AuthApiUpdateResponse> + Send{async{Default::default()}}
	// GET /cron
	fn background_cron(&self, _req: BackgroundCronRequest) -> impl Future<Output = BackgroundCronResponse> + Send{async{Default::default()}}
	// GET /image
	fn images_list(&self, _req: ImagesListRequest) -> impl Future<Output = ImagesListResponse> + Send{async{Default::default()}}
	// GET /job
	fn jobs_list(&self, _req: JobsListRequest) -> impl Future<Output = JobsListResponse> + Send{async{Default::default()}}
	// POST /job
	fn jobs_push(&self, _req: JobsPushRequest) -> impl Future<Output = JobsPushResponse> + Send{async{Default::default()}}
	// DELETE /job/{id}
	fn jobs_delete(&self, _req: JobsDeleteRequest) -> impl Future<Output = JobsDeleteResponse> + Send{async{Default::default()}}
	// GET /job/{id}/cat
	fn jobs_file_cat(&self, _req: JobsFileCatRequest) -> impl Future<Output = JobsFileCatResponse> + Send{async{Default::default()}}
	// GET /job/{id}/ls
	fn jobs_file_ls(&self, _req: JobsFileLsRequest) -> impl Future<Output = JobsFileLsResponse> + Send{async{Default::default()}}
	// GET /job/{id}/task
	fn jobs_tasklist(&self, _req: JobsTasklistRequest) -> impl Future<Output = JobsTasklistResponse> + Send{async{Default::default()}}
	// GET /status
	fn status_interface_status(&self, _req: StatusInterfaceStatusRequest) -> impl Future<Output = StatusInterfaceStatusResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for auth_api_user_get
#[derive(Debug)]
pub struct AuthApiUserGetRequest{
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for AuthApiUserGetRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for auth_api_user_get
#[derive(Debug)]
pub enum AuthApiUserGetResponse{
	Status200(User),
	Status400(String),
	Status403,
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AuthApiUserGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiUserGetResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for auth_api_out
#[derive(Debug)]
pub struct AuthApiOutRequest{
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for AuthApiOutRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for auth_api_out
#[derive(Debug)]
pub enum AuthApiOutResponse{
	Status204,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AuthApiOutResponse{
	fn default() -> Self{
		Self::Status204
	}
}
impl axum::response::IntoResponse for AuthApiOutResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for auth_api_signin
#[derive(Debug)]
pub struct AuthApiSigninRequest{
	pub body: PathsAuthSigninPostRequestBodyContentApplicationJsonSchema,
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for AuthApiSigninRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for auth_api_signin
#[derive(Debug)]
pub enum AuthApiSigninResponse{
	Status200(User),
	Status400(String),
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AuthApiSigninResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiSigninResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for auth_api_signup
#[derive(Debug)]
pub struct AuthApiSignupRequest{
	pub body: PathsAuthSignupPostRequestBodyContentApplicationJsonSchema,
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for AuthApiSignupRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for auth_api_signup
#[derive(Debug)]
pub enum AuthApiSignupResponse{
	Status200(User),
	Status400(String),
	Status403,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AuthApiSignupResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiSignupResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for auth_api_update
#[derive(Debug)]
pub struct AuthApiUpdateRequest{
	pub body: PathsAuthUpdatePostRequestBodyContentApplicationJsonSchema,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for AuthApiUpdateRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for auth_api_update
#[derive(Debug)]
pub enum AuthApiUpdateResponse{
	Status200(User),
	Status400(String),
	Status403,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for AuthApiUpdateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiUpdateResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for background_cron
#[derive(Debug)]
pub struct BackgroundCronRequest{
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for BackgroundCronRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for background_cron
#[derive(Debug)]
pub enum BackgroundCronResponse{
	Status204,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for BackgroundCronResponse{
	fn default() -> Self{
		Self::Status204
	}
}
impl axum::response::IntoResponse for BackgroundCronResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for images_list
#[derive(Debug)]
pub struct ImagesListRequest{
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for ImagesListRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for images_list
#[derive(Debug)]
pub enum ImagesListResponse{
	Status200(Vec<String>),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for ImagesListResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for ImagesListResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_list
#[derive(Debug)]
pub struct JobsListRequest{
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsListRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_list
#[derive(Debug)]
pub enum JobsListResponse{
	Status200(Vec<Job>),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsListResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for JobsListResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_push
#[derive(Debug)]
pub struct JobsPushRequest{
	pub body: PathsJobPostRequestBodyContentMultipartFormDataSchema,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsPushRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_push
#[derive(Debug)]
pub enum JobsPushResponse{
	Status200(Job),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsPushResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for JobsPushResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_delete
#[derive(Debug)]
pub struct JobsDeleteRequest{
	pub id:Uuid,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsDeleteRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_delete
#[derive(Debug)]
pub enum JobsDeleteResponse{
	Status204,
	Status400(String),
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsDeleteResponse{
	fn default() -> Self{
		Self::Status204
	}
}
impl axum::response::IntoResponse for JobsDeleteResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_file_cat
#[derive(Debug)]
pub struct JobsFileCatRequest{
	pub id:Uuid,
	pub path:String,
	pub limit:Option<i32>,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsFileCatRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_file_cat
#[derive(Debug)]
pub enum JobsFileCatResponse{
	Status200(Vec<u8>),
	Status400(String),
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsFileCatResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for JobsFileCatResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/octet-stream").body(axum::body::Body::from(v)).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_file_ls
#[derive(Debug)]
pub struct JobsFileLsRequest{
	pub id:Uuid,
	pub path:Option<String>,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsFileLsRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_file_ls
#[derive(Debug)]
pub enum JobsFileLsResponse{
	Status200(Vec<String>),
	Status400(String),
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsFileLsResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for JobsFileLsResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for jobs_tasklist
#[derive(Debug)]
pub struct JobsTasklistRequest{
	pub id:Uuid,
	pub request: http::Request<()>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
impl AsRef<http::Request<()>> for JobsTasklistRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for jobs_tasklist
#[derive(Debug)]
pub enum JobsTasklistResponse{
	Status200(Vec<Task>),
	Status400(String),
	Status404,
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for JobsTasklistResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for JobsTasklistResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status404=> axum::response::Response::builder().status(http::StatusCode::from_u16(404).unwrap()).body(axum::body::Body::empty()).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}
// Request type for status_interface_status
#[derive(Debug)]
pub struct StatusInterfaceStatusRequest{
	pub request: http::Request<()>,
}
impl AsRef<http::Request<()>> for StatusInterfaceStatusRequest{
	fn as_ref(&self) -> &http::Request<()>{&self.request}
}
// Response type for status_interface_status
#[derive(Debug)]
pub enum StatusInterfaceStatusResponse{
	Status200(Vec<Device>),
	Status400(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for StatusInterfaceStatusResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for StatusInterfaceStatusResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Raw(v)=>v,
		}
	}
}












#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Device{
	pub r#memory_size_megabytes:i32,
	pub r#memory_used_megabytes:i32,
	pub r#name:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Job{
	pub r#id:Uuid,
	pub r#id_root:Uuid,
	pub r#name:String,
	pub r#status:Vec<Task>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct JobRequest{
	pub r#archive:Vec<u8>,
	pub r#image:String,
	pub r#json:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Task{
	pub r#code:i32,
	pub r#path:Vec<String>,
	pub r#target:String,
	pub r#time_end:i32,
	pub r#time_run:i32,
}

pub type Uuid=uuid::Uuid;

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct User{
	pub r#auth_email:String,
	pub r#auth_email_password:String,
	pub r#id:Uuid,
	pub r#name:String,
	pub r#picture:String,
}



#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsAuthSigninPostRequestBodyContentApplicationJsonSchema{
	pub r#email:String,
	pub r#password:String,
}
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsAuthSignupPostRequestBodyContentApplicationJsonSchema{
	pub r#email:String,
	pub r#name:String,
	pub r#password:String,
}
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsAuthUpdatePostRequestBodyContentApplicationJsonSchema{
	pub r#name:String,
	pub r#password:String,
	pub r#password_new:String,
}
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsJobPostRequestBodyContentMultipartFormDataSchema{
	pub r#config:Option<String>,
	pub r#files:Vec<Vec<u8>>,
	pub r#image:String,
	pub r#paths:Vec<String>,
}

// following part is only for server

use axum;
use axum::http;
use axum::extract::FromRequest;

/// Helper function to generate text responses
fn text_response(code: http::StatusCode, body: String)->axum::response::Response{
	axum::response::Response::builder()
		.status(code)
		.header(http::header::CONTENT_TYPE, "text/plain")
		.body(axum::body::Body::from(body))
		.unwrap()
}

/// Returns axum::Router with root handlers for all operations registered
pub fn axum_router_operations<S: ApiInterface + Sync + Send + 'static>(instance :std::sync::Arc<S>)->axum::Router{
	let router = axum::Router::new();
	let i = instance.clone();
	let router = router.route("/auth", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::auth_api_user_get(i.as_ref(), AuthApiUserGetRequest{
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/auth/out", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::auth_api_out(i.as_ref(), AuthApiOutRequest{
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/auth/signin", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::auth_api_signin(i.as_ref(), AuthApiSigninRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/auth/signup", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::auth_api_signup(i.as_ref(), AuthApiSignupRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/auth/update", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::auth_api_update(i.as_ref(), AuthApiUpdateRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/cron", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::background_cron(i.as_ref(), BackgroundCronRequest{
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/image", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::images_list(i.as_ref(), ImagesListRequest{
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_list(i.as_ref(), JobsListRequest{
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_push(i.as_ref(), JobsPushRequest{
			body:{
	let r=http::Request::from_parts(parts.clone(), body);
	let v=match axum::extract::Multipart::from_request(r, &()).await{Ok(v)=>v,Err(e)=>return text_response(http::StatusCode::BAD_REQUEST, e.body_text())};
	match async |mut x: axum::extract::Multipart| -> std::result::Result<PathsJobPostRequestBodyContentMultipartFormDataSchema,String>{
	let mut o:PathsJobPostRequestBodyContentMultipartFormDataSchema=Default::default();
	while let Some(field) = x.next_field().await.map_err(|e| e.body_text())? {
		match field.name().unwrap_or_default() {
			"config" =>o.config=Some(field.text().await.map_err(|e| e.body_text())?),
			"files" =>o.files.push(field.bytes().await.map(|v| v.to_vec()).map_err(|e| e.body_text())?),
			"image" =>o.image=field.text().await.map_err(|e| e.body_text())?,
			"paths" =>o.paths.push(field.text().await.map_err(|e| e.body_text())?),
			other => return Err(format!("unknown field {other} in multipart-formdata"))
		}
	}
	Ok(o)
}(v).await {
	Ok(v)=>v,
	Err(e)=>return text_response(http::StatusCode::BAD_REQUEST,e)
}
},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job/{id}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_delete(i.as_ref(), JobsDeleteRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job/{id}/cat", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_file_cat(i.as_ref(), JobsFileCatRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			r#path:{let v=query.get("path").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: path in query={:?}", query))}},
			r#limit:{let v=query.get("limit").and_then(|v| v.parse().ok());v},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job/{id}/ls", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_file_ls(i.as_ref(), JobsFileLsRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			r#path:{let v=query.get("path").and_then(|v| v.parse().ok());v},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/job/{id}/task", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::jobs_tasklist(i.as_ref(), JobsTasklistRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: http::Request::from_parts(parts.clone(), ()),
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let i = instance.clone();
	let router = router.route("/status", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::status_interface_status(i.as_ref(), StatusInterfaceStatusRequest{
			request: http::Request::from_parts(parts.clone(), ()),
		}).await;
		axum::response::IntoResponse::into_response(ret)
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"schemas":{"Device":{"properties":{"memory_size_megabytes":{"format":"int32","type":"integer"},"memory_used_megabytes":{"format":"int32","type":"integer"},"name":{"type":"string"}},"required":["name","memory_used_megabytes","memory_size_megabytes"],"type":"object"},"Job":{"properties":{"id":{"$ref":"#/components/schemas/UUID"},"id_root":{"$ref":"#/components/schemas/UUID"},"name":{"type":"string"},"status":{"items":{"$ref":"#/components/schemas/Task"},"type":"array"}},"required":["id","id_root","name","status"],"type":"object"},"JobRequest":{"properties":{"archive":{"format":"byte","type":"string"},"image":{"type":"string"},"json":{"type":"string"}},"required":["json","image","archive"],"type":"object"},"Task":{"properties":{"code":{"format":"int32","type":"integer"},"path":{"items":{"type":"string"},"type":"array"},"target":{"type":"string"},"time_end":{"format":"int32","type":"integer"},"time_run":{"format":"int32","type":"integer"}},"required":["target","code","time_run","time_end","path"],"type":"object"},"UUID":{"format":"uuid","type":"string"},"User":{"properties":{"auth_email":{"type":"string"},"auth_email_password":{"type":"string"},"id":{"$ref":"#/components/schemas/UUID"},"name":{"type":"string"},"picture":{"type":"string"}},"required":["id","name","picture","auth_email","auth_email_password"],"type":"object"}},"securitySchemes":{"BearerAuth":{"scheme":"Bearer","type":"http"}}},"info":{"title":"API overview","version":"0.0.0"},"openapi":"3.0.0","paths":{"/auth":{"get":{"description":"\tユーザー情報を取得します、認証が必要","operationId":"AuthApi_user_get","responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"description":"Access is forbidden."},"404":{"description":"The server cannot find the requested resource."}},"security":[{"BearerAuth":[]}]}},"/auth/out":{"get":{"description":"\tログアウト","operationId":"AuthApi_out","responses":{"204":{"description":"There is no content to send for this request, but the headers may be useful. "}}}},"/auth/signin":{"post":{"description":"\tログイン","operationId":"AuthApi_signin","requestBody":{"content":{"application/json":{"schema":{"properties":{"email":{"type":"string"},"password":{"type":"string"}},"required":["email","password"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"description":"The server cannot find the requested resource."}}}},"/auth/signup":{"post":{"description":"\tユーザー登録","operationId":"AuthApi_signup","requestBody":{"content":{"application/json":{"schema":{"properties":{"email":{"type":"string"},"name":{"type":"string"},"password":{"type":"string"}},"required":["name","email","password"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"description":"Access is forbidden."}}}},"/auth/update":{"post":{"description":"\tユーザー情報更新","operationId":"AuthApi_update","requestBody":{"content":{"application/json":{"schema":{"properties":{"name":{"type":"string"},"password":{"type":"string"},"password_new":{"type":"string"}},"required":["name","password","password_new"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]}},"/cron":{"get":{"description":"\tタスクキューを進めます。実行可能なプロセスが存在すれば実行し、状態を更新します。1分毎など定期的に呼び出してください。","operationId":"Background_cron","responses":{"204":{"description":"There is no content to send for this request, but the headers may be useful. "}}}},"/image":{"get":{"description":"\tジョブのランナーとして指定可能なdocker image一覧です。FaceSimulatorはその一つです。","operationId":"Images_list","responses":{"200":{"content":{"application/json":{"schema":{"items":{"type":"string"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/job":{"get":{"description":"\tジョブ一覧を返します、認証が必要","operationId":"Jobs_list","responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Job"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}},"security":[{"BearerAuth":[]}]},"post":{"description":"\t新規にジョブを開始します。\n\t- files(required): 実行に必要なファイル一覧、アップロードされます。不要なら0件も可能。\n\t- paths(required): filesと1対1対応するパス一覧。\n\t- image(required): ジョブを実行するdocker image。\n\t- config: config用jsonファイル。指定しなければfileから推測されます（トップレベルかつ.json拡張子が付いたファイル）。","operationId":"Jobs_push","requestBody":{"content":{"multipart/form-data":{"encoding":{"files":{"contentType":"*/*"},"paths":{"contentType":"text/plain"}},"schema":{"properties":{"config":{"type":"string"},"files":{"items":{"format":"binary","type":"string"},"type":"array"},"image":{"type":"string"},"paths":{"items":{"type":"string"},"type":"array"}},"required":["files","paths","image"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Job"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}},"security":[{"BearerAuth":[]}]}},"/job/{id}":{"delete":{"description":"\tジョブを削除します、認証が必要","operationId":"Jobs_delete","parameters":[{"in":"path","name":"id","required":true,"schema":{"$ref":"#/components/schemas/UUID"},"style":"simple"}],"responses":{"204":{"description":"There is no content to send for this request, but the headers may be useful. "},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"description":"The server cannot find the requested resource."}},"security":[{"BearerAuth":[]}]}},"/job/{id}/cat":{"get":{"description":"\tジョブのファイルを配信します。path_fileは/（スラッシュ）を含めて良いです。","operationId":"Jobs_file_cat","parameters":[{"in":"path","name":"id","required":true,"schema":{"$ref":"#/components/schemas/UUID"},"style":"simple"},{"explode":false,"in":"query","name":"path","required":true,"schema":{"type":"string"},"style":"form"},{"explode":false,"in":"query","name":"limit","schema":{"format":"int32","type":"integer"},"style":"form"}],"responses":{"200":{"content":{"application/octet-stream":{"schema":{"format":"binary","type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"description":"The server cannot find the requested resource."}},"security":[{"BearerAuth":[]}]}},"/job/{id}/ls":{"get":{"description":"\tジョブのファイル一覧を返します、認証が必要","operationId":"Jobs_file_ls","parameters":[{"in":"path","name":"id","required":true,"schema":{"$ref":"#/components/schemas/UUID"},"style":"simple"},{"explode":false,"in":"query","name":"path","schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"type":"string"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"description":"The server cannot find the requested resource."}},"security":[{"BearerAuth":[]}]}},"/job/{id}/task":{"get":{"description":"\tジョブのタスク一覧を返します、認証が必要","operationId":"Jobs_tasklist","parameters":[{"in":"path","name":"id","required":true,"schema":{"$ref":"#/components/schemas/UUID"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Task"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"404":{"description":"The server cannot find the requested resource."}},"security":[{"BearerAuth":[]}]}},"/status":{"get":{"description":"\tシステム情報（GPUの使用状態）と稼働中のタスクを取得","operationId":"StatusInterface_status","responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Device"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}}},"servers":[{"description":"開発用","url":"/api","variables":{}}]}"###
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
	// GET /auth
	// async fn auth_api_user_get(&self, _req: AuthApiUserGetRequest) -> AuthApiUserGetResponse{Default::default()}
	// GET /auth/out
	// async fn auth_api_out(&self, _req: AuthApiOutRequest) -> AuthApiOutResponse{Default::default()}
	// POST /auth/signin
	// async fn auth_api_signin(&self, _req: AuthApiSigninRequest) -> AuthApiSigninResponse{Default::default()}
	// POST /auth/signup
	// async fn auth_api_signup(&self, _req: AuthApiSignupRequest) -> AuthApiSignupResponse{Default::default()}
	// POST /auth/update
	// async fn auth_api_update(&self, _req: AuthApiUpdateRequest) -> AuthApiUpdateResponse{Default::default()}
	// GET /cron
	// async fn background_cron(&self, _req: BackgroundCronRequest) -> BackgroundCronResponse{Default::default()}
	// GET /image
	// async fn images_list(&self, _req: ImagesListRequest) -> ImagesListResponse{Default::default()}
	// GET /job
	// async fn jobs_list(&self, _req: JobsListRequest) -> JobsListResponse{Default::default()}
	// POST /job
	// async fn jobs_push(&self, _req: JobsPushRequest) -> JobsPushResponse{Default::default()}
	// DELETE /job/{id}
	// async fn jobs_delete(&self, _req: JobsDeleteRequest) -> JobsDeleteResponse{Default::default()}
	// GET /job/{id}/cat
	// async fn jobs_file_cat(&self, _req: JobsFileCatRequest) -> JobsFileCatResponse{Default::default()}
	// GET /job/{id}/ls
	// async fn jobs_file_ls(&self, _req: JobsFileLsRequest) -> JobsFileLsResponse{Default::default()}
	// GET /job/{id}/task
	// async fn jobs_tasklist(&self, _req: JobsTasklistRequest) -> JobsTasklistResponse{Default::default()}
	// GET /status
	// async fn status_interface_status(&self, _req: StatusInterfaceStatusRequest) -> StatusInterfaceStatusResponse{Default::default()}
}

/// Estimates the origin URL (scheme://host) from an HTTP request
/// Priority: Forwarded > X-Forwarded-* > Host
pub fn origin_from_request<B>(req: &http::Request<B>) -> Option<String> {
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
		.get(http::header::FORWARDED)
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
		.get(http::header::HOST)
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
