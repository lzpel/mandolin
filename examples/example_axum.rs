use std::fs;
use mandolin;
// cargo run --example example_axum
fn main(){
	//エラーをまとめる方法を考える
	mandolin::Mandolin::new_from_path("./test_openapi/openapi_petstore.yaml")
		.unwrap()
		.unwrap()
		.template(fs::File::open("./test_openapi/openapi_petstore.yaml").unwrap())
		.unwrap()
		.template(fs::File::open("./test_openapi/openapi_petstore.yaml").unwrap())
		.unwrap();
	mandolin::build("./test_openapi/openapi_petstore.yaml", "./output/openapi_petstore.rs").expect("TODO: panic message");
}