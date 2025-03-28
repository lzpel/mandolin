pub mod templates;

use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
pub fn filter_pointer(value: &minijinja::Value, path: &str) -> Option<minijinja::Value> {
	let path_modified=path.strip_prefix("#").unwrap_or(path);
	let value = serde_json::Value::deserialize(value).unwrap();
	let pointed = value.pointer(path_modified);
	pointed.map(|v| minijinja::Value::deserialize(v).unwrap())
}
pub fn environment<S: serde::Serialize>(
	templates: &HashMap<String, String>,
	value: S,
) -> Result<minijinja::Environment, minijinja::Error> {
	let minijinja_value = minijinja::Value::from_serialize(&value);
	let mut env = minijinja::Environment::new();
	for (k, v) in templates {
		env.add_template(k.as_str(), v.as_str())?;
	}
	{
		let value = minijinja_value.clone();
		env.add_filter("p", move |path: &minijinja::Value| {
			filter_pointer(&value, path.as_str().unwrap_or_default())
		});
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
	fn test_filter() {
		let v = api_map().get("openapi.yaml").unwrap().clone();
		let bindings = templates::templates();
		let e = environment(&bindings, v).unwrap();
		println!("{:?}", e.templates().map(|v| v.0).collect::<Vec<_>>());
		let result = e
			.get_template("RUST_SERVER_AXUM")
			.unwrap()
			.render(0)
			.unwrap();
		println!("{}", result)
	}
}
