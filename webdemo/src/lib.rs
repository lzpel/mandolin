use std::io::Cursor;
use mandolin;
use wasm_bindgen::prelude::*;
pub fn add(left: u64, right: u64) -> u64 {
	left + right
}

#[wasm_bindgen]
pub fn example(openapi: &str) -> String {
	//エラーをまとめる方法を考える
	let mut buffer = Cursor::new(Vec::new());
	let reader = Cursor::new(include_bytes!("../../builtin/main.tera"));
	mandolin::Mandolin::new(Cursor::new(openapi.as_bytes()))
		.unwrap()
		.template(reader)
		.unwrap()
		.write(&mut buffer)
		.unwrap()
		.unwrap();
	String::from_utf8(buffer.into_inner()).expect("Found invalid UTF-8")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
	#[test]
	fn generate() {
		let mut buffer = Cursor::new(Vec::new());
		mandolin::Mandolin::new_from_path("../test_openapi/openapi_petstore.yaml")
			.unwrap()
			.unwrap()
			.template_from_path("../builtin/main.tera")
			.unwrap()
			.write(&mut buffer)
			.unwrap()
			.unwrap();
		let result = String::from_utf8(buffer.into_inner()).expect("Found invalid UTF-8");
		println!("{}", result);
	}
	#[test]
	fn generate2() {
		println!("{}", example(include_str!("../../test_openapi/openapi_petstore.yaml")));
	}
}
