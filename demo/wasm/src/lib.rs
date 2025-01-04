use std::io::Cursor;
use mandolin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
    let mut a: i32 = 0;
    for i in 1..value+1 {
        a += i;
    }
    a
}

#[wasm_bindgen]
pub fn upper(value: String) -> String {
	value.to_uppercase()
}
#[wasm_bindgen]
pub fn sandbox1(value: &str) -> String {
	mandolin::sandbox1(value)
}

#[wasm_bindgen]
pub fn example(openapi_yaml: &str) -> String {
	//エラーをまとめる方法を考える
	let v=serde_yaml::from_str(openapi_yaml).unwrap();
	mandolin::Mandolin::new(v)
		.unwrap()
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
	}
	#[test]
	fn generate() {
		let v=serde_yaml::from_str(include_str!("../../../test_openapi/openapi_petstore.yaml")).unwrap();
		let result=mandolin::Mandolin::new(v)
			.unwrap()
			.template_from_path("../../builtin/main.tera")
			.unwrap()
			.render()
			.unwrap();
		println!("{}", result);
	}
	#[test]
	fn generate2() {
		let v=include_str!("../../../test_openapi/openapi_petstore.yaml");
		println!("{}", example(v));
	}
}
