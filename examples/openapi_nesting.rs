












































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
	// GET /
	fn widget_service_list(&self, _req: WidgetServiceListRequest) -> impl Future<Output = WidgetServiceListResponse> + Send{async{Default::default()}}
	// POST /
	fn widget_service_create(&self, _req: WidgetServiceCreateRequest) -> impl Future<Output = WidgetServiceCreateResponse> + Send{async{Default::default()}}
	// GET /customGet
	fn widget_service_custom_get(&self, _req: WidgetServiceCustomGetRequest) -> impl Future<Output = WidgetServiceCustomGetResponse> + Send{async{Default::default()}}
	// GET /{id}
	fn widget_service_get(&self, _req: WidgetServiceGetRequest) -> impl Future<Output = WidgetServiceGetResponse> + Send{async{Default::default()}}
	// DELETE /{id}
	fn widget_service_delete(&self, _req: WidgetServiceDeleteRequest) -> impl Future<Output = WidgetServiceDeleteResponse> + Send{async{Default::default()}}
	// PATCH /{id}
	fn widget_service_update(&self, _req: WidgetServiceUpdateRequest) -> impl Future<Output = WidgetServiceUpdateResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for widget_service_list
#[derive(Debug)]
pub struct WidgetServiceListRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceListRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_list
#[derive(Debug)]
pub enum WidgetServiceListResponse{
	Status200(WidgetCollectionWithNextLink),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceListResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widget_service_create
#[derive(Debug)]
pub struct WidgetServiceCreateRequest{
	pub body: WidgetCreate,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceCreateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_create
#[derive(Debug)]
pub enum WidgetServiceCreateResponse{
	Status200(Widget),
	Status201(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceCreateResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widget_service_custom_get
#[derive(Debug)]
pub struct WidgetServiceCustomGetRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceCustomGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_custom_get
#[derive(Debug)]
pub enum WidgetServiceCustomGetResponse{
	Status200(Widget),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceCustomGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widget_service_get
#[derive(Debug)]
pub struct WidgetServiceGetRequest{
	pub id:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceGetRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_get
#[derive(Debug)]
pub enum WidgetServiceGetResponse{
	Status200(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceGetResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for widget_service_delete
#[derive(Debug)]
pub struct WidgetServiceDeleteRequest{
	pub id:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceDeleteRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_delete
#[derive(Debug)]
pub enum WidgetServiceDeleteResponse{
	Status200,
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceDeleteResponse{
	fn default() -> Self{
		Self::Status200
	}
}
// Request type for widget_service_update
#[derive(Debug)]
pub struct WidgetServiceUpdateRequest{
	pub id:String,
	pub body: WidgetUpdate,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for WidgetServiceUpdateRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for widget_service_update
#[derive(Debug)]
pub enum WidgetServiceUpdateResponse{
	Status200(Widget),
	StatusDefault(Error),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for WidgetServiceUpdateResponse{
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
pub struct Inner{
	pub r#latitude:f64,
	pub r#longitude:f64,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct InnerUpdate{
	pub r#latitude:Option<f64>,
	pub r#longitude:Option<f64>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Widget{
	pub r#color:String,
	pub r#id:String,
	pub r#pos:Inner,
	pub r#weight:i32,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct WidgetCollectionWithNextLink{
	pub r#nextLink:Option<String>,
	pub r#value:Vec<Widget>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct WidgetCreate{
	pub r#color:String,
	pub r#pos:Inner,
	pub r#weight:i32,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct WidgetUpdate{
	pub r#color:Option<String>,
	pub r#pos:Option<InnerUpdate>,
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
	let router = router.route("/", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_list(i.as_ref(), WidgetServiceListRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceListResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceListResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceListResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_create(i.as_ref(), WidgetServiceCreateRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceCreateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceCreateResponse::Status201(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(201).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceCreateResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceCreateResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/customGet", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_custom_get(i.as_ref(), WidgetServiceCustomGetRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceCustomGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceCustomGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/{id}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_get(i.as_ref(), WidgetServiceGetRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceGetResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceGetResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceGetResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/{id}", axum::routing::delete(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_delete(i.as_ref(), WidgetServiceDeleteRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceDeleteResponse::Status200=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).body(axum::body::Body::empty()).unwrap(),
			WidgetServiceDeleteResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceDeleteResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/{id}", axum::routing::patch(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::widget_service_update(i.as_ref(), WidgetServiceUpdateRequest{
			r#id:{let v=path.get("id").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: id in path={:?}", path))}},
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			WidgetServiceUpdateResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceUpdateResponse::StatusDefault(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(500).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			WidgetServiceUpdateResponse::Raw(v)=>v,
		}
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"parameters":{"WidgetKey":{"in":"path","name":"id","required":true,"schema":{"type":"string"},"style":"simple"}},"schemas":{"Error":{"properties":{"code":{"format":"int32","type":"integer"},"message":{"type":"string"}},"required":["code","message"],"type":"object"},"Inner":{"properties":{"latitude":{"type":"number"},"longitude":{"type":"number"}},"required":["latitude","longitude"],"type":"object"},"InnerUpdate":{"properties":{"latitude":{"type":"number"},"longitude":{"type":"number"}},"type":"object"},"Widget":{"properties":{"color":{"enum":["red","blue"],"type":"string"},"id":{"type":"string"},"pos":{"$ref":"#/components/schemas/Inner"},"weight":{"format":"int32","type":"integer"}},"required":["id","weight","color","pos"],"type":"object"},"WidgetCollectionWithNextLink":{"description":"Paged response of Widget items","properties":{"nextLink":{"description":"The link to the next page of items","format":"uri","type":"string"},"value":{"description":"The items on this page","items":{"$ref":"#/components/schemas/Widget"},"type":"array"}},"required":["value"],"type":"object"},"WidgetCreate":{"description":"Resource create operation model.","properties":{"color":{"enum":["red","blue"],"type":"string"},"pos":{"$ref":"#/components/schemas/Inner"},"weight":{"format":"int32","type":"integer"}},"required":["weight","color","pos"],"type":"object"},"WidgetUpdate":{"description":"Resource create or update operation model.","properties":{"color":{"enum":["red","blue"],"type":"string"},"pos":{"$ref":"#/components/schemas/InnerUpdate"},"weight":{"format":"int32","type":"integer"}},"type":"object"}}},"info":{"title":"Widget Service","version":"0.0.0"},"openapi":"3.0.0","paths":{"/":{"get":{"description":"Lists all instances of the resource.","operationId":"WidgetService_list","responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/WidgetCollectionWithNextLink"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}}},"post":{"description":"Creates a new instance of the resource.","operationId":"WidgetService_create","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/WidgetCreate"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"201":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"Resource create operation completed successfully."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}}}},"/customGet":{"get":{"operationId":"WidgetService_customGet","responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."}}}},"/{id}":{"delete":{"description":"Deletes an existing instance of the resource.","operationId":"WidgetService_delete","parameters":[{"$ref":"#/components/parameters/WidgetKey"}],"responses":{"200":{"description":"Resource deleted successfully."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}}},"get":{"description":"Gets an instance of the resource.","operationId":"WidgetService_get","parameters":[{"$ref":"#/components/parameters/WidgetKey"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}}},"patch":{"description":"Updates an existing instance of the resource.","operationId":"WidgetService_update","parameters":[{"$ref":"#/components/parameters/WidgetKey"}],"requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/WidgetUpdate"}}},"required":true},"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Widget"}}},"description":"The request has succeeded."},"default":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/Error"}}},"description":"An unexpected error response."}}}}},"servers":[{"description":"Default server added by mandolin","url":"/api"}]}"###
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
	// GET /
	// async fn widget_service_list(&self, _req: WidgetServiceListRequest) -> WidgetServiceListResponse{Default::default()}
	// POST /
	// async fn widget_service_create(&self, _req: WidgetServiceCreateRequest) -> WidgetServiceCreateResponse{Default::default()}
	// GET /customGet
	// async fn widget_service_custom_get(&self, _req: WidgetServiceCustomGetRequest) -> WidgetServiceCustomGetResponse{Default::default()}
	// GET /{id}
	// async fn widget_service_get(&self, _req: WidgetServiceGetRequest) -> WidgetServiceGetResponse{Default::default()}
	// DELETE /{id}
	// async fn widget_service_delete(&self, _req: WidgetServiceDeleteRequest) -> WidgetServiceDeleteResponse{Default::default()}
	// PATCH /{id}
	// async fn widget_service_update(&self, _req: WidgetServiceUpdateRequest) -> WidgetServiceUpdateResponse{Default::default()}
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
