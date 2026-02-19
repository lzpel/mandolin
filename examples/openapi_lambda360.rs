












































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
	// GET /hello
	fn hello_say_hello(&self, _req: HelloSayHelloRequest) -> impl Future<Output = HelloSayHelloResponse> + Send{async{Default::default()}}
	// POST /shape
	fn shape_compute(&self, _req: ShapeComputeRequest) -> impl Future<Output = ShapeComputeResponse> + Send{async{Default::default()}}
	// GET /step/{sha256}
	fn step_exists(&self, _req: StepExistsRequest) -> impl Future<Output = StepExistsResponse> + Send{async{Default::default()}}
	// GET /view
	fn viewer_view(&self, _req: ViewerViewRequest) -> impl Future<Output = ViewerViewResponse> + Send{async{Default::default()}}
}


/// Auth Context: Struct to hold authentication information
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct AuthContext{
    pub subject: String,   // User identifier (e.g., "auth0|123", "google-oauth2|456")
    pub subject_id: u128,  // UUID compatible numeric ID
    pub scopes: Vec<String>, // Scopes (e.g., "read:foo", "write:bar")
}


// Request type for hello_say_hello
#[derive(Debug)]
pub struct HelloSayHelloRequest{
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for HelloSayHelloRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for hello_say_hello
#[derive(Debug)]
pub enum HelloSayHelloResponse{
	Status200(String),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for HelloSayHelloResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for shape_compute
#[derive(Debug)]
pub struct ShapeComputeRequest{
	pub body: ShapeNode,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for ShapeComputeRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for shape_compute
#[derive(Debug)]
pub enum ShapeComputeResponse{
	Status200(Vec<u8>),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for ShapeComputeResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for step_exists
#[derive(Debug)]
pub struct StepExistsRequest{
	pub sha256:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for StepExistsRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for step_exists
#[derive(Debug)]
pub enum StepExistsResponse{
	Status200(FileExists),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for StepExistsResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
// Request type for viewer_view
#[derive(Debug)]
pub struct ViewerViewRequest{
	pub sha256:String,
	pub request: axum::http::Request<axum::body::Body>,
}
impl AsRef<axum::http::Request<axum::body::Body>> for ViewerViewRequest{
	fn as_ref(&self) -> &axum::http::Request<axum::body::Body>{&self.request}
}
// Response type for viewer_view
#[derive(Debug)]
pub enum ViewerViewResponse{
	Status200(Vec<u8>),
	Raw(axum::response::Response),// Variant for custom responses
}
impl Default for ViewerViewResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}





#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct FileExists{
	pub r#exists:bool,
	pub r#expiresAt:Option<chrono::DateTime<chrono::Utc>>,
	pub r#uploadUrl:Option<String>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct IntersectNode{
	pub r#a:ShapeNode,
	pub r#b:ShapeNode,
	pub r#op:String,
}

#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
#[serde(untagged)]
pub enum NumberOrExpr{
	Variant0(f64),
	Variant1(String),
}
impl Default for NumberOrExpr{fn default()->Self{Self::Variant0(Default::default())}}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct RotateNode{
	pub r#axis:Vec<NumberOrExpr>,
	pub r#deg:NumberOrExpr,
	pub r#op:String,
	pub r#shape:ShapeNode,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct ScaleNode{
	pub r#factor:NumberOrExpr,
	pub r#op:String,
	pub r#shape:ShapeNode,
}

#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
#[serde(untagged)]
pub enum ShapeNode{
	Variant0(StepNode),
	Variant1(UnionShapeNode),
	Variant2(IntersectNode),
	Variant3(SubtractNode),
	Variant4(ScaleNode),
	Variant5(TranslateNode),
	Variant6(RotateNode),
	Variant7(StretchNode),
}
impl Default for ShapeNode{fn default()->Self{Self::Variant0(Default::default())}}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct ShapeNodeBase{
	pub r#op:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct StepNode{
	pub r#content_hash:Option<String>,
	pub r#op:String,
	pub r#path:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct StretchNode{
	pub r#cut:Vec<NumberOrExpr>,
	pub r#delta:Vec<NumberOrExpr>,
	pub r#op:String,
	pub r#shape:ShapeNode,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct SubtractNode{
	pub r#a:ShapeNode,
	pub r#b:ShapeNode,
	pub r#op:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct TranslateNode{
	pub r#op:String,
	pub r#shape:ShapeNode,
	pub r#xyz:Vec<NumberOrExpr>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct UnionShapeNode{
	pub r#a:ShapeNode,
	pub r#b:ShapeNode,
	pub r#op:String,
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
	let router = router.route("/hello", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::hello_say_hello(i.as_ref(), HelloSayHelloRequest{
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			HelloSayHelloResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			HelloSayHelloResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/shape", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::shape_compute(i.as_ref(), ShapeComputeRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(axum::http::StatusCode::BAD_REQUEST, v)},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			ShapeComputeResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "model/gltf-binary").body(axum::body::Body::from(v)).unwrap(),
			ShapeComputeResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/step/{sha256}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::step_exists(i.as_ref(), StepExistsRequest{
			r#sha256:{let v=path.get("sha256").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: sha256 in path={:?}", path))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			StepExistsResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			StepExistsResponse::Raw(v)=>v,
		}
	}));
	let i = instance.clone();
	let router = router.route("/view", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: axum::http::HeaderMap,
			request: axum::http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=S::viewer_view(i.as_ref(), ViewerViewRequest{
			r#sha256:{let v=query.get("sha256").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(axum::http::StatusCode::from_u16(400).unwrap(), format!("parse error: sha256 in query={:?}", query))}},
			request: axum::http::Request::from_parts(parts.clone(), Default::default()),
		}).await;
		match ret{
			ViewerViewResponse::Status200(v)=> axum::response::Response::builder().status(axum::http::StatusCode::from_u16(200).unwrap()).header(axum::http::header::CONTENT_TYPE, "model/gltf-binary").body(axum::body::Body::from(v)).unwrap(),
			ViewerViewResponse::Raw(v)=>v,
		}
	}));
	let router = router.route("/openapi.json", axum::routing::get(|| async move{
			r###"{"components":{"schemas":{"FileExists":{"properties":{"exists":{"type":"boolean"},"expiresAt":{"format":"date-time","type":"string"},"uploadUrl":{"type":"string"}},"required":["exists"],"type":"object"},"IntersectNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"ブーリアン共通部分 (BRepAlgoAPI_Common)","properties":{"a":{"$ref":"#/components/schemas/ShapeNode"},"b":{"$ref":"#/components/schemas/ShapeNode"},"op":{"enum":["intersect"],"type":"string"}},"required":["op","a","b"],"type":"object"},"NumberOrExpr":{"anyOf":[{"format":"double","type":"number"},{"type":"string"}],"description":"数値定数または $式 (例: 100.0, \"$width\", \"$width * 0.5 + 50\")"},"RotateNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"回転","properties":{"axis":{"description":"回転軸ベクトル [ax, ay, az]","items":{"$ref":"#/components/schemas/NumberOrExpr"},"type":"array"},"deg":{"allOf":[{"$ref":"#/components/schemas/NumberOrExpr"}],"description":"回転角度 (度)"},"op":{"enum":["rotate"],"type":"string"},"shape":{"$ref":"#/components/schemas/ShapeNode"}},"required":["op","shape","axis","deg"],"type":"object"},"ScaleNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"一様拡大縮小","properties":{"factor":{"$ref":"#/components/schemas/NumberOrExpr"},"op":{"enum":["scale"],"type":"string"},"shape":{"$ref":"#/components/schemas/ShapeNode"}},"required":["op","shape","factor"],"type":"object"},"ShapeNode":{"anyOf":[{"$ref":"#/components/schemas/StepNode"},{"$ref":"#/components/schemas/UnionShapeNode"},{"$ref":"#/components/schemas/IntersectNode"},{"$ref":"#/components/schemas/SubtractNode"},{"$ref":"#/components/schemas/ScaleNode"},{"$ref":"#/components/schemas/TranslateNode"},{"$ref":"#/components/schemas/RotateNode"},{"$ref":"#/components/schemas/StretchNode"}],"description":"★ここが主役：discriminated union を “ShapeNode” として定義\nこれが OpenAPI で oneOf + discriminator になりやすい"},"ShapeNodeBase":{"description":"形状演算ノードの共通フィールド（任意）\n※これは OpenAPI の oneOf 生成のために必須ではないが、共通項を置きたい場合に便利","properties":{"op":{"type":"string"}},"required":["op"],"type":"object"},"StepNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"STEPファイルの読み込み","properties":{"content_hash":{"description":"キャッシュ無効化用コンテンツハッシュ \"sha256:\u003chex64\u003e\"","type":"string"},"op":{"enum":["step"],"type":"string"},"path":{"description":"STEPファイルのパス (S3キー等)","type":"string"}},"required":["op","path"],"type":"object"},"StretchNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"伸縮: 切断面で形状を分割して指定方向に伸ばす","properties":{"cut":{"description":"切断面の座標 [cx, cy, cz] (mm)","items":{"$ref":"#/components/schemas/NumberOrExpr"},"type":"array"},"delta":{"description":"各軸方向の伸縮量 [dx, dy, dz] (mm)","items":{"$ref":"#/components/schemas/NumberOrExpr"},"type":"array"},"op":{"enum":["stretch"],"type":"string"},"shape":{"$ref":"#/components/schemas/ShapeNode"}},"required":["op","shape","cut","delta"],"type":"object"},"SubtractNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"ブーリアン差演算: a から b をくり抜く (BRepAlgoAPI_Cut)","properties":{"a":{"$ref":"#/components/schemas/ShapeNode"},"b":{"$ref":"#/components/schemas/ShapeNode"},"op":{"enum":["subtract"],"type":"string"}},"required":["op","a","b"],"type":"object"},"TranslateNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"平行移動","properties":{"op":{"enum":["translate"],"type":"string"},"shape":{"$ref":"#/components/schemas/ShapeNode"},"xyz":{"description":"移動量 [x, y, z] (mm)","items":{"$ref":"#/components/schemas/NumberOrExpr"},"type":"array"}},"required":["op","shape","xyz"],"type":"object"},"UnionShapeNode":{"allOf":[{"$ref":"#/components/schemas/ShapeNodeBase"}],"description":"ブーリアン合体 (BRepAlgoAPI_Fuse)","properties":{"a":{"$ref":"#/components/schemas/ShapeNode"},"b":{"$ref":"#/components/schemas/ShapeNode"},"op":{"enum":["union"],"type":"string"}},"required":["op","a","b"],"type":"object"}}},"info":{"title":"Lambda360 API","version":"0.0.0"},"openapi":"3.0.0","paths":{"/hello":{"get":{"operationId":"Hello_sayHello","responses":{"200":{"content":{"text/plain":{"schema":{"type":"string"}}},"description":"The request has succeeded."}}}},"/shape":{"post":{"description":"ShapeNode を受け取り、演算結果を GLB (GLTF Binary) として返す","operationId":"Shape_compute","requestBody":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/ShapeNode"}}},"required":true},"responses":{"200":{"content":{"model/gltf-binary":{"schema":{"format":"binary","type":"string"}}},"description":"The request has succeeded."}}}},"/step/{sha256}":{"get":{"operationId":"Step_exists","parameters":[{"in":"path","name":"sha256","required":true,"schema":{"type":"string"},"style":"simple"}],"responses":{"200":{"content":{"application/json":{"schema":{"$ref":"#/components/schemas/FileExists"}}},"description":"The request has succeeded."}}}},"/view":{"get":{"operationId":"Viewer_view","parameters":[{"explode":false,"in":"query","name":"sha256","required":true,"schema":{"type":"string"},"style":"form"}],"responses":{"200":{"content":{"model/gltf-binary":{"schema":{"format":"binary","type":"string"}}},"description":"The request has succeeded."}}}}},"servers":[{"description":"Main server","url":"/api","variables":{}}]}"###
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
	// GET /hello
	// async fn hello_say_hello(&self, _req: HelloSayHelloRequest) -> HelloSayHelloResponse{Default::default()}
	// POST /shape
	// async fn shape_compute(&self, _req: ShapeComputeRequest) -> ShapeComputeResponse{Default::default()}
	// GET /step/{sha256}
	// async fn step_exists(&self, _req: StepExistsRequest) -> StepExistsResponse{Default::default()}
	// GET /view
	// async fn viewer_view(&self, _req: ViewerViewRequest) -> ViewerViewResponse{Default::default()}
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

fn canonicalize_jcs_from_str(json: &str) -> Result<String, String> {
    // 1) まず JSON として解釈できることを保証（ここで構文エラーは落とす）
    // 2) RFC8785(JCS)で canonical 文字列へ
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    serde_jcs::to_string(&v).map_err(|e| format!("{e:?}"))
}

fn canonicalize_jcs_from_shape(shape: &ShapeNode) -> Result<String, String> {
    serde_jcs::to_string(shape).map_err(|e| format!("{e:?}"))
}

fn main() {
    // 1) StepNode
    let j1 = r#"
    {
      "op": "step",
      "path": "s3://lambda360/parts/PA-001-DF7.STEP",
      "content_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }
    "#;

    // 2) TranslateNode
    let j2 = r#"
    {
      "op": "translate",
      "shape": {
        "op": "step",
        "path": "s3://lambda360/parts/PA-001-DF7.STEP",
        "content_hash": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
      },
      "xyz": [10, 0, -5]
    }
    "#;

    // 3) SubtractNode（NumberOrExpr に文字列式も混ぜる）
    let j3 = r#"
    {
      "op": "subtract",
      "a": {
        "op": "union",
        "a": {
          "op": "step",
          "path": "s3://lambda360/parts/base.STEP",
          "content_hash": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        },
        "b": {
          "op": "translate",
          "shape": {
            "op": "step",
            "path": "s3://lambda360/parts/boss.STEP",
            "content_hash": "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
          },
          "xyz": ["$dx", 0, 0]
        }
      },
      "b": {
        "op": "rotate",
        "shape": {
          "op": "step",
          "path": "s3://lambda360/parts/hole.STEP",
          "content_hash": "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        },
        "axis": [0, 0, 1],
        "deg": "$angleDeg"
      }
    }
    "#;

    let cases = [("case1_step", j1), ("case2_translate", j2), ("case3_subtract", j3)];

    for (name, old_json) in cases {
        println!("=== {name} ===");

        // before: 入力JSONをRFC8785(JCS)で正規化
        let old_canon = canonicalize_jcs_from_str(old_json)
            .unwrap_or_else(|e| panic!("canonicalize(before) failed: {e}"));

        // parse: ShapeNodeへデシリアライズ（ここで「パースが正しい」ことを検証）
        let shape: ShapeNode = serde_json::from_str(old_json)
            .unwrap_or_else(|e| panic!("parse ShapeNode failed: {e}"));

        // after: ShapeNodeをRFC8785(JCS)で正規化シリアライズ
        let new_canon = canonicalize_jcs_from_shape(&shape)
            .unwrap_or_else(|e| panic!("canonicalize(after) failed: {e}"));

        // “情報が変化していない”ことを canonical JSON 文字列で保証
        assert_eq!(
            old_canon, new_canon,
            "JCS mismatch: before != after\n--- before(JCS) ---\n{old_canon}\n--- after(JCS) ---\n{new_canon}\n"
        );

        println!("OK: JCS(before)==JCS(after)");
        // 必要ならダンプ
        // println!("{old_canon}");
    }

    println!("All cases passed.");
}
