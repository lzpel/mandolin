# mandolin
<a href="https://crates.io/crates/mandolin"><img alt="crates.io" src="https://img.shields.io/crates/v/mandolin.svg?style=for-the-badge&logo=rust" height="20"/></a>

Generate openapi-based server code

Online demo with wasm: https://lzpel.github.io/mandolin/

## Using mandolin

Render server code using builtin template

```rust
use mandolin;
use serde_yaml;
use std::fs;
fn main() {
  let input=serde_yaml::from_str(fs::read_to_string("./openapi/openapi.yaml").unwrap().as_str()).unwrap();
  let output=mandolin::Mandolin::new(input)
          .template(mandolin::templates::MAIN)
          .render()
          .unwrap();
  fs::write("./src/server.rs", output).unwrap();
}
```

Render server code using custom template

```rust
use mandolin;
use serde_yaml;
use std::fs;
fn main() {
    let input=serde_yaml::from_str(fs::read_to_string("./openapi/openapi.yaml").unwrap().as_str()).unwrap();
    let output=mandolin::Mandolin::new(input)
        .template(fs::read_to_string("./custom/template.tera").unwrap())
        .render()
        .unwrap();
    fs::write("./src/server.rs", output).unwrap();
}
```

## version

- 0.1.4
  - replace minijinja from tera
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