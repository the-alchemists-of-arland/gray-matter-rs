![](https://github.com/the-alchemists-of-arland/gray-matter-rs/actions/workflows/ci.yml/badge.svg?branch=main)
[![Latest Version](https://img.shields.io/crates/v/gray_matter.svg)](https://crates.io/crates/gray_matter)
[![Documentation](https://docs.rs/gray_matter/badge.svg)](https://docs.rs/gray_matter)

gray_matter is a tool for easily extracting front matter out of a string. It is a fast Rust implementation of [gray-matter](https://github.com/jonschlinkert/gray-matter). It supports the following front matter formats:

- TOML
- YAML
- JSON

It also has an `Engine` trait interface for implementing your own parsers that work with gray_matter.

## Usage

### Add `gray_matter` as a dependency

Append this crate to the `Cargo.toml`:

```toml
[dependencies]
# other dependencies...
gray_matter = "0.2"
```

### Basic parsing

```rust
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;

const INPUT: &str = r#"---
title: gray-matter-rs
tags:
  - gray-matter
  - rust
---
Some excerpt
---
Other stuff
"#;

fn main() {
    // Select one parser engine, such as YAML, and parse it
    // into gray_matter's custom data type: `Pod`
    let matter = Matter::<YAML>::new();
    let result = matter.parse(INPUT);

    // You can now inspect the data from gray_matter.
    assert_eq!(result.content, "Some excerpt\n---\nOther stuff");
    assert_eq!(result.excerpt, Some("Some excerpt".to_owned()));
    assert_eq!(result.data.as_ref().unwrap()["title"].as_string(), Ok("gray-matter-rs".to_string()));
    assert_eq!(result.data.as_ref().unwrap()["tags"][0].as_string(), Ok("gray-matter".to_string()));
    assert_eq!(result.data.as_ref().unwrap()["tags"][1].as_string(), Ok("rust".to_string()));

    // The `Pod` data type can be a bit unwieldy, so
    // you can also deserialize it into a custom struct
    #[derive(Deserialize, Debug)]
    struct FrontMatter {
        title: String,
        tags: Vec<String>
    }

    // Deserialize `result` manually:
    let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
    println!("{:?}", front_matter);
    // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }

    // ...or skip a step, by using `parse_with_struct`.
    let result_with_struct = matter.parse_with_struct::<FrontMatter>(INPUT).unwrap();
    println!("{:?}", result_with_struct.data)
    // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }
}
```

### Custom Delimiters

The default delimiter is `---`, both for front matter and excerpts. You can change this by modifiying the `Matter` struct.

```rust
use gray_matter::{Matter, ParsedEntityStruct};
use gray_matter::engine::YAML;
use serde::Deserialize;

fn main() {
    let mut matter: Matter<YAML> = Matter::new();
    matter.delimiter = "~~~".to_owned();
    matter.excerpt_delimiter = Some("<!-- endexcerpt -->".to_owned());

    #[derive(Deserialize, Debug)]
    struct FrontMatter {
        abc: String,
    }

    let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(
        "~~~\nabc: xyz\n~~~\nfoo\nbar\nbaz\n<!-- endexcerpt -->\ncontent",
    ).unwrap();
}
```

#### Custom close delimiter

The open and close delimiter are the same by default (`---`). You can change this by modifiying `close_delimiter` property of `Matter` struct

```rust
use gray_matter::{Matter, ParsedEntityStruct};
use gray_matter::engine::YAML;
use serde::Deserialize;

fn main() {
    let mut matter: Matter<YAML> = Matter::new();
    matter.delimiter = "<!--".to_owned();
    matter.close_delimiter = Some("-->".to_owned());
    matter.excerpt_delimiter = Some("<!-- endexcerpt -->".to_owned());

    #[derive(Deserialize, Debug)]
    struct FrontMatter {
        abc: String,
    }

    let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(
        "<!--\nabc: xyz\n-->\nfoo\nbar\nbaz\n<!-- endexcerpt -->\ncontent",
    ).unwrap();
}
```

## Contributors
<a href="https://github.com/the-alchemists-of-arland/gray-matter-rs/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=the-alchemists-of-arland/gray-matter-rs" />
</a>

## Contribution

If you need more parser engines, feel free to create a **PR** to help me complete this crate.

