# mandolin
<a href="https://crates.io/crates/mandolin"><img alt="crates.io" src="https://img.shields.io/crates/v/mandolin.svg?style=for-the-badge&logo=rust" height="20"/></a>

Generate server code in Rust from openapi specification

Online demo with wasm: https://satoshi-misumi.github.io/mandolin/

## Using mandolin

Render axum server code using builtin template

```rust
use mandolin;
use serde_yaml;
use std::fs;
fn main() {
  let input=serde_yaml::from_str(fs::read_to_string("./openapi/openapi.yaml").unwrap().as_str()).unwrap();
  let output=mandolin::Mandolin::new(input)
          .template(mandolin::templates::HEADER)
          .template(mandolin::templates::SCHEMA)
          .template(mandolin::templates::TRAIT)
          .template(mandolin::templates::SERVER_AXUM)
          .render()
          .unwrap();
  fs::write("./examples/server_builtin.out.rs", output).unwrap();
}
```

Render axum server code using custom template

```rust
use mandolin;
use serde_yaml;
use std::fs;
fn main() {
  let input=serde_yaml::from_str(fs::read_to_string("./openapi/openapi.yaml").unwrap().as_str()).unwrap();
  let output=mandolin::Mandolin::new(input)
          .template(fs::read_to_string("./templates/header.template").unwrap())
          .template(fs::read_to_string("./templates/schema.template").unwrap())
          .template(fs::read_to_string("./templates/trait.template").unwrap())
          .template(fs::read_to_string("./templates/server_axum.template").unwrap())
          .render()
          .unwrap();
  fs::write("./examples/server_custom.out.rs", output).unwrap();
}
```

## version

- 0.1.7 hotfix
- 0.1.6 independent from regex, tera
- 0.1.5 fix ref filter
- 0.1.4 replace minijinja from tera
- 0.1.3
  - simplify mandolin::Mandolin::new `pub fn new(api: OpenAPI) -> Result<Self, serde_yaml::Error>` into `pub fn new(api: OpenAPI) -> Self`
  - remove mandolin::Mandolin::template_from_path
  - move serde_yaml(deprecated) in dependency into in dev-dependency
  - update README.md
  - add examples
  - rename mandolin::builtin into mandolin::templates
  - exclude demo from crate
- 0.1.0 publish

## my favorite mandolin music

- 月に舞う/武藤理恵 https://youtu.be/OVKkRj0di2I
- Suite Spagnola/C.Mandonico https://youtu.be/fCkcP_cuneU