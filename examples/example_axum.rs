use std::fs;
use mandolin;
// cargo run --example example_axum
fn main(){
	//エラーをまとめる方法を考える
	let v=serde_yaml::from_str(include_str!("../test_openapi/openapi_petstore.yaml")).unwrap();
	let result=mandolin::Mandolin::new(v)
		.unwrap()
		.template(fs::File::open("./test_openapi/openapi_petstore.yaml").unwrap())
		.unwrap()
		.template(fs::File::open("./test_openapi/openapi_petstore.yaml").unwrap())
		.unwrap();
	mandolin::build("./test_openapi/openapi_petstore.yaml", "./output/openapi_petstore.rs").expect("TODO: panic message");
}