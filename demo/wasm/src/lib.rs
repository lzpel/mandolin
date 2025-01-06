extern crate console_error_panic_hook;
use mandolin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
	console_error_panic_hook::set_once();
    let mut a: i32 = 0;
    for i in 1..value+1 {
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
	let v=serde_yaml::from_str(openapi_yaml).unwrap();
	//let v=serde_yaml::from_str(include_str!("../../../openapi/openapi_petstore.yaml")).unwrap();
	mandolin::Mandolin::new(v)
		.unwrap()
		.template(mandolin::builtin::MAIN)
		.render()
		.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = sums(10);
		assert_eq!(result, 55);
		println!("{}", "it_works");
		println!("{}", mandolin::builtin::MAIN);
	}
	#[test]
	fn generate() {
		let v=serde_yaml::from_str(include_str!("../../../openapi/openapi_petstore.yaml")).unwrap();
		let result=mandolin::Mandolin::new(v)
			.unwrap()
			.template_from_path("../../builtin/main.tera")
			.unwrap()
			.render()
			.unwrap();
		println!("{}", result);
	}
	#[test]
	fn test_example_petstore() {
		println!("{}", example(include_str!("../../../openapi/openapi_petstore.yaml")));
	}
	#[test]
	fn test_example_min() {
		println!("{}", example(include_str!("../../../openapi/openapi_min.yaml")));
	}
}
