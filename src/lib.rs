mod filter;
mod function;
pub mod templates;

use std::sync::Mutex;

use openapiv3::OpenAPI;

pub type JpUnit = (String, minijinja::Value);
pub type JpList = Vec<JpUnit>;

pub fn environment(
	mut value: OpenAPI,
) -> Result<minijinja::Environment<'static>, minijinja::Error> {
	// Add default api if no api defined
	if value.servers.is_empty() {
		value.servers.push(openapiv3::Server {
			url: "/api".to_string(),
			description: Some("Default server added by mandolin".to_string()),
			..Default::default()
		});
	}
	// Add operators
	let value = minijinja::Value::from_serialize(&value);
	let value_jp = function::jp_list(&value, "#");
	let mut env = minijinja::Environment::new();
	for [k, v] in templates::TEMPLATES {
		env.add_template(k, v)?
	}
	{
		let ls = value_jp.clone();
		env.add_filter("include_ref", move |value: minijinja::Value| {
			filter::include_ref(&ls, value)
		});
		let ls = value_jp.clone();
		env.add_filter("include_pointer", move |value: &str| {
			filter::include_pointer(&ls, value)
		});
	}
	env.add_filter("decode", filter::decode);
	env.add_filter("encode", filter::encode);
	env.add_filter("split", filter::split);
	env.add_filter("re_replace", filter::re_replace);
	env.add_filter("to_pascal_case", filter::to_pascal_case);
	env.add_filter("to_snake_case", filter::to_snake_case);
	env.add_filter("to_camel_case", filter::to_camel_case);
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
	let queue = std::sync::Arc::new(Mutex::new(function::NestedSchema::default()));
	{
		let q = std::sync::Arc::clone(&queue);
		env.add_function("schema_drain", move || {
			function::schema_drain(&mut q.lock().unwrap())
		});
		let q = std::sync::Arc::clone(&queue);
		env.add_function(
			"schema_push",
			move |pointer: &str, content: Option<&str>| {
				function::schema_push(&mut q.lock().unwrap(), pointer, content)
			},
		);
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
				let path = entry.path();
				let extension = path.extension()?.to_str()?;
				let name = entry.file_name().to_str()?.to_string();

				match extension {
					"yaml" | "yml" => {
						let file = File::open(&path).ok()?;
						let reader = std::io::BufReader::new(file);
						let openapi = serde_yaml::from_reader(reader).ok()?;
						Some((name, openapi))
					}
					"json" => {
						let file = File::open(&path).ok()?;
						let reader = std::io::BufReader::new(file);
						let openapi = serde_json::from_reader(reader).ok()?;
						Some((name, openapi))
					}
					_ => None,
				}
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
	fn render_target(template: &str, extention: &str) {
		for (k, input_api) in api_map() {
			println!("render start: {k}");
			let env = environment(input_api).unwrap();
			let template = env.get_template(template).unwrap();
			let output = template.render(0).unwrap();
			write(format!("out/{k}.{extention}"), output.as_str()).unwrap();
			println!("render complete: {k}");
		}
	}
	#[test]
	fn render() {
		render_target("TYPESCRIPT_HONO", "ts");
		render_target("RUST_AXUM", "rs");
		render_target("GO_SERVER", "go");
	}
}
