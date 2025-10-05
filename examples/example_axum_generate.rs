use mandolin;
use serde_yaml;
fn main() {
	// read openapi.yaml
	let input_openapi_path = std::env::args()
		.nth(1)
		.unwrap_or_else(|| "./openapi/openapi_plant.yaml".to_string());
	let input_string = std::fs::read_to_string(input_openapi_path).unwrap();
	let input_api = serde_yaml::from_str(&input_string.as_str()).unwrap();
	// make environment
	let env = mandolin::environment(input_api).unwrap();
	// write the rendered output
	let output = env.get_template("RUST_AXUM").unwrap().render(0).unwrap();
	std::fs::write("examples/example_axum_generated.rs", output).unwrap();
}
