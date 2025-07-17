use mandolin;
use serde_yaml;
use std::fs;
fn main() {
	// read openapi.yaml
	let input_api = serde_yaml::from_str(
		fs::read_to_string("./openapi/openapi_petstore.yaml")
			.unwrap()
			.as_str(),
	)
	.unwrap();
	let env = mandolin::environment(input_api).unwrap();
	// write the rendered output
	let output = env.get_template("RUST_AXUM").unwrap().render(0).unwrap();
	fs::write("./out/server_builtin.ts", output).unwrap();
}
