extern crate console_error_panic_hook;
use mandolin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn example(openapi_string: &str, template: &str) -> String {
	console_error_panic_hook::set_once();
	//エラーをまとめる方法を考える
	let v = match serde_json::from_str(openapi_string) {
		Ok(v) => v,
		Err(_) => match serde_yaml::from_str(openapi_string) {
			Ok(v) => v,
			Err(e) => {
				return format!("# Error\n\nThis text cannot be interpreted as an OpenAPI in YAML or JSON format.\n\n## detail\n\n{}\n\n## content\n\n{}", e.to_string(), openapi_string);
			}
		},
	};
	let e = mandolin::environment(v).unwrap();
	let result = e.get_template(template).unwrap().render(false);
	result.unwrap_or_else(|e| {
		format!("# Error\n\nOpenAPI is OK, But cannot render with '{template}' template from this OpenApi specification.\n\n## detail\n\n{}", e.to_string())
	})
}

#[wasm_bindgen]
pub fn templates() -> Vec<String> {
	["RUST_AXUM", "TYPESCRIPT_HONO"]
		.map(|v| v.to_string())
		.to_vec()
}

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
	console_error_panic_hook::set_once();
	let mut a: i32 = 0;
	for i in 1..value + 1 {
		a += i;
	}
	a
}

#[wasm_bindgen]
pub fn upper(value: String) -> String {
	console_error_panic_hook::set_once();
	value.to_uppercase()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let result = sums(10);
		assert_eq!(result, 55);
		println!("{}", "it_works");
	}
	#[test]
	fn generate() {
		let v = serde_yaml::from_str(include_str!("../../out/openapi.yaml")).unwrap();
		let env = mandolin::environment(v).unwrap();
		let result = env
			.get_template(templates()[0].as_str())
			.unwrap()
			.render(false)
			.unwrap();
		println!("{}", result);
	}
	#[test]
	fn test_example_petstore() {
		println!(
			"{}",
			example(
				include_str!("../../out/openapi_petstore.yaml"),
				templates()[0].as_str()
			)
		);
	}
	#[test]
	fn test_example_min() {
		println!(
			"{}",
			example(
				include_str!("../../out/openapi.yaml"),
				templates()[0].as_str()
			)
		);
	}
}
