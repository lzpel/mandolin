














































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
}
// Response type for hello_say_hello
#[derive(Debug)]
pub enum HelloSayHelloResponse{
	Status200(String),
	Error(String),
}
impl Default for HelloSayHelloResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for HelloSayHelloResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for shape_compute
#[derive(Debug)]
pub struct ShapeComputeRequest{
	pub body: Box<ShapeNode>,
}
// Response type for shape_compute
#[derive(Debug)]
pub enum ShapeComputeResponse{
	Status200(Vec<u8>),
	Error(String),
}
impl Default for ShapeComputeResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for ShapeComputeResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "model/gltf-binary").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for step_exists
#[derive(Debug)]
pub struct StepExistsRequest{
	pub sha256:String,
}
// Response type for step_exists
#[derive(Debug)]
pub enum StepExistsResponse{
	Status200(FileExists),
	Error(String),
}
impl Default for StepExistsResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for StepExistsResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(serde_json::to_vec_pretty(&v).expect("error serialize response json"))).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
	}
}

// Request type for viewer_view
#[derive(Debug)]
pub struct ViewerViewRequest{
	pub sha256:String,
}
// Response type for viewer_view
#[derive(Debug)]
pub enum ViewerViewResponse{
	Status200(Vec<u8>),
	Error(String),
}
impl Default for ViewerViewResponse{
	fn default() -> Self{
		Self::Status200(Default::default())
	}
}
impl axum::response::IntoResponse for ViewerViewResponse{
	fn into_response(self) -> axum::response::Response{
		match self{
			Self::Status200(v)=> axum::response::Response::builder().status(http::StatusCode::from_u16(200).unwrap()).header(http::header::CONTENT_TYPE, "model/gltf-binary").body(axum::body::Body::from(v)).unwrap(),
			Self::Error(msg) => axum::response::Response::builder().status(500).header(http::header::CONTENT_TYPE, "text/plain").body(axum::body::Body::from(msg)).unwrap(),
		}
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
	pub r#a:Box<ShapeNode>,
	pub r#b:Box<ShapeNode>,
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
	pub r#shape:Box<ShapeNode>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct ScaleNode{
	pub r#factor:NumberOrExpr,
	pub r#shape:Box<ShapeNode>,
}

#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
#[serde(tag="op")]
pub enum ShapeNode{
	#[serde(rename="step")]
	Step(StepNode),
	#[serde(rename="union")]
	Union(UnionShapeNode),
	#[serde(rename="intersect")]
	Intersect(IntersectNode),
	#[serde(rename="subtract")]
	Subtract(SubtractNode),
	#[serde(rename="scale")]
	Scale(ScaleNode),
	#[serde(rename="translate")]
	Translate(TranslateNode),
	#[serde(rename="rotate")]
	Rotate(RotateNode),
	#[serde(rename="stretch")]
	Stretch(StretchNode),
}
impl Default for ShapeNode{fn default()->Self{Self::Step(Default::default())}}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct ShapeNodeBase{
	pub r#op:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct StepNode{
	pub r#content_hash:Option<String>,
	pub r#path:String,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct StretchNode{
	pub r#cut:Vec<NumberOrExpr>,
	pub r#delta:Vec<NumberOrExpr>,
	pub r#shape:Box<ShapeNode>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct SubtractNode{
	pub r#a:Box<ShapeNode>,
	pub r#b:Box<ShapeNode>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct TranslateNode{
	pub r#shape:Box<ShapeNode>,
	pub r#xyz:Vec<NumberOrExpr>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct UnionShapeNode{
	pub r#a:Box<ShapeNode>,
	pub r#b:Box<ShapeNode>,
}




// following part is only for client

#[cfg(feature = "mandolin_client")]
pub trait ApiClient {
    fn get_client(&self) -> &reqwest::Client;
    fn get_base_url(&self) -> &str;
}

#[cfg(feature = "mandolin_client")]
impl<T: ApiClient + Sync> ApiInterface for T {

