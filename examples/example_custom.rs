use mandolin;
use serde_yaml;
use std::fs;

fn main() {
	let input_api = serde_yaml::from_str(
		fs::read_to_string("./openapi/openapi.yaml")
			.unwrap()
			.as_str(),
	)
	.unwrap();
	let mut env = mandolin::environment(input_api).unwrap();
	// add your templates
	let content = fs::read_to_string("./templates/rust_server_axum.template").unwrap();
	env.add_template("RUST_SERVER_AXUM", content.as_str())
		.unwrap();

	let content = fs::read_to_string("./templates/schema.template").unwrap();
	env.add_template("SCHEMA", content.as_str()).unwrap();

	let content = fs::read_to_string("./templates/server_axum.template").unwrap();
	env.add_template("SERVER_AXUM", content.as_str()).unwrap();

	let content = fs::read_to_string("./templates/trait.template").unwrap();
	env.add_template("TRAIT", content.as_str()).unwrap();
	// render
	let template = env.get_template("RUST_SERVER_AXUM").unwrap();
	let output = template.render(0).unwrap();
	fs::write("./output/server_custom.out.rs", output).unwrap();
}
