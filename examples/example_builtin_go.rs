use mandolin;
use serde_yaml;
use std::fs;
// go template is under developping
fn main() {
	// read openapi.yaml
	let input_string = fs::read_to_string("./openapi/openapi_petstore.yaml").unwrap();
	let input_api = serde_yaml::from_str(&input_string.as_str()).unwrap();
	// make environment
	let env = mandolin::environment(input_api).unwrap();
	// write the rendered output
	let output = env
		.get_template("GO_SERVER")
		.unwrap()
		.render(0)
		.unwrap();
	fs::write("./out/server_builtin_go.go", output).unwrap();
}
