use mandolin;
use serde_yaml;
use std::fs;

fn main() {
	// read openapi.yaml
	let input_api = serde_yaml::from_str(
		fs::read_to_string("./openapi/openapi.yaml")
			.unwrap()
			.as_str(),
	)
	.unwrap();
	let mut env = mandolin::environment(input_api).unwrap();
	// add your templates
	let content = fs::read_to_string("./templates/rust_axum.template").unwrap();
	env.add_template("RUST_AXUM", content.as_str()).unwrap();

	let content = fs::read_to_string("./templates/rust_schema.template").unwrap();
	env.add_template("RUST_SCHEMA", content.as_str()).unwrap();

	let content = fs::read_to_string("./templates/rust_trait.template").unwrap();
	env.add_template("RUST_TRAIT", content.as_str()).unwrap();

	// write the rendered output
	let output = env.get_template("RUST_AXUM").unwrap().render(0).unwrap();
	fs::write("./out/server_builtin.rs", output).unwrap();
}
