














































// This file was automatically generated from OpenAPI specification by mandolin https://github.com/lzpel/mandolin

/* Cargo.toml to build this server

[features]
mandolin_client = ["dep:reqwest"]

[dependencies]
serde= { version="*", features = ["derive"] }
serde_json= "*"
axum = { version = "*", features = ["multipart"] }
tokio = { version = "*", features = ["rt", "rt-multi-thread", "macros", "signal"] }
reqwest = { version = "*", features = ["json"], optional = true }
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

	// GET /auth/callback_oauth
	fn auth_api_callback_oauth(&self, _req: AuthApiCallbackOauthRequest) -> impl Future<Output = AuthApiCallbackOauthResponse> + Send{async{Default::default()}}

	// POST /auth/email
	fn auth_api_email(&self, _req: AuthApiEmailRequest) -> impl Future<Output = AuthApiEmailResponse> + Send{async{Default::default()}}

	// GET /auth/google
	fn auth_api_google(&self, _req: AuthApiGoogleRequest) -> impl Future<Output = AuthApiGoogleResponse> + Send{async{Default::default()}}

	// GET /auth/out
	fn auth_api_out(&self, _req: AuthApiOutRequest) -> impl Future<Output = AuthApiOutResponse> + Send{async{Default::default()}}

	// GET /page
	fn page_api_get(&self, _req: PageApiGetRequest) -> impl Future<Output = PageApiGetResponse> + Send{async{Default::default()}}

	// POST /page
	fn page_api_push(&self, _req: PageApiPushRequest) -> impl Future<Output = PageApiPushResponse> + Send{async{Default::default()}}

	// POST /page/upload
	fn page_api_upload(&self, _req: PageApiUploadRequest) -> impl Future<Output = PageApiUploadResponse> + Send{async{Default::default()}}

	// GET /user
	fn user_api_user_get(&self, _req: UserApiUserGetRequest) -> impl Future<Output = UserApiUserGetResponse> + Send{async{Default::default()}}

	// POST /user
	fn user_api_user_set(&self, _req: UserApiUserSetRequest) -> impl Future<Output = UserApiUserSetResponse> + Send{async{Default::default()}}

	// DELETE /user
	fn user_api_user_pop(&self, _req: UserApiUserPopRequest) -> impl Future<Output = UserApiUserPopResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}



// Request type for auth_api_callback_oauth
#[derive(Debug)]
pub struct AuthApiCallbackOauthRequest{
	pub code:String,
	pub state:String,
}
// Response type for auth_api_callback_oauth
#[derive(Debug)]
pub enum AuthApiCallbackOauthResponse{
	Status200(String),
	Status400(String),
	Error(String),
}
impl Default for AuthApiCallbackOauthResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiCallbackOauthResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for auth_api_email
#[derive(Debug)]
pub struct AuthApiEmailRequest{
	pub body: PathsAuthEmailPostRequestBodyContentApplicationJsonSchema,
}
// Response type for auth_api_email
#[derive(Debug)]
pub enum AuthApiEmailResponse{
	Status204(()),
	Status400(String),
	Error(String),
}
impl Default for AuthApiEmailResponse{
	fn default() -> Self{
		Self::Status204(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiEmailResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for auth_api_google
#[derive(Debug)]
pub struct AuthApiGoogleRequest{
}
// Response type for auth_api_google
#[derive(Debug)]
pub enum AuthApiGoogleResponse{
	Status200(String),
	Error(String),
}
impl Default for AuthApiGoogleResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiGoogleResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for auth_api_out
#[derive(Debug)]
pub struct AuthApiOutRequest{
}
// Response type for auth_api_out
#[derive(Debug)]
pub enum AuthApiOutResponse{
	Status204(()),
	Error(String),
}
impl Default for AuthApiOutResponse{
	fn default() -> Self{
		Self::Status204(Default::default())
	}
}
impl axum::response::IntoResponse for AuthApiOutResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for page_api_get
#[derive(Debug)]
pub struct PageApiGetRequest{
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for page_api_get
#[derive(Debug)]
pub enum PageApiGetResponse{
	Status200(Vec<Page>),
	Status400(String),
	Status403(()),
	Error(String),
}
impl Default for PageApiGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for PageApiGetResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for page_api_push
#[derive(Debug)]
pub struct PageApiPushRequest{
	pub body: PathsPagePostRequestBodyContentApplicationJsonSchema,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for page_api_push
#[derive(Debug)]
pub enum PageApiPushResponse{
	Status200(Page),
	Status400(String),
	Status403(()),
	Error(String),
}
impl Default for PageApiPushResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for PageApiPushResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for page_api_upload
#[derive(Debug)]
pub struct PageApiUploadRequest{
	pub fileName:String,
	pub expiresIn:Option<i32>,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for page_api_upload
#[derive(Debug)]
pub enum PageApiUploadResponse{
	Status200(Vec<String>),
	Status400(String),
	Error(String),
}
impl Default for PageApiUploadResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for PageApiUploadResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for user_api_user_get
#[derive(Debug)]
pub struct UserApiUserGetRequest{
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for user_api_user_get
#[derive(Debug)]
pub enum UserApiUserGetResponse{
	Status200(User),
	Status400(String),
	Status403(()),
	Error(String),
}
impl Default for UserApiUserGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for UserApiUserGetResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for user_api_user_set
#[derive(Debug)]
pub struct UserApiUserSetRequest{
	pub body: PathsUserPostRequestBodyContentApplicationJsonSchema,
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for user_api_user_set
#[derive(Debug)]
pub enum UserApiUserSetResponse{
	Status200(User),
	Status400(String),
	Status403(()),
	Error(String),
}
impl Default for UserApiUserSetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for UserApiUserSetResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for user_api_user_pop
#[derive(Debug)]
pub struct UserApiUserPopRequest{
	pub security: AuthContext, /*[{"BearerAuth": []}]*/
}
// Response type for user_api_user_pop
#[derive(Debug)]
pub enum UserApiUserPopResponse{
	Status204(()),
	Status400(String),
	Status403(()),
	Error(String),
}
impl Default for UserApiUserPopResponse{
	fn default() -> Self{
		Self::Status204(Default::default())
	}
}
impl axum::response::IntoResponse for UserApiUserPopResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status204(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(204).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Status400(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(400).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Status403(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(403).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}












#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct NotFoundResponse{
	pub r#body:(),
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Page{
	pub r#content:String,
	pub r#id:Uuid,
	pub r#id_root:Uuid,
	pub r#name:String,
	pub r#path_image:Vec<String>,
	pub r#path_model:String,
	pub r#progress:i32,
	pub r#script:String,
	pub r#view_image:Vec<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Plan{
	pub r#currency:String,
	pub r#description:Option<String>,
	pub r#id:Uuid,
	pub r#name:String,
	pub r#priceMonthly:i32,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Subscription{
	pub r#id:Uuid,
	pub r#id_node:Uuid,
	pub r#id_root:Uuid,
	pub r#status:String,
}

pub type Uuid=uuid::Uuid;

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct User{
	pub r#auth_email:String,
	pub r#auth_email_password:String,
	pub r#auth_google:String,
	pub r#id:Uuid,
	pub r#is_active:bool,
	pub r#name:String,
	pub r#picture:String,
}



#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsUserPostRequestBodyContentApplicationJsonSchema{
	pub r#user:User,
}
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsAuthEmailPostRequestBodyContentApplicationJsonSchema{
	pub r#email:String,
}
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsPagePostRequestBodyContentApplicationJsonSchema{
	pub r#path_image:Vec<String>,
	pub r#script:String,
	pub r#view_image:Vec<String>,
}

// following part is only for client

#[cfg(feature = "mandolin_client")]
pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

#[cfg(feature = "mandolin_client")]
impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), client: reqwest::Client::new() }
    }
}

