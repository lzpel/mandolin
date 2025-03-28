pub mod templates;
pub mod filter;
use std::collections::HashMap;
use serde::Deserialize;

pub fn jp_list(value: &minijinja::Value, prefix: &str)->Vec<(String, minijinja::Value)>{
	let mut output=Default::default();
	fn recursive(path: String, value: &minijinja::Value, output: &mut Vec<(String, minijinja::Value)>) {
		output.push((path.clone(), value.clone())); //注目箇所を追加
		if let Some(v) = value.as_object() {
			//子を検索
			if let Some(v) = v.try_iter_pairs() {
				v.for_each(|(k, v)| {
					recursive(format!("{path}/{}", filter::jp_encode(k.as_str().unwrap_or_default())), &v, output)
				});
			} else if let Some(v) = v.try_iter() {
				v.enumerate().for_each(|(k, v)| {
					recursive(format!("{path}/{k}"), &v, output)
				});
			}
		}
	}
	recursive(prefix.to_string(), value, &mut output);
	output
}
pub fn environment<S: serde::Serialize>(
	templates: &HashMap<String, String>,
	value: S,
) -> Result<minijinja::Environment, minijinja::Error> {
	let value = minijinja::Value::from_serialize(&value);
	let value_jp = jp_list(&value, "#");
	let mut env = minijinja::Environment::new();
	for (k, v) in templates {
		env.add_template(k.as_str(), v.as_str())?;
	}
	env.add_filter("decode", filter::jp_decode);
	env.add_filter("encode", filter::jp_encode);
	env.add_filter("to_pascal_case", filter::to_pascal_case);
	env.add_filter("to_snake_case", filter::to_snake_case);
	{
		let value_jp = value_jp.clone();
		let methods=["get", "put", "post", "delete", "options", "head", "patch", "trace"];
		env.add_function("ls_operation", move || {
			let o: Vec<(String, minijinja::Value)> = value_jp
				.iter()
				.filter(|(k, _)| {
					k.strip_prefix("#/paths/").is_some_and(|v|{
						let mut w=v.split("/");
						w.next().is_some()
							&& w.next().is_some_and(|v| methods.contains(&v))
							&& w.next().is_none()
					})
				})
				.cloned()
				.collect();
			Ok(minijinja::Value::from_serialize(o))
		})
	}
	{
		let value_jp = value_jp.clone();
		env.add_function("ls_schema", move || {
			let o: Vec<(String, minijinja::Value)> = value_jp
				.iter()
				.filter(|(_k, v)| {
					openapiv3::Schema::deserialize(v)
						.is_ok_and(|v| match v.schema_kind{
							openapiv3::SchemaKind::Type(_) => true,
							_ => false,
						})
				})
				.cloned()
				.collect();
			Ok(minijinja::Value::from_serialize(o))
		})
	}
	Ok(env)
}
#[cfg(test)]
mod tests {
	use super::*;
	use openapiv3::OpenAPI;
	use std::collections::HashMap;
	use std::fs;
	use std::fs::File;
	use std::io::Write;
	use std::path::Path;

	fn api_map() -> HashMap<String, OpenAPI> {
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
	fn test() {
		let bindings = templates::templates();
		for (k,v) in api_map(){
			let e = environment(&bindings, v).unwrap();
			println!("{k} {:?}", e.templates().map(|v| v.0).collect::<Vec<_>>());
			let result = e
				.get_template("RUST_SERVER_AXUM")
				.unwrap()
				.render(0)
				.unwrap();
			println!("{}", result)
		}
	}
}
