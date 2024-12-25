use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, BufWriter, Error, Read, Write};
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
	pub fn new<R: Read>(reader: R) -> Result<Self, serde_yaml::Error> {
		Ok(Self {
			api: serde_yaml::from_reader(reader)?,
			tera: Tera::default()
		})
	}
	pub fn new_from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Result<Self, serde_yaml::Error>> {
		fs::File::open(path).map(|fi| Self::new(BufReader::new(fi)))
	}
	pub fn template<R: Read + 'static>(&mut self, mut reader: R) -> std::io::Result<&mut Self> {
		let mut content = String::new();
		reader.read_to_string(&mut content).map(|e|{
			self.tera.add_raw_template(self.tera.get_template_names().count().to_string().as_str(), content.as_str()).expect("tera parsing error");
			self
		})
	}
	pub fn template_from_path<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<&mut Self> {
		let f=fs::File::open(path)?;
		self.template(BufReader::new(f))
	}
	pub fn write<W: Write>(&self, mut writer: W) -> tera::Result<tera::Result<std::io::Result<()>>> {
		Context::from_serialize(&self.api).map(|context|{
			self.tera.render("main", &context).map(|v|{
				writer.write_all(v.as_bytes())
			})
		})
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn build() {
		crate::build("./test_openapi/openapi_petstore.yaml", "./output/openapi_petstore.rs").expect("TODO: panic message");
	}
	#[test]
	fn camel_case() {
		println!("{}", crate::camel_case("abc_def"))
	}
}