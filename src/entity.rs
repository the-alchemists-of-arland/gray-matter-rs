use crate::value::pod::Pod;

/// **ParsedEntity** stores the parse result.
#[derive(PartialEq, Debug)]
pub struct ParsedEntity {
    pub data: Pod,
    pub content: String,
    pub excerpt: String,
    pub orig: String,
}

/// **ParsedEntity** stores the parse result and deserialize data to struct.
#[derive(PartialEq, Debug)]
pub struct ParsedEntityStruct<T: serde::de::DeserializeOwned> {
    pub data: T,
    pub content: String,
    pub excerpt: String,
    pub orig: String,
}
