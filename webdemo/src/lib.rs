use std::fs;
use mandolin;
use wasm_bindgen::prelude::*;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn example(){
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
