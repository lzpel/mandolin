pub mod templates;

use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind};
use serde::Deserialize;
use std::collections::BTreeMap;

pub struct Mandolin {
	api: OpenAPI,
	templates: Vec<String>,
}
impl Mandolin {
	pub fn new(api: OpenAPI) -> Self {
		Self {
			api,
			templates: vec![],
		}
	}
	pub fn template<T: AsRef<str>>(&mut self, template: T) -> &mut Self {
		self.templates.push(template.as_ref().to_string());
		self
	}
	pub fn decode<S: AsRef<str>>(content: S) -> String {
		content.as_ref().replace("~0", "~").replace("~1", "/") // RFC6901
	}
	pub fn decode_list<S: AsRef<str>>(content: S) -> Vec<String> {
		content
			.as_ref()
			.split("/")
			.map(|v| Self::decode(v))
			.collect()
	}
	pub fn encode<S: AsRef<str>>(content: S) -> String {
		content.as_ref().replace("~", "~0").replace("/", "~1") // RFC6901
	}
	pub fn snake_case<S: AsRef<str>>(s: S) -> String {
		let mut result = String::new();
		let mut flag = false;
		for c in s.as_ref().chars() {
			if c.is_ascii_alphanumeric() == false {
				flag = true; //前置判定
			} else {
				flag |= c.is_uppercase();
				flag &= !result.is_empty();
				//↑当該判定
				if flag {
					result.push('_')
				}
				result.push(c.to_ascii_lowercase());
				flag = false;
			}
		}
		result
	}
	pub fn pascal_case<S: AsRef<str>>(s: S) -> String {
		let mut result = String::new();
		let mut flag = false;
		for c in s.as_ref().chars() {
			if c.is_ascii_alphanumeric() == false {
				flag = true; //前置判定
			} else {
				flag |= result.is_empty();
				//↑当該判定
				if flag {
					result.push(c.to_ascii_uppercase())
				} else {
					result.push(c)
				}
				flag = false;
			}
		}
		result
	}
	fn p(
		api: minijinja::Value,
		path: &str,
		no_err: bool,
	) -> Result<minijinja::Value, minijinja::Error> {
		let default = if no_err {
			Ok(minijinja::Value::UNDEFINED)
		} else {
			Err(minijinja::Error::new(
				minijinja::ErrorKind::NonKey,
				format!("p: {path} not found"),
			))
		};
		let mut parent = api;
		for p0 in path.split("/").skip(1) {
			let p = Self::decode(&p0);
			parent = if let Some(map) = parent.as_object() {
				match map.get_value(&minijinja::Value::from(&p)) {
					None => match p
						.parse::<usize>()
						.ok()
						.and_then(|i| map.get_value(&minijinja::Value::from(i)))
					{
						None => return default,
						Some(latest) => latest,
					},
					Some(latest) => latest,
				}
			} else {
				return default;
			}
		}
		Ok(parent)
	}
	fn r_base(
		api: &minijinja::Value,
		value: minijinja::Value,
		no_err: bool,
	) -> Result<minijinja::Value, minijinja::Error> {
		match ReferenceOr::<()>::deserialize(&value) {
			Ok(ReferenceOr::Reference { reference }) => {
				Self::p(api.clone(), reference.as_str(), no_err)
			}
			_ => Ok(value),
		}
	}
	fn r(
		api: &minijinja::Value,
		value: minijinja::Value,
		no_err: bool,
	) -> Result<minijinja::Value, minijinja::Error> {
		let v = Self::r_base(api, value, no_err)?;
		if let Ok(v) = BTreeMap::<minijinja::Value, minijinja::Value>::deserialize(&v) {
			return v
				.into_iter()
				.map(|(k, v)| Self::r_base(api, v, no_err).map(|v| (k, v)))
				.collect();
		} else if let Ok(v) = Vec::<minijinja::Value>::deserialize(&v) {
			return v
				.into_iter()
				.map(|v| Self::r_base(api, v, no_err))
				.collect();
		}
		Ok(v)
	}
	fn pr<'a>(
		api: minijinja::Value,
		path: &str,
		no_err: bool,
	) -> Result<minijinja::Value, minijinja::Error> {
		let v = Self::p(api.clone(), path, no_err)?;
		Self::r(&api, v, false)
	}
	fn recursive_pointed_objects(
		path: String,
		value: &minijinja::Value,
		output: &mut Vec<(String, minijinja::Value)>,
	) {
		output.push((path.clone(), value.clone())); //注目箇所を追加
		if let Some(v) = value.as_object() {
			//子を検索
			if let Some(v) = v.try_iter_pairs() {
				v.for_each(|(k, v)| {
					Self::recursive_pointed_objects(
						format!("{path}/{}", Self::encode(k.to_string())),
						&v,
						output,
					)
				});
			} else if let Some(v) = v.try_iter() {
				v.enumerate().for_each(|(k, v)| {
					Self::recursive_pointed_objects(format!("{path}/{k}"), &v, output)
				});
			}
		}
	}
	pub fn render(&self) -> Result<String, minijinja::Error> {
		let mut env = minijinja::Environment::new();
		let api = minijinja::Value::from_serialize(&self.api);
		let map_pointed_objects: Vec<(String, minijinja::Value)> = {
			let mut output = Default::default();
			Self::recursive_pointed_objects("#".to_string(), &api, &mut output);
			output
		};
		env.add_filter("snake_case", |value: &str| Self::snake_case(value));
		env.add_filter("pascal_case", |value: &str| Self::pascal_case(value));
		env.add_filter("decode", |value: &str| Self::decode(value));
		env.add_filter("encode", |value: &str| Self::encode(value));
		env.add_filter("decode_list", |value: &str| Self::decode_list(value));
		{
			let api = api.clone();
			env.add_filter("p", move |value: &minijinja::Value| {
				Self::p(api.clone(), value.as_str().unwrap_or_default(), true)
			});
		}
		{
			let api = api.clone();
			env.add_filter("r", move |value: minijinja::Value| {
				Self::r(&api, value, false)
			});
		}
		{
			let api = api.clone();
			env.add_filter("pr", move |value: &minijinja::Value| {
				Self::pr(api.clone(), value.as_str().unwrap_or_default(), true)
			});
		}
		{
			let map_pointed_objects = map_pointed_objects.clone();
			env.add_function("ls_operation", move || {
				let o: Vec<(String, minijinja::Value)> = map_pointed_objects
					.iter()
					.filter(|(k, _)| {
						let mut v = k.split("/");
						v.next().is_some_and(|v| v.eq("#"))
							&& v.next().is_some_and(|v| v.eq("paths"))
							&& v.next().is_some() && v.next().is_some_and(|v| {
							[
								"get", "put", "post", "delete", "options", "head", "patch", "trace",
							]
							.iter()
							.any(|i| i.eq(&v))
						}) && v.next().is_none()
					})
					.cloned()
					.collect();
				Ok(minijinja::Value::from_serialize(o))
			})
		}
		{
			let map_pointed_objects = map_pointed_objects.clone();
			env.add_function("ls_schema", move || {
				let o: Vec<(String, Schema)> = map_pointed_objects
					.iter()
					.filter_map(|(k, v)| Schema::deserialize(v).ok().map(|v| (k, v)))
					.filter(|(_, v)| match &v.schema_kind {
						SchemaKind::Type(_) => true,
						_ => false,
					})
					.map(|(k, v)| (k.clone(), v))
					.collect();
				Ok(minijinja::Value::from_serialize(o))
			})
		}
		{
			let map_pointed_objects = map_pointed_objects.clone();
			env.add_function("ls_all", move || {
				Ok(minijinja::Value::from_serialize(
					map_pointed_objects.clone(),
				))
			})
		}
		let v = self.templates.join("\n");
		env.add_template("main", v.as_str())?;
		let template = env.get_template("main")?;
		template.render(&self.api)
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use serde::{Deserialize, Serialize};
	use std::collections::{BTreeMap, HashMap};
	use std::fs;
	use std::fs::File;
	use std::io::Write;
	use std::path::Path;
	fn apis() -> HashMap<String, OpenAPI> {
		fs::read_dir(&Path::new(".").join("openapi"))
			.unwrap()
			.filter_map(Result::ok)
			.filter_map(|entry| {
				entry
					.path()
					.to_str()
					.unwrap_or_default()
					.contains("yaml")
					.then(|| {
						(
							entry.file_name().to_str().unwrap_or_default().to_string(),
							serde_yaml::from_reader(std::io::BufReader::new(
								File::open(entry.path()).unwrap(),
							))
							.unwrap(),
						)
					})
			})
			.collect()
	}
	fn write<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> std::io::Result<()> {
		let mut writer = std::io::BufWriter::new(File::create(path)?);
		println!("{}", content.as_ref());
		writeln!(writer, "{}", content.as_ref())
	}
	#[test]
	fn test_filter() {
		let v = apis().get("openapi.yaml").unwrap().clone();
		let r = Mandolin::new(v)
			.template("{{'#'|p|tojson}}\n{{'#/paths'|p|tojson}}\n{{'#/servers/0'|p|tojson}}")
			.render()
			.unwrap();
		println!("{}", r)
	}
	#[test]
	fn test_render_schema() {
		let r = Mandolin::new(apis().remove("openapi_nesting.yaml").unwrap())
			.template(templates::SCHEMA)
			.template(templates::DUMP)
			.render()
			.unwrap();
		write("examples/test_render_schema.out.rs", r).unwrap()
	}
	#[test]
	fn test_render_trait() {
		let r = Mandolin::new(apis().remove("openapi_nesting.yaml").unwrap())
			.template(templates::HEADER)
			.template(templates::SCHEMA)
			.template(templates::TRAIT)
			.template(templates::DUMP)
			.render()
			.unwrap();
		write("examples/test_render_trait.out.rs", r).unwrap()
	}
	#[test]
	fn test_render() {
		for (key, value) in apis() {
			let v = Mandolin::new(value)
				.template(templates::HEADER)
				.template(templates::SCHEMA)
				.template(templates::TRAIT)
				.template(templates::SERVER_AXUM)
				.render()
				.unwrap();
			write(format!("examples/test_render_{key}.out.rs"), v).unwrap();
		}
	}
	#[test]
	fn test_macro() {
		let r = Mandolin::new(apis().remove("openapi.yaml").unwrap())
			.template("{%- macro SCHEMA(schema) -%}{{schema}}{%- endmacro -%}{{ SCHEMA(openapi) }}")
			.render()
			.unwrap();
		println!("{}", r)
	}
	#[test]
	fn test_try_iter_pairs() {
		#[derive(Serialize, Deserialize)]
		struct Point {
			x: i32,
			y: i32,
		}
		let v0 = minijinja::Value::from_serialize(
			"abc"
				.chars()
				.map(|v| v.to_string())
				.collect::<Vec<String>>(),
		);
		let v1 = minijinja::Value::from_serialize(
			"abc"
				.chars()
				.map(|v| (format!("key_{v}"), format!("value_{v}")))
				.collect::<HashMap<String, String>>(),
		);
		let v2 = minijinja::Value::from_serialize(Point { x: 0, y: 1 });
		let v3 = minijinja::Value::from_serialize("abc");
		let detector = |v: minijinja::Value| {
			if let Ok(v) = BTreeMap::<minijinja::Value, minijinja::Value>::deserialize(&v) {
				return v
					.into_iter()
					.map(|(k, v)| format!("{k}={v}"))
					.collect::<String>();
			} else if let Ok(v) = Vec::<minijinja::Value>::deserialize(&v) {
				return v.into_iter().map(|v| format!("{v}!")).collect::<String>();
			}
			return ["this is just value".to_string()]
				.into_iter()
				.collect::<String>();
		};
		println!("{}", detector(v0));
		println!("{}", detector(v1));
		println!("{}", detector(v2));
		println!("{}", detector(v3));
	}
	#[test]
	fn test_cases() {
		let r = "#/paths/~1/get/responses/default/content/application~1json/schema/$ref";
		println!("{}", Mandolin::pascal_case(r));
		println!("{}", Mandolin::snake_case(r));
	}
}
