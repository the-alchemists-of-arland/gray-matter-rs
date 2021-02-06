# gray-matter-rs
![](https://github.com/yuchanns/gray-matter-rs/workflows/main/badge.svg?branch=main)

A rust implementation of [gray-matter](https://github.com/jonschlinkert/gray-matter).

**Support Parsers**
* toml
* yaml
* json
* ... (more custom parsers)

## Usage
### Add Dependency
Append this crate to the `Cargo.toml`:
```toml
[dependencies]
# other dependencies...
gray_matter = "0.1"
```
### Parse
```rust
use gray_matter::matter::Matter;
use gray_matter::engine::yaml::YAML;
use serde::Deserialize;

fn main() {
    // select one parser engine, such as YAML
    let matter: Matter<YAML> = Matter::new();
    let input = r#"---
title: gray-matter-rs
tags:
  - gray-matter
  - rust
---
Some excerpt
---
Other stuff"#;
    let result = matter.matter(input);
    println!("content: {:?}", result.content);
    println!("excerpt: {:?}", result.excerpt);
    println!("title: {:?}", result.data["title"].as_string().unwrap());
    println!("tags[0]: {:?}", result.data["tags"][0].as_string().unwrap());
    println!("tags[1]: {:?}", result.data["tags"][1].as_string().unwrap());
    // content: "Some excerpt\n---\nOther stuff"
    // excerpt: "Some excerpt"
    // title: "gray-matter-rs"
    // tags[0]: "gray-matter"
    // tags[1]: "rust"
    #[derive(Deserialize, Debug)]
    struct FrontMatter {
        title: String,
        tags: Vec<String>
    }
    let front_matter: FrontMatter = result.data.deserialize().unwrap();
    println!("{:?}", front_matter);
    // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }
    let result_with_struct: ParsedEntityStruct<FrontMatter> = matter.matter_struct(input);
    println!("{:?}", result_with_struct.data)
    // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }
}
```
## Contribution
If you need more parser engines, just free to create a **PR** to help me complete this crate.
