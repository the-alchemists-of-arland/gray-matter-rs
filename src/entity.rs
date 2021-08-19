use crate::Pod;

/// `ParsedEntity` stores a parsed result.
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
/// let result: ParsedEntity = matter.parse(text);
/// 
/// assert_eq!(result.data.unwrap()["field"], Pod::String("Value".to_owned()));
/// assert_eq!(result.excerpt, Some("Here is excerpt".to_owned()));
/// assert_eq!(result.content, "Here is excerpt\n---\nHere is content")
/// ```
#[derive(PartialEq, Debug)]
pub struct ParsedEntity {
    /// [`Some(Pod)`](crate::Pod) if front matter was found. `None` otherwise.
    pub data: Option<Pod>,
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

/// `ParsedEntityStruct` stores the parsed result with the front matter deserialized into a struct `T`.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
/// # use gray_matter::{Matter, Pod, ParsedEntityStruct};
/// # use gray_matter::engine::YAML;
/// #[derive(serde::Deserialize)]
/// struct FrontMatter {
///     field: i32,
/// }
///
/// let text = r#"---
/// field: -134
/// ---
/// Here is excerpt
/// ---
/// Here is content"#;
///
/// let matter = Matter::<YAML>::new();
/// let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(text).unwrap();
/// 
/// assert_eq!(result.data.field, -134);
/// assert_eq!(result.excerpt, Some("Here is excerpt".to_owned()));
/// assert_eq!(result.content, "Here is excerpt\n---\nHere is content")
/// ```
#[derive(PartialEq, Debug)]
pub struct ParsedEntityStruct<T: serde::de::DeserializeOwned> {
    /// The front matter data, deserialized into `T`.
    pub data: T,
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
