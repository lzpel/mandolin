pub mod builtin;

use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use openapiv3::{Content, MediaType, OpenAPI, Operation, Paths, ReferenceOr, RequestBody};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct LappedMediaType {
	pub media_type: String,
	#[serde(flatten)]
	pub media: MediaType,
}
impl TryFrom<&Content> for LappedMediaType {
	type Error = ();
	fn try_from(content: &Content) -> Result<Self, Self::Error> {
		if content.is_empty() {
			Err(())
		} else {
			let v = content.iter().filter(|(k, _)| k.contains("json")).next().unwrap_or(content.iter().next().expect("No content"));
			Ok(Self {
				media_type: v.0.to_string(),
				media: v.1.clone(),
			})
		}
	}
}
#[derive(Serialize, Deserialize)]
pub struct LappedRequestBody {
	pub default_content: LappedMediaType,
	pub identifier: String,
	#[serde(flatten)]
	pub request_body: RequestBody,
	//required
	//content
}
#[derive(Serialize, Deserialize)]
pub struct LappedOperation {
	pub path: String,
	pub method: String,
	pub function: String,
	pub request_identifier: String,
	pub response_identifier: String,
	//Option<ReferenceOr<RequestBody>>だがReference<>を許容しないのでOption<RequestBody>
	//Option<RequestBody>だがcontext_oneを追加してOption<LappedRequestBody>
	pub request_body: Option<LappedRequestBody>,
	#[serde(flatten)]
	pub operation: Operation,
}
impl LappedOperation {
	pub fn new(path: &str, method: &str, operation: &Operation) -> Self {
		let function = match &operation.operation_id {
			None => format!("{}{}", method, path).replace("/", "_"),
			Some(v) => v.clone()
		};
		let body = operation.request_body.as_ref().and_then(|v| {
			let request_body = v.as_item().expect("Referenced request body is not allowd");
			//jsonを含めば、json、それ以外ならbytes
			Some(LappedRequestBody {
				request_body: request_body.clone(),
				default_content: LappedMediaType::try_from(&request_body.content).unwrap(),
				identifier: camel_case(format!("request_body_{}", function.as_str()).as_str()),
			})
		});
		Self {
			path: path.to_string(),
			method: method.to_string(),
			operation: operation.clone(),
			function: function.clone(),
			request_identifier: camel_case(format!("request_{}", function.as_str()).as_str()),
			response_identifier: camel_case(format!("response_{}", function.as_str()).as_str()),
			request_body: body,
		}
	}
}

pub fn generate<R: Read, W: Write>(reader: R, mut writer: W) -> usize {
	//openAPI構造体を読み込む
	let openapi: OpenAPI = serde_yaml::from_reader(reader).expect("can't read openapi");
	let main_tera_template = include_str!("../builtin/main.tera");
	let result = {
		let mut this = Tera::default();
		this.add_raw_template("main", main_tera_template).expect("tera parsing error");
		//フィルターの登録
		this.register_filter("ref", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let i = tera::try_get_value!("ref", "value", ReferenceOr<()>, value);
			let v = match i {
				ReferenceOr::Reference { reference } => reference.replace("#/components/schemas/", "").to_string(),
				ReferenceOr::Item(_) => "".to_string(),
			};
			Ok(tera::to_value(v).unwrap())
		});
		this.register_filter("content_into_media", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let i = tera::try_get_value!("content_into_media", "value", Content, value);
			LappedMediaType::try_from(&i)
				.map(|v| tera::to_value(v).unwrap())
				.map_err(|_| tera::Error::from("content_into_media: no content"))
		});
		this.register_filter("paths_into_operations", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let paths = tera::try_get_value!("paths_into_operations", "value", Paths, value);
			let operations: Vec<LappedOperation> = paths.iter()
				.filter(|(_, path)| path.as_item().is_some())
				.map(|(path_key, path)| {
					let path = path.as_item().unwrap();
					[
						("get", path.get.clone()),
						("delete", path.delete.clone()),
						("head", path.head.clone()),
						("options", path.options.clone()),
						("patch", path.patch.clone()),
						("post", path.post.clone()),
						("put", path.put.clone()),
						("trace", path.trace.clone()),
					]
						.into_iter()
						.filter(|(_method, operation)| operation.is_some())
						.map(|(method, operation)| LappedOperation::new(path_key, method, &operation.unwrap()))
				})
				.flatten()
				.collect();
			tera::to_value(operations).map_err(|e| tera::Error::from(e.to_string()))
		});
		this.register_filter("re_replace", |value: &tera::Value, dict: &HashMap<String, tera::Value>| {
			let null = tera::to_value("").unwrap();
			let i = tera::try_get_value!("re_replace", "value", String, value);
			let f = tera::try_get_value!("re_replace", "from", String, dict.get("from").unwrap_or(&null));
			let t = tera::try_get_value!("re_replace", "to", String, dict.get("to").unwrap_or(&null));
			let re = Regex::new(f.as_str()).expect(format!("regex error: {}", f.as_str()).as_str());
			let o = re.replace_all(i.as_str(), t.as_str()).to_string();
			tera::to_value(o).map_err(|e| tera::Error::from(e.to_string()))
		});
		let cloned = openapi.clone();
		this.register_function("self", move |_: &HashMap<String, tera::Value>| {
			tera::to_value(&cloned).map_err(|e| tera::Error::from(e.to_string()))
		});
		let context = Context::from_serialize(&openapi).expect("failed to import context from openapi");
		this.render("main", &context).expect("tera render error")
	};
	writer.write_all(result.as_bytes()).expect("TODO: panic message");
	return 0;
}

