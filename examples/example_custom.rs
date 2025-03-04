use mandolin;
use serde_yaml;
use std::fs;
fn main() {
	let input = serde_yaml::from_str(
		fs::read_to_string("./openapi/openapi.yaml")
			.unwrap()
			.as_str(),
	)
	.unwrap();
	let output = mandolin::Mandolin::new(input)
		.template(fs::read_to_string("./templates/header.template").unwrap())
		.template(fs::read_to_string("./templates/schema.template").unwrap())
		.template(fs::read_to_string("./templates/trait.template").unwrap())
		.template(fs::read_to_string("./templates/server_axum.template").unwrap())
		.render()
		.unwrap();
	fs::write("./examples/server_custom.out.rs", output).unwrap();
}
