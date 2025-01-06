# mandolin
<a href="https://crates.io/crates/mandolin"><img alt="crates.io" src="https://img.shields.io/crates/v/mandolin.svg?style=for-the-badge&logo=rust" height="20"/></a>

Generate openapi-based server code

Online demo with wasm: https://lzpel.github.io/mandolin/

## Using mandolin

Render server code using builtin template

```rust
use mandolin;
use std::fs;
fn main() {
    let input=serde_yaml::from_str(fs::read_to_string("./your/openapi.yaml").unwrap().as_str()).unwrap();
    let output=mandolin::Mandolin::new(input)
        .unwrap()
        .template(mandolin::builtin::MAIN)
        .render()
        .unwrap();
    fs::write("./src/server.rs", output).unwrap();
}
```

Render server code using custom template

```rust
use mandolin;
use std::fs;
fn main() {
    let input=serde_yaml::from_str(fs::read_to_string("./your/openapi.yaml").unwrap().as_str()).unwrap();
    let output=mandolin::Mandolin::new(input)
        .unwrap()
        .template(fs::read_to_string("./main.tera").unwrap().as_str())
        .render()
        .unwrap();
    fs::write("./src/server.rs", output).unwrap();
}
```

## my favorite mandolin music

- 月に舞う/武藤理恵 https://youtu.be/OVKkRj0di2I?si=bZn0yI6C9QrVjPKV
- Suite Spagnola https://youtu.be/fCkcP_cuneU?si=21Da2RJgVRyPD8Et