use mandolin;
use serde_yaml;
use std::fs;
fn main() {
    let input=serde_yaml::from_str(fs::read_to_string("./openapi/openapi.yaml").unwrap().as_str()).unwrap();
    let output=mandolin::Mandolin::new(input)
        .template(fs::read_to_string("./templates/main.tera").unwrap())
        .render()
        .unwrap();
    fs::write("./src/server.rs", output).unwrap();
}