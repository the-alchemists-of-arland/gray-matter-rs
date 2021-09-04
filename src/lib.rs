#![allow(clippy::from_over_into)]

//! **gray_matter** is a tool for easily extracting front matter out of a string. It is a fast Rust
//! implementation of the original [gray-matter](https://github.com/jonschlinkert/gray-matter) by
//! [Jon Schlinkert](https://github.com/jonschlinkert).
//!
//! ## What can gray_matter do?
//!
//! It can take some string or file like this:
//!
//! ```markdown
//! ---
//! title: This is the title
//! tags:
//!     - awesome-tag
//!     - more-awesome-tag
//! ---
//!
//! # Header
//!
//! This is my really cool document!
//! ```
//!
//! and strip the YAML contained within the delimiters (`---`), to create a struct like this[^1]:
//!
//! ```ignore
//! MyStruct {
//!     title: "This is the title",
//!     tags: ["awesome-tag", "more-awesome-tag"]
//! }
//! ```
//!
//! [^1]: The struct is purely for demonstration.
//!
//! ## Why would I use this?
//!
//! You want to have configurability inside plain text documents, like for example a Markdown document.
//! [Pandoc](https://pandoc.org) and [Hugo](https://gohugo.io) are examples of tools that use this
//! kind of configuration, but the sky really is the limit.
//!
//! ## What formats are supported?
//!
//! **gray_matter** has built in support for [YAML](crate::engine::YAML),
//! [TOML](crate::engine::TOML) and [JSON](crate::engine::JSON), but the
//! [`Engine`](crate::engine::Engine) trait allows for virtually any format you wish to be
//! supported.
//!
//! # Examples
//!
//! ## Basic parsing
//!
//! ```rust
//! use gray_matter::Matter;
//! use gray_matter::engine::YAML;
//! use serde::Deserialize;
//!
//! const INPUT: &str = r#"---
//! title: gray-matter-rs
//! tags:
//!   - gray-matter
//!   - rust
//! ---
//! Some excerpt
//! ---
//! Other stuff
//! "#;
//!
//! fn main() {
//!     // Select one parser engine, such as YAML, and parse it
//!     // into gray_matter's custom data type: `Pod`
//!     let matter = Matter::<YAML>::new();
//!     let result = matter.parse(INPUT);
//!
//!     // You can now inspect the data from gray_matter.
//!     assert_eq!(result.content, "Some excerpt\n---\nOther stuff");
//!     assert_eq!(result.excerpt, Some("Some excerpt".to_owned()));
//!     assert_eq!(result.data.as_ref().unwrap()["title"].as_string(), Ok("gray-matter-rs".to_string()));
//!     assert_eq!(result.data.as_ref().unwrap()["tags"][0].as_string(), Ok("gray-matter".to_string()));
//!     assert_eq!(result.data.as_ref().unwrap()["tags"][1].as_string(), Ok("rust".to_string()));
//!
//!     // The `Pod` data type can be a bit unwieldy, so
//!     // you can also deserialize it into a custom struct
//!     #[derive(Deserialize, Debug)]
//!     struct FrontMatter {
//!         title: String,
//!         tags: Vec<String>
//!     }
//!
//!     // Deserialize `result` manually:
//!     let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
//!     println!("{:?}", front_matter);
//!     // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }
//!
//!     // ...or skip a step, by using `parse_with_struct`.
//!     let result_with_struct = matter.parse_with_struct::<FrontMatter>(INPUT).unwrap();
//!     println!("{:?}", result_with_struct.data)
//!     // FrontMatter { title: "gray-matter-rs", tags: ["gray-matter", "rust"] }
//! }
//! ```

// Test README
#[cfg(doctest)]
macro_rules! doc_check {
    ($x:expr) => {
        #[doc = $x]
        extern "C" {}
    };
}

#[cfg(doctest)]
doc_check!(include_str!("../README.md"));

/// A module containing the [`Engine`](crate::engine::Engine) trait, along with gray_matter's default engines.
pub mod engine;

#[doc(hidden)]
pub mod entity;
#[doc(inline)]
pub use entity::{ParsedEntity, ParsedEntityStruct};

#[doc(hidden)]
pub mod matter;
#[doc(inline)]
pub use matter::Matter;

#[doc(hidden)]
pub mod value;
#[doc(inline)]
pub use value::{error::Error, pod::Pod};

#[cfg(test)]
mod tests;