pub fn build<P: AsRef<Path>>(openapi_path: P, rust_script_path: P) -> Result<(), std::io::Error> {
	let fi = fs::File::open(openapi_path)?;
	let fo = fs::File::create(rust_script_path)?;
	generate(BufReader::new(fi), BufWriter::new(fo));
	return Ok(());
}
fn camel_case(text: &str) -> String {
	let re = Regex::new(r"_+(.)").unwrap();
	let text_with_capital = format!("_{}", text);
	let result = re.replace_all(text_with_capital.as_str(), |caps: &regex::Captures| {
		format!("{}", &caps[1].to_uppercase())
	});
	result.to_string()
}

pub struct Mandolin {
	api: OpenAPI,
	tera: Tera
}
impl Mandolin {
	pub fn new(api: OpenAPI) -> Result<Self, serde_yaml::Error> {
		let mut this = Tera::default();
		//フィルターの登録
		this.register_filter("ref", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let i = tera::try_get_value!("ref", "value", ReferenceOr<()>, value);
			let v = match i {
				ReferenceOr::Reference { reference } => reference.replace("#/components/schemas/", "").to_string(),
				ReferenceOr::Item(_) => "".to_string(),
			};
			Ok(tera::to_value(v).unwrap())
		});
		this.register_filter("content_into_media", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let i = tera::try_get_value!("content_into_media", "value", Content, value);
			LappedMediaType::try_from(&i)
				.map(|v| tera::to_value(v).unwrap())
				.map_err(|_| tera::Error::from("content_into_media: no content"))
		});
		this.register_filter("paths_into_operations", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
			let paths = tera::try_get_value!("paths_into_operations", "value", Paths, value);
			let operations: Vec<LappedOperation> = paths.iter()
				.filter(|(_, path)| path.as_item().is_some())
				.map(|(path_key, path)| {
					let path = path.as_item().unwrap();
					[
						("get", path.get.clone()),
						("delete", path.delete.clone()),
						("head", path.head.clone()),
						("options", path.options.clone()),
						("patch", path.patch.clone()),
						("post", path.post.clone()),
						("put", path.put.clone()),
						("trace", path.trace.clone()),
					]
						.into_iter()
						.filter(|(_method, operation)| operation.is_some())
						.map(|(method, operation)| LappedOperation::new(path_key, method, &operation.unwrap()))
				})
				.flatten()
				.collect();
			tera::to_value(operations).map_err(|e| tera::Error::from(e.to_string()))
		});
		this.register_filter("re_replace", |value: &tera::Value, dict: &HashMap<String, tera::Value>| {
			let null = tera::to_value("").unwrap();
			let i = tera::try_get_value!("re_replace", "value", String, value);
			let f = tera::try_get_value!("re_replace", "from", String, dict.get("from").unwrap_or(&null));
			let t = tera::try_get_value!("re_replace", "to", String, dict.get("to").unwrap_or(&null));
			let re = Regex::new(f.as_str()).expect(format!("regex error: {}", f.as_str()).as_str());
			let o = re.replace_all(i.as_str(), t.as_str()).to_string();
			tera::to_value(o).map_err(|e| tera::Error::from(e.to_string()))
		});
		Ok(Self {
			api: api,
			tera: this
		})
	}
	pub fn template(&mut self, template: &str) -> &mut Self {
		self.tera.add_raw_template("main", template).expect("tera parsing error");
		self
	}
	pub fn template_from_path<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<&mut Self> {
		let f=fs::read_to_string(path)?;
		Ok(self.template(f.as_str()))
	}

	pub fn render(&self) -> Result<String, tera::Error> {
		let context=Context::from_serialize(&self.api)?;
		self.tera.render("main", &context)
	}
}
pub fn sandbox1(x: &str)-> String{
	let x: OpenAPI = serde_yaml::from_str(x).unwrap();
	serde_json::to_string(&x).unwrap()
}

#[macro_export] macro_rules! path_defined { () => { file!() }; }
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_build() {
		build("./openapi/openapi_petstore.yaml", "./output/openapi_petstore.rs").expect("TODO: panic message");
	}
	#[test]
	fn test_camel_case() {
		println!("{}", camel_case("abc_def"))
	}
	#[test]
	fn test_sandbox1(){
		println!("{}", sandbox1(include_str!("../openapi/openapi_petstore.yaml")));
		println!("{}", path_defined!())
	}
}