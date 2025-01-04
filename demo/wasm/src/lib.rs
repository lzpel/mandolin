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
pub fn example(openapi: String) -> String {
	//エラーをまとめる方法を考える
	let mut buffer = Cursor::new(Vec::new());
	let reader = Cursor::new(include_bytes!("../../../builtin/main.tera"));
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
		let result = sums(10);
		assert_eq!(result, 55);
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
		let v=include_str!("../../test_openapi/openapi_petstore.yaml").to_string();
		println!("{}", example(v));
	}
}