    // GET /hello
    fn hello_say_hello(&self, req: HelloSayHelloRequest) -> impl Future<Output = HelloSayHelloResponse> + Send {
        let url = format!("{}{}", self.get_base_url(), "/hello"
        );
        let client = self.get_client().clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return HelloSayHelloResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.text().await { Ok(v) => HelloSayHelloResponse::Status200(v), Err(e) => HelloSayHelloResponse::Error(e.to_string()) },
                code => HelloSayHelloResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // POST /shape
    fn shape_compute(&self, req: ShapeComputeRequest) -> impl Future<Output = ShapeComputeResponse> + Send {
        let url = format!("{}{}", self.get_base_url(), "/shape"
        );
        let client = self.get_client().clone();
        async move {
            let r = match client.post(&url)
                .json(&req.body)
                .send().await {
                Ok(r) => r,
                Err(e) => return ShapeComputeResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.bytes().await { Ok(v) => ShapeComputeResponse::Status200(v.to_vec()), Err(e) => ShapeComputeResponse::Error(e.to_string()) },
                code => ShapeComputeResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /step/{sha256}
    fn step_exists(&self, req: StepExistsRequest) -> impl Future<Output = StepExistsResponse> + Send {
        let url = format!("{}{}", self.get_base_url(), "/step/{sha256}"
            .replace("{sha256}", &req.r#sha256.to_string())
        );
        let client = self.get_client().clone();
        async move {
            let r = match client.get(&url)
                .send().await {
                Ok(r) => r,
                Err(e) => return StepExistsResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.json().await { Ok(v) => StepExistsResponse::Status200(v), Err(e) => StepExistsResponse::Error(e.to_string()) },
                code => StepExistsResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }

    // GET /view
    fn viewer_view(&self, req: ViewerViewRequest) -> impl Future<Output = ViewerViewResponse> + Send {
        let url = format!("{}{}", self.get_base_url(), "/view"
        );
        let client = self.get_client().clone();
        async move {
            let r = match client.get(&url)
                .query(&[("sha256", req.r#sha256.to_string())])
                .send().await {
                Ok(r) => r,
                Err(e) => return ViewerViewResponse::Error(e.to_string()),
            };
            match r.status().as_u16() {
                200 =>
                    match r.bytes().await { Ok(v) => ViewerViewResponse::Status200(v.to_vec()), Err(e) => ViewerViewResponse::Error(e.to_string()) },
                code => ViewerViewResponse::Error(format!("unexpected status: {code}")),
            }
        }
    }
}

// following part is only for server

use axum;
use axum::http;
use axum::extract::FromRequest;

/// Axum-specific API interface trait
/// Implement this trait alongside ApiInterface to use axum_router.
/// Override methods here for axum-specific behavior (streaming, custom headers, etc.)
pub trait ApiInterfaceAxum: ApiInterface + Sync{
	/// Authentication process: Generate AuthContext from request
	fn authorize(&self, _req: http::Request<()>) -> impl Future<Output = Result<AuthContext, String>> + Send{async { Ok(Default::default()) } }

	// GET /hello
	fn hello_say_hello(&self, _raw: http::Request<()>, req: HelloSayHelloRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::hello_say_hello(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// POST /shape
	fn shape_compute(&self, _raw: http::Request<()>, req: ShapeComputeRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::shape_compute(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /step/{sha256}
	fn step_exists(&self, _raw: http::Request<()>, req: StepExistsRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::step_exists(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}

	// GET /view
	fn viewer_view(&self, _raw: http::Request<()>, req: ViewerViewRequest) -> impl Future<Output = axum::response::Response> + Send{
		let fut = <Self as ApiInterface>::viewer_view(self, req);
		async move{ axum::response::IntoResponse::into_response(fut.await) }
	}
}

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
	let router = router.route("/hello", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::hello_say_hello(i.as_ref(), http::Request::from_parts(parts.clone(), ()), HelloSayHelloRequest{
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/shape", axum::routing::post(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::shape_compute(i.as_ref(), http::Request::from_parts(parts.clone(), ()), ShapeComputeRequest{
			body:match axum::body::to_bytes(body, usize::MAX).await.map_err(|v| format!("{v:?}")).and_then(|v| serde_json::from_slice(&v).map_err(|v| v.to_string())) {Ok(v)=>v,Err(v)=>return text_response(http::StatusCode::BAD_REQUEST, v)},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/step/{sha256}", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::step_exists(i.as_ref(), http::Request::from_parts(parts.clone(), ()), StepExistsRequest{
			r#sha256:{let v=path.get("sha256").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: sha256 in path={:?}", path))}},
		}).await;
		ret
	}));

	let i = instance.clone();
	let router = router.route("/view", axum::routing::get(|
			path: axum::extract::Path<HashMap<String,String>>,
			query: axum::extract::Query<HashMap<String,String>>,
			header: http::HeaderMap,
			request: http::Request<axum::body::Body>,
		| async move{
			let (parts, body) = request.into_parts();
			let ret=<S as ApiInterfaceAxum>::viewer_view(i.as_ref(), http::Request::from_parts(parts.clone(), ()), ViewerViewRequest{
			r#sha256:{let v=query.get("sha256").and_then(|v| v.parse().ok());match v {Some(v)=>v, None=>return text_response(http::StatusCode::from_u16(400).unwrap(), format!("parse error: sha256 in query={:?}", query))}},
		}).await;
		ret
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

	// GET /hello
	// async fn hello_say_hello(&self, _req: HelloSayHelloRequest) -> HelloSayHelloResponse{Default::default()}

	// POST /shape
	// async fn shape_compute(&self, _req: ShapeComputeRequest) -> ShapeComputeResponse{Default::default()}

	// GET /step/{sha256}
	// async fn step_exists(&self, _req: StepExistsRequest) -> StepExistsResponse{Default::default()}

	// GET /view
	// async fn viewer_view(&self, _req: ViewerViewRequest) -> ViewerViewResponse{Default::default()}
}
impl ApiInterfaceAxum for TestServer{
	// Override for axum-specific behavior (e.g. custom auth, streaming, custom headers)
	// async fn authorize(&self, _req: http::Request<()>) -> Result<AuthContext, String>{ Ok(Default::default()) }

	// GET /hello
	// async fn hello_say_hello(&self, _raw: http::Request<()>, req: HelloSayHelloRequest) -> axum::response::Response{ axum::response::IntoResponse::into_response(<Self as ApiInterface>::hello_say_hello(self, req).await) }

	// POST /shape
	// async fn shape_compute(&self, _raw: http::Request<()>, req: ShapeComputeRequest) -> axum::response::Response{ axum::response::IntoResponse::into_response(<Self as ApiInterface>::shape_compute(self, req).await) }

	// GET /step/{sha256}
	// async fn step_exists(&self, _raw: http::Request<()>, req: StepExistsRequest) -> axum::response::Response{ axum::response::IntoResponse::into_response(<Self as ApiInterface>::step_exists(self, req).await) }

	// GET /view
	// async fn viewer_view(&self, _raw: http::Request<()>, req: ViewerViewRequest) -> axum::response::Response{ axum::response::IntoResponse::into_response(<Self as ApiInterface>::viewer_view(self, req).await) }
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
mod base64_serde {
	use serde::{Deserialize,Deserializer,Serializer};
	fn enc(b: &[u8]) -> String {
		const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
		b.chunks(3).flat_map(|c| {
			let n = c.iter().fold(0u32, |a,&b| a<<8|b as u32) << (8*(3-c.len()));
			[T[(n>>18&63)as usize], T[(n>>12&63)as usize],
			 if c.len()>1 {T[(n>>6&63)as usize]} else {b'='},
			 if c.len()>2 {T[(n&63)as usize]}    else {b'='}]
		}).map(|b| b as char).collect()
	}
	fn dec(s: &str) -> Result<Vec<u8>, String> {
		const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
		let v: Result<Vec<u8>,_> = s.bytes().filter(|&b| b!=b'=')
			.map(|b| T.iter().position(|&c|c==b).map(|i|i as u8).ok_or(format!("invalid base64 char: {b}")))
			.collect();
		Ok(v?.chunks(4).flat_map(|c| {
			let n = c.iter().fold(0u32, |a,&b| a<<6|b as u32) << (4-c.len())*6;
			(0..c.len()-1).map(move |i| (n>>(16-8*i)) as u8)
		}).collect())
	}
	pub fn serialize<S:Serializer>(b: &Vec<u8>, s: S) -> Result<S::Ok,S::Error> {
		s.serialize_str(&enc(b))
	}
	pub fn deserialize<'de,D:Deserializer<'de>>(d: D) -> Result<Vec<u8>,D::Error> {
		dec(&String::deserialize(d)?).map_err(serde::de::Error::custom)
	}
	pub mod opt {
		use serde::{Deserialize,Deserializer,Serializer};
		pub fn serialize<S:Serializer>(b: &Option<Vec<u8>>, s: S) -> Result<S::Ok,S::Error> {
			match b { Some(b) => s.serialize_some(&super::enc(b)), None => s.serialize_none() }
		}
		pub fn deserialize<'de,D:Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>,D::Error> {
			Option::<String>::deserialize(d)?.map(|s| super::dec(&s).map_err(serde::de::Error::custom)).transpose()
		}
	}
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
