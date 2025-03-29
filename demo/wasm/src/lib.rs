extern crate console_error_panic_hook;
use mandolin;
use wasm_bindgen::prelude::*;
use mandolin::environment;

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

#[wasm_bindgen]
pub fn example(openapi_yaml: &str) -> String {
	console_error_panic_hook::set_once();
	//エラーをまとめる方法を考える
	let v = match serde_yaml::from_str(openapi_yaml) {
		Ok(v) => v,
		Err(e) => {
			return format!("# Error\n\nThis text cannot be interpreted as an OpenAPI in YAML format.\n\n## detail\n\n{}\n\n## content\n\n{}", e.to_string(), openapi_yaml);
		}
	};
	let t=mandolin::templates::templates();
	let e = environment(&t, v).unwrap();
	let result = e
		.get_template("RUST_SERVER_AXUM")
		.unwrap()
		.render(false);
	result.unwrap_or_else(|e| {
		format!("# Error\n\nCannot render rust code from this OpenApi specification.\n\n## detail\n\n{}", e.to_string())
	})
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
		let v = serde_yaml::from_str(include_str!("../../../openapi/openapi.yaml")).unwrap();
		let template = mandolin::templates::templates();
		let env=mandolin::environment(&template, v).unwrap();
		let result = env
			.get_template("RUST_SERVER_AXUM")
			.unwrap()
			.render(0)
			.unwrap();
		println!("{}", result);
	}
	#[test]
	fn test_example_petstore() {
		println!("{}", example(include_str!("../../../openapi/openapi.yaml")));
	}
	#[test]
	fn test_example_min() {
		println!("{}", example(include_str!("../../../openapi/openapi.yaml")));
	}
}
