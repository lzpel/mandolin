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
	let env = mandolin::environment(input_api).unwrap();
	let template = env.get_template("RUST_SERVER_AXUM").unwrap();
	let output = template.render(0).unwrap();
	// write the rendered output
	fs::write("./output/server_builtin.rs", output).unwrap();
}
