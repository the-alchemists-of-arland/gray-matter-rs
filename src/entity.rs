/// `ParsedEntity` stores a parsed result with given data type `D`.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// # use gray_matter::{Matter, Pod, ParsedEntity};
/// # use gray_matter::engine::TOML;
/// let text = r#"---
/// field = "Value"
/// ---
/// Here is excerpt
/// ---
/// Here is content"#;
///
/// let matter = Matter::<TOML>::new();
/// let result: ParsedEntity = matter.parse(text).unwrap();
///
/// assert_eq!(result.data.unwrap()["field"], Pod::String("Value".to_owned()));
/// assert_eq!(result.excerpt, Some("Here is excerpt".to_owned()));
/// assert_eq!(result.content, "Here is excerpt\n---\nHere is content")
/// ```
#[derive(PartialEq, Debug)]
pub struct ParsedEntity<D: serde::de::DeserializeOwned = crate::Pod> {
    /// `D` if front matter was found. `None` otherwise.
    pub data: Option<D>,
    /// The full input, but with the front matter and delimiters stripped out. Any excerpt is also
    /// part of this field.
    pub content: String,
    /// A string containing the excerpt, if found. `None` otherwise.
    pub excerpt: Option<String>,
    /// The original input.
    pub orig: String,
    /// The raw front matter. Empty string if no front matter is found.
    pub matter: String,
}
