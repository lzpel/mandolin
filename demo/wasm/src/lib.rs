extern crate console_error_panic_hook;
use mandolin;
use mandolin::{templates, Mandolin};
use wasm_bindgen::prelude::*;

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
			return format!("# Error\n\nThis text cannot be interpreted as an OpenAPI in YAML format.\n\n## detail\n\n{}", e.to_string());
		}
	};
	//let v=serde_yaml::from_str(include_str!("../../../openapi/openapi_petstore.yaml")).unwrap();
	let v = Mandolin::new(v)
		.template(templates::HEADER)
		.template(templates::SCHEMA)
		.template(templates::TRAIT)
		.template(templates::SERVER_AXUM)
		.render();
	match v {
		Ok(v) => v,
		Err(e) => {
			return format!("# Error\n\nCannot render rust code from this OpenApi specification.\n\n## detail\n\n{}", e.to_string());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn it_works() {
		let result = sums(10);
		assert_eq!(result, 55);
		println!("{}", "it_works");
	}
	#[test]
	fn generate() {
		let v = serde_yaml::from_str(include_str!("../../../openapi/openapi.yaml")).unwrap();
		let result = mandolin::Mandolin::new(v)
			.template(fs::read_to_string("../../templates/main.tera").unwrap())
			.render()
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
