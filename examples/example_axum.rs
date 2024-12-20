use mandolin;
// cargo run --example example_axum
fn main(){
	mandolin::build("./test_openapi/openapi_petstore.yaml", "./output/openapi_petstore.rs").expect("TODO: panic message");
}