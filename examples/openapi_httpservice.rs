












































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
	// GET /widgets
	fn widgets_list(&self, _req: WidgetsListRequest) -> impl Future<Output = WidgetsListResponse> + Send{async{Default::default()}}
	// POST /widgets
	fn widgets_create(&self, _req: WidgetsCreateRequest) -> impl Future<Output = WidgetsCreateResponse> + Send{async{Default::default()}}
	// GET /widgets/{id}
	fn widgets_read(&self, _req: WidgetsReadRequest) -> impl Future<Output = WidgetsReadResponse> + Send{async{Default::default()}}
	// DELETE /widgets/{id}
	fn widgets_delete(&self, _req: WidgetsDeleteRequest) -> impl Future<Output = WidgetsDeleteResponse> + Send{async{Default::default()}}
	// PATCH /widgets/{id}
	fn widgets_update(&self, _req: WidgetsUpdateRequest) -> impl Future<Output = WidgetsUpdateResponse> + Send{async{Default::default()}}
	// POST /widgets/{id}/analyze
	fn widgets_analyze(&self, _req: WidgetsAnalyzeRequest) -> impl Future<Output = WidgetsAnalyzeResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for widgets_list
#[derive(Debug)]
pub struct WidgetsListRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsListRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_list
#[derive(Debug)]
pub enum WidgetsListResponse{
	Status200(Vec<Widget>),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsListResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widgets_create
#[derive(Debug)]
pub struct WidgetsCreateRequest{
	pub body: WidgetCreate,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsCreateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_create
#[derive(Debug)]
pub enum WidgetsCreateResponse{
	Status200(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsCreateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widgets_read
#[derive(Debug)]
pub struct WidgetsReadRequest{
	pub id:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsReadRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_read
#[derive(Debug)]
pub enum WidgetsReadResponse{
	Status200(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsReadResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widgets_delete
#[derive(Debug)]
pub struct WidgetsDeleteRequest{
	pub id:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsDeleteRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_delete
#[derive(Debug)]
pub enum WidgetsDeleteResponse{
	Status204,
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsDeleteResponse{
	fn default() -> Self{
		Self::Status204
	}
}
// Request type for widgets_update
#[derive(Debug)]
pub struct WidgetsUpdateRequest{
	pub id:String,
	pub body: WidgetUpdate,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsUpdateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_update
#[derive(Debug)]
pub enum WidgetsUpdateResponse{
	Status200(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsUpdateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widgets_analyze
#[derive(Debug)]
pub struct WidgetsAnalyzeRequest{
	pub id:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetsAnalyzeRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widgets_analyze
#[derive(Debug)]
pub enum WidgetsAnalyzeResponse{
	Status200(String),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetsAnalyzeResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}





#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Error{
	pub r#code:i32,
	pub r#message:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Widget{
	pub r#color:String,
	pub r#id:String,
	pub r#weight:i32,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct WidgetCreate{
	pub r#color:String,
	pub r#weight:i32,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct WidgetUpdate{
	pub r#color:Option<String>,
	pub r#weight:Option<i32>,
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
	let router = router.route("/widgets", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_list(i.as_ref(), WidgetsListRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsListResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsListResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsListResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/widgets", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_create(i.as_ref(), WidgetsCreateRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsCreateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsCreateResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsCreateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/widgets/{id}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_read(i.as_ref(), WidgetsReadRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsReadResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsReadResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsReadResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/widgets/{id}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_delete(i.as_ref(), WidgetsDeleteRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsDeleteResponse::Status204=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(204).unwrap()).body(axum::body::Body::empty()).unwrap(),
			WidgetsDeleteResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsDeleteResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/widgets/{id}", axum::routing::patch(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_update(i.as_ref(), WidgetsUpdateRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsUpdateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsUpdateResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsUpdateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/widgets/{id}/analyze", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widgets_analyze(i.as_ref(), WidgetsAnalyzeRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetsAnalyzeResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsAnalyzeResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetsAnalyzeResponse::Raw(v)=>v,
		}
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"parameters":{"Widget.id":{"in":"path","name":"id","required":true,"schema":{"type":"string"},"style":"simple"}},"schemas":{"Error":{"properties":{"code":{"format":"int32","type":"integer"},"message":{"type":"string"}},"required":["code","message"],"type":"object"},"Widget":{"properties":{"color":{"enum":["red","blue"],"type":"string"},"id":{"type":"string"},"weight":{"format":"int32","type":"integer"}},"required":["id","weight","color"],"type":"object"},"WidgetCreate":{"properties":{"color":{"enum":["red","blue"],"type":"string"},"weight":{"format":"int32","type":"integer"}},"required":["weight","color"],"type":"object"},"WidgetUpdate":{"properties":{"color":{"enum":["red","blue"],"type":"string"},"weight":{"format":"int32","type":"integer"}},"type":"object"}}},"info":{"title":"Widget Service","version":"0.0.0"},"openapi":"3.0.0","paths":{"/widgets":{"get":{"operationId":"Widgets_list","responses":{"200":{"content":{"application/json":{"schema":{"items":{"$ref":"#/components/schemas/Widget"},"type":"array"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]},"post":{"operationId":"Widgets_create","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/WidgetCreate"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]}},"/widgets/{id}":{"delete":{"operationId":"Widgets_delete","parameters":[{"in":"path","name":"id","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"204":{"description":"There is no content to send for this request, but the headers may be useful. "},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]},"get":{"operationId":"Widgets_read","parameters":[{"in":"path","name":"id","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]},"patch":{"operationId":"Widgets_update","parameters":[{"$ref":"#/components/parameters/Widget.id"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/WidgetUpdate"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]}},"/widgets/{id}/analyze":{"post":{"operationId":"Widgets_analyze","parameters":[{"in":"path","name":"id","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"type":"string"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}},"tags":["Widgets"]}}},"servers":[{"description":"Default server added by mandolin","url":"/api"}],"tags":[{"name":"Widgets"}]}"###
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
	// GET /widgets
	// async fn widgets_list(&self, _req: WidgetsListRequest) -> WidgetsListResponse{Default::default()}
	// POST /widgets
	// async fn widgets_create(&self, _req: WidgetsCreateRequest) -> WidgetsCreateResponse{Default::default()}
	// GET /widgets/{id}
	// async fn widgets_read(&self, _req: WidgetsReadRequest) -> WidgetsReadResponse{Default::default()}
	// DELETE /widgets/{id}
	// async fn widgets_delete(&self, _req: WidgetsDeleteRequest) -> WidgetsDeleteResponse{Default::default()}
	// PATCH /widgets/{id}
	// async fn widgets_update(&self, _req: WidgetsUpdateRequest) -> WidgetsUpdateResponse{Default::default()}
	// POST /widgets/{id}/analyze
	// async fn widgets_analyze(&self, _req: WidgetsAnalyzeRequest) -> WidgetsAnalyzeResponse{Default::default()}
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
