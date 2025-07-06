mod filter;
mod function;
pub mod templates;

use openapiv3::OpenAPI;

pub type JpUnit = (String, minijinja::Value);
pub type JpList = Vec<JpUnit>;

pub fn environment(value: OpenAPI) -> Result<minijinja::Environment<'static>, minijinja::Error> {
	let value = minijinja::Value::from_serialize(&value);
	let value_jp = function::jp_list(&value, "#");
	let mut env = minijinja::Environment::new();
	for [k, v] in templates::TEMPLATES {
		env.add_template(k, v)?
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
		env.add_function("ls", move |value: &str| {
			function::ls(&ls, function::LsMode::LS((value, false)))
		});
		let ls = value_jp.clone();
		env.add_function("ls_recursive", move |value: &str| {
			function::ls(&ls, function::LsMode::LS((value, true)))
		});
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
		let path = path.as_ref();
		// 親ディレクトリがある場合は作成する
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)?;
		}
		let mut writer = std::io::BufWriter::new(File::create(path)?);
		writeln!(writer, "{}", content.as_ref())
	}
	#[test]
	fn render() {
		for (k, input_api) in api_map() {
			println!("render start: {k}");
			let env = environment(input_api).unwrap();
			let template = env.get_template("RUST_SERVER_AXUM").unwrap();
			let output = template.render(0).unwrap();
			write(format!("out/{k}.rs"), output.as_str()).unwrap();
			println!("render complete: {k}");
		}
	}
}
