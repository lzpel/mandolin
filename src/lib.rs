mod filter;
mod function;
pub mod templates;

use std::collections::HashMap;
use openapiv3::OpenAPI;

pub type JpUnit = (String, minijinja::Value);
pub type JpList = Vec<JpUnit>;

pub fn environment(
	templates: &HashMap<String, String>,
	value: OpenAPI,
) -> Result<minijinja::Environment, minijinja::Error> {
	let value = minijinja::Value::from_serialize(&value);
	let value_jp = function::jp_list(&value, "#");
	let mut env = minijinja::Environment::new();
	for (k, v) in templates {
		env.add_template(k.as_str(), v.as_str())?;
	}
	{
		let ls = value_jp.clone();
		env.add_filter("r", move |value: minijinja::Value| filter::r(&ls, value));
		let ls = value_jp.clone();
		env.add_filter("p", move |value: &str| filter::point(&ls, value));
	}
	env.add_filter("decode", filter::decode);
	env.add_filter("encode", filter::encode);
	env.add_filter("split", filter::split);
	env.add_filter("to_pascal_case", filter::to_pascal_case);
	env.add_filter("to_snake_case", filter::to_snake_case);
	{
		let ls = value_jp.clone();
		env.add_function("ls", move || function::ls(&ls, function::LsMode::ALL));
		let ls = value_jp.clone();
		env.add_function("ls_operation", move || {
			function::ls(&ls, function::LsMode::OPERATION)
		});
		let ls = value_jp.clone();
		env.add_function("ls_schema", move || {
			function::ls(&ls, function::LsMode::SCHEMA)
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
	fn test() {
		let bindings = templates::templates();
		for (k, v) in api_map() {
			let e = environment(&bindings, v).unwrap();
			println!("{k} {:?}", e.templates().map(|v| v.0).collect::<Vec<_>>());
			let result = e
				.get_template("RUST_SERVER_AXUM")
				.unwrap()
				.render(0)
				.unwrap();
			println!("{}", result);
			write(format!("output/{k}.rs"), result.as_str()).unwrap();
		}
	}
}