#[cfg(feature = "mandolin_client")]
impl ApiInterface for ApiClient {

    // GET /auth/callback_oauth
    fn auth_api_callback_oauth(&self, req: AuthApiCallbackOauthRequest) -> impl Future<Output = AuthApiCallbackOauthResponse> + Send {
        let url = format!("{}{}", self.base_url, "/auth/callback_oauth"
        );
        let client = self.client.clone();
        async move {
            let r = match client.get(&url)
                .query(&[("code", req.r#code.to_string())])
                .query(&[("state", req.r#state.to_string())])
                .send().await {
                Ok(r) => r,
                Err(e) => return AuthApiCallbackOauthResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => AuthApiCallbackOauthResponse::Status200(v), Err(e) => AuthApiCallbackOauthResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => AuthApiCallbackOauthResponse::Status400(v), Err(e) => AuthApiCallbackOauthResponse::Error(e.to_string()) },
                code => AuthApiCallbackOauthResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // POST /auth/email
    fn auth_api_email(&self, req: AuthApiEmailRequest) -> impl Future<Output = AuthApiEmailResponse> + Send {
        let url = format!("{}{}", self.base_url, "/auth/email"
        );
        let client = self.client.clone();
        async move {
            let r = match client.post(&url)
                .json(&req.body)
                .send().await {
                Ok(r) => r,
                Err(e) => return AuthApiEmailResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                204 =>
                    match r.json().await { Ok(v) => AuthApiEmailResponse::Status204(v), Err(e) => AuthApiEmailResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => AuthApiEmailResponse::Status400(v), Err(e) => AuthApiEmailResponse::Error(e.to_string()) },
                code => AuthApiEmailResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /auth/google
    fn auth_api_google(&self, req: AuthApiGoogleRequest) -> impl Future<Output = AuthApiGoogleResponse> + Send {
        let url = format!("{}{}", self.base_url, "/auth/google"
        );
        let client = self.client.clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return AuthApiGoogleResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => AuthApiGoogleResponse::Status200(v), Err(e) => AuthApiGoogleResponse::Error(e.to_string()) },
                code => AuthApiGoogleResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /auth/out
    fn auth_api_out(&self, req: AuthApiOutRequest) -> impl Future<Output = AuthApiOutResponse> + Send {
        let url = format!("{}{}", self.base_url, "/auth/out"
        );
        let client = self.client.clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return AuthApiOutResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                204 =>
                    match r.json().await { Ok(v) => AuthApiOutResponse::Status204(v), Err(e) => AuthApiOutResponse::Error(e.to_string()) },
                code => AuthApiOutResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /page
    fn page_api_get(&self, req: PageApiGetRequest) -> impl Future<Output = PageApiGetResponse> + Send {
        let url = format!("{}{}", self.base_url, "/page"
        );
        let client = self.client.clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return PageApiGetResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => PageApiGetResponse::Status200(v), Err(e) => PageApiGetResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => PageApiGetResponse::Status400(v), Err(e) => PageApiGetResponse::Error(e.to_string()) },
                403 =>
                    match r.json().await { Ok(v) => PageApiGetResponse::Status403(v), Err(e) => PageApiGetResponse::Error(e.to_string()) },
                code => PageApiGetResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // POST /page
    fn page_api_push(&self, req: PageApiPushRequest) -> impl Future<Output = PageApiPushResponse> + Send {
        let url = format!("{}{}", self.base_url, "/page"
        );
        let client = self.client.clone();
        async move {
            let r = match client.post(&url)
                .json(&req.body)
                .send().await {
                Ok(r) => r,
                Err(e) => return PageApiPushResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => PageApiPushResponse::Status200(v), Err(e) => PageApiPushResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => PageApiPushResponse::Status400(v), Err(e) => PageApiPushResponse::Error(e.to_string()) },
                403 =>
                    match r.json().await { Ok(v) => PageApiPushResponse::Status403(v), Err(e) => PageApiPushResponse::Error(e.to_string()) },
                code => PageApiPushResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // POST /page/upload
    fn page_api_upload(&self, req: PageApiUploadRequest) -> impl Future<Output = PageApiUploadResponse> + Send {
        let url = format!("{}{}", self.base_url, "/page/upload"
        );
        let client = self.client.clone();
        async move {
            let r = match client.post(&url)
                .query(&[("fileName", req.r#fileName.to_string())])
                .query(&req.r#expiresIn.as_ref().map(|v| [("expiresIn", v.to_string())]))
                .send().await {
                Ok(r) => r,
                Err(e) => return PageApiUploadResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => PageApiUploadResponse::Status200(v), Err(e) => PageApiUploadResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => PageApiUploadResponse::Status400(v), Err(e) => PageApiUploadResponse::Error(e.to_string()) },
                code => PageApiUploadResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /user
    fn user_api_user_get(&self, req: UserApiUserGetRequest) -> impl Future<Output = UserApiUserGetResponse> + Send {
        let url = format!("{}{}", self.base_url, "/user"
        );
        let client = self.client.clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return UserApiUserGetResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => UserApiUserGetResponse::Status200(v), Err(e) => UserApiUserGetResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => UserApiUserGetResponse::Status400(v), Err(e) => UserApiUserGetResponse::Error(e.to_string()) },
                403 =>
                    match r.json().await { Ok(v) => UserApiUserGetResponse::Status403(v), Err(e) => UserApiUserGetResponse::Error(e.to_string()) },
                code => UserApiUserGetResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // POST /user
    fn user_api_user_set(&self, req: UserApiUserSetRequest) -> impl Future<Output = UserApiUserSetResponse> + Send {
        let url = format!("{}{}", self.base_url, "/user"
        );
        let client = self.client.clone();
        async move {
            let r = match client.post(&url)
                .json(&req.body)
                .send().await {
                Ok(r) => r,
                Err(e) => return UserApiUserSetResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => UserApiUserSetResponse::Status200(v), Err(e) => UserApiUserSetResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => UserApiUserSetResponse::Status400(v), Err(e) => UserApiUserSetResponse::Error(e.to_string()) },
                403 =>
                    match r.json().await { Ok(v) => UserApiUserSetResponse::Status403(v), Err(e) => UserApiUserSetResponse::Error(e.to_string()) },
                code => UserApiUserSetResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // DELETE /user
    fn user_api_user_pop(&self, req: UserApiUserPopRequest) -> impl Future<Output = UserApiUserPopResponse> + Send {
        let url = format!("{}{}", self.base_url, "/user"
        );
        let client = self.client.clone();
        async move {
            let r = match client.delete(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return UserApiUserPopResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                204 =>
                    match r.json().await { Ok(v) => UserApiUserPopResponse::Status204(v), Err(e) => UserApiUserPopResponse::Error(e.to_string()) },
                400 =>
                    match r.json().await { Ok(v) => UserApiUserPopResponse::Status400(v), Err(e) => UserApiUserPopResponse::Error(e.to_string()) },
                403 =>
                    match r.json().await { Ok(v) => UserApiUserPopResponse::Status403(v), Err(e) => UserApiUserPopResponse::Error(e.to_string()) },
                code => UserApiUserPopResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }
}

// following part is only for server

use axum;
use axum::http;
use axum::extract::FromRequest;

/// Axum-specific API interface trait
/// All ApiInterface implementors automatically satisfy this via blanket impl.
/// Override methods here for axum-specific behavior (streaming, custom headers, etc.)
pub trait ApiInterfaceAxum: ApiInterface + Sync{
	/// Authentication process: Generate AuthContext from request
	fn authorize(&self, _req: http::Request<()>) -> impl Future<Output = Result<AuthContext, String>> + Send{async { Ok(Default::default()) } }

	// GET /auth/callback_oauth
	fn auth_api_callback_oauth(&self, _raw: http::Request<()>, req: AuthApiCallbackOauthRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::auth_api_callback_oauth(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// POST /auth/email
	fn auth_api_email(&self, _raw: http::Request<()>, req: AuthApiEmailRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::auth_api_email(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /auth/google
	fn auth_api_google(&self, _raw: http::Request<()>, req: AuthApiGoogleRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::auth_api_google(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /auth/out
	fn auth_api_out(&self, _raw: http::Request<()>, req: AuthApiOutRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::auth_api_out(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /page
	fn page_api_get(&self, _raw: http::Request<()>, req: PageApiGetRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::page_api_get(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// POST /page
	fn page_api_push(&self, _raw: http::Request<()>, req: PageApiPushRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::page_api_push(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// POST /page/upload
	fn page_api_upload(&self, _raw: http::Request<()>, req: PageApiUploadRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::page_api_upload(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /user
	fn user_api_user_get(&self, _raw: http::Request<()>, req: UserApiUserGetRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::user_api_user_get(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// POST /user
	fn user_api_user_set(&self, _raw: http::Request<()>, req: UserApiUserSetRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::user_api_user_set(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// DELETE /user
	fn user_api_user_pop(&self, _raw: http::Request<()>, req: UserApiUserPopRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::user_api_user_pop(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}
}
impl<T: ApiInterface + Sync> ApiInterfaceAxum for T{}

/// Helper function to generate text responses
fn text_response(code: http::StatusCode, body: String)->axum::response::Response{
	axum::response::Response::builder()
		.status(code)
		.header(http::header::CONTENT_TYPE, "text/plain")
		.body(axum::body::Body::from(body))
		.unwrap()
}

/// Returns axum::Router with root handlers for all operations registered
pub fn axum_router_operations<S: ApiInterfaceAxum + Sync + Send + 'static>(instance :std::sync::Arc<S>)->axum::Router{
	let router = axum::Router::new();

	let i = instance.clone();
	let router = router.route("/auth/callback_oauth", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::auth_api_callback_oauth(i.as_ref(), http::Request::from_parts(parts.clone(), ()), AuthApiCallbackOauthRequest{
			r#code:{let v=query.get("code").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: code in query={:?}", query))}},
			r#state:{let v=query.get("state").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: state in query={:?}", query))}},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/auth/email", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::auth_api_email(i.as_ref(), http::Request::from_parts(parts.clone(), ()), AuthApiEmailRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/auth/google", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::auth_api_google(i.as_ref(), http::Request::from_parts(parts.clone(), ()), AuthApiGoogleRequest{
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/auth/out", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::auth_api_out(i.as_ref(), http::Request::from_parts(parts.clone(), ()), AuthApiOutRequest{
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/page", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::page_api_get(i.as_ref(), http::Request::from_parts(parts.clone(), ()), PageApiGetRequest{
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/page", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::page_api_push(i.as_ref(), http::Request::from_parts(parts.clone(), ()), PageApiPushRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/page/upload", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::page_api_upload(i.as_ref(), http::Request::from_parts(parts.clone(), ()), PageApiUploadRequest{
			r#fileName:{let v=query.get("fileName").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: fileName in query={:?}", query))}},
			r#expiresIn:{let v=query.get("expiresIn").and_then(|v| v.parse().ok());v},
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/user", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::user_api_user_get(i.as_ref(), http::Request::from_parts(parts.clone(), ()), UserApiUserGetRequest{
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/user", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::user_api_user_set(i.as_ref(), http::Request::from_parts(parts.clone(), ()), UserApiUserSetRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/user", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::user_api_user_pop(i.as_ref(), http::Request::from_parts(parts.clone(), ()), UserApiUserPopRequest{
			security: match i.as_ref().authorize(http::Request::from_parts(parts.clone(), ())).await {
				Ok(v)=>v,
				Err(e)=>return text_response(http::StatusCode::UNAUTHORIZED, e)
			},
		}).await;
		ret
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"schemas":{"NotFoundResponse":{"properties":{"body":{"type":"null"}},"required":["body"],"type":"object"},"Page":{"properties":{"content":{"type":"string"},"id":{"$ref":"#/components/schemas/UUID"},"id_root":{"$ref":"#/components/schemas/UUID"},"name":{"type":"string"},"path_image":{"items":{"type":"string"},"type":"array"},"path_model":{"type":"string"},"progress":{"format":"int32","type":"integer"},"script":{"type":"string"},"view_image":{"items":{"type":"string"},"type":"array"}},"required":["id","id_root","name","content","path_model","path_image","view_image","script","progress"],"type":"object"},"Plan":{"properties":{"currency":{"enum":["JPY","USD"],"type":"string"},"description":{"type":"string"},"id":{"$ref":"#/components/schemas/UUID"},"name":{"type":"string"},"priceMonthly":{"format":"int32","type":"integer"}},"required":["id","name","priceMonthly","currency"],"type":"object"},"Subscription":{"properties":{"id":{"$ref":"#/components/schemas/UUID"},"id_node":{"$ref":"#/components/schemas/UUID"},"id_root":{"$ref":"#/components/schemas/UUID"},"status":{"enum":["active","trialing","canceled","past_due"],"type":"string"}},"required":["id","id_root","id_node","status"],"type":"object"},"UUID":{"format":"uuid","type":"string"},"User":{"properties":{"auth_email":{"type":"string"},"auth_email_password":{"type":"string"},"auth_google":{"type":"string"},"id":{"$ref":"#/components/schemas/UUID"},"is_active":{"type":"boolean"},"name":{"type":"string"},"picture":{"type":"string"}},"required":["id","name","picture","auth_email","auth_google","auth_email_password","is_active"],"type":"object"}},"securitySchemes":{"BearerAuth":{"scheme":"Bearer","type":"http"}}},"info":{"title":"Sarod","version":"0.0.0"},"openapi":"3.1.0","paths":{"/auth/callback_oauth":{"get":{"description":"Adds a user.\nFor OAuth callback.","operationId":"AuthApi_callback_oauth","parameters":[{"explode":false,"in":"query","name":"code","required":true,"schema":{"type":"string"},"style":"form"},{"explode":false,"in":"query","name":"state","required":true,"schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/auth/email":{"post":{"description":"Sends a link to the specified email address to verify it.\nOnly those who click the link can reach the user addition screen.\nuri: Extract domain from frontend for callback\nemail: Email address","operationId":"AuthApi_email","requestBody":{"content":{"application/json":{"schema":{"properties":{"email":{"type":"string"}},"required":["email"],"type":"object"}}},"required":true},"responses":{"204":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"There is no content to send for this request, but the headers may be useful. "},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}}}},"/auth/google":{"get":{"description":"Google login","operationId":"AuthApi_google","responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}}},"/auth/out":{"get":{"description":"Logout","operationId":"AuthApi_out","responses":{"204":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"There is no content to send for this request, but the headers may be useful. "}}}},"/page":{"get":{"description":"Returns a list of videos for the home screen. Requires authentication.","operationId":"PageApi_get","responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Page"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]},"post":{"description":"Adds/updates a video. Requires authentication.","operationId":"PageApi_push","requestBody":{"content":{"application/json":{"schema":{"properties":{"path_image":{"items":{"type":"string"},"type":"array"},"script":{"type":"string"},"view_image":{"items":{"type":"string"},"type":"array"}},"required":["path_image","view_image","script"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Page"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]}},"/page/upload":{"post":{"description":"Returns a URL for uploading large files to a temporary area.","operationId":"PageApi_upload","parameters":[{"explode":false,"in":"query","name":"fileName","required":true,"schema":{"type":"string"},"style":"form"},{"explode":false,"in":"query","name":"expiresIn","schema":{"format":"int32","type":"integer"},"style":"form"}],"responses":{"200":{"content":{"application/json":{"schema":{"items":{"type":"string"},"type":"array"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."}},"security":[{"BearerAuth":[]}]}},"/user":{"delete":{"description":"Deletes a user. Requires authentication.","operationId":"UserApi_user_pop","responses":{"204":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"There is no content to send for this request, but the headers may be useful. "},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]},"get":{"description":"Retrieves user information. Requires authentication.","operationId":"UserApi_user_get","responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]},"post":{"description":"Sets user name or profile. Requires authentication.","operationId":"UserApi_user_set","requestBody":{"content":{"application/json":{"schema":{"properties":{"user":{"$ref":"#/components/schemas/User"}},"required":["user"],"type":"object"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/User"}}},"description":"The request has succeeded."},"400":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The server could not understand the request due to invalid syntax."},"403":{"content":{"application/json":{"schema":{"type":"null"}}},"description":"Access is forbidden."}},"security":[{"BearerAuth":[]}]}}},"servers":[{"description":"For development","url":"/api","variables":{}}]}"###
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
pub fn axum_router<S: ApiInterfaceAxum + Sync + Send + 'static>(instance: S)->axum::Router{
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

	// GET /auth/callback_oauth
	// async fn auth_api_callback_oauth(&self, _req: AuthApiCallbackOauthRequest) -> AuthApiCallbackOauthResponse{Default::default()}

	// POST /auth/email
	// async fn auth_api_email(&self, _req: AuthApiEmailRequest) -> AuthApiEmailResponse{Default::default()}

	// GET /auth/google
	// async fn auth_api_google(&self, _req: AuthApiGoogleRequest) -> AuthApiGoogleResponse{Default::default()}

	// GET /auth/out
	// async fn auth_api_out(&self, _req: AuthApiOutRequest) -> AuthApiOutResponse{Default::default()}

	// GET /page
	// async fn page_api_get(&self, _req: PageApiGetRequest) -> PageApiGetResponse{Default::default()}

	// POST /page
	// async fn page_api_push(&self, _req: PageApiPushRequest) -> PageApiPushResponse{Default::default()}

	// POST /page/upload
	// async fn page_api_upload(&self, _req: PageApiUploadRequest) -> PageApiUploadResponse{Default::default()}

	// GET /user
	// async fn user_api_user_get(&self, _req: UserApiUserGetRequest) -> UserApiUserGetResponse{Default::default()}

	// POST /user
	// async fn user_api_user_set(&self, _req: UserApiUserSetRequest) -> UserApiUserSetResponse{Default::default()}

	// DELETE /user
	// async fn user_api_user_pop(&self, _req: UserApiUserPopRequest) -> UserApiUserPopResponse{Default::default()}
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
