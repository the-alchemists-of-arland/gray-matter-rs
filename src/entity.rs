use crate::value::pod::Pod;

/// **ParsedEntity** stores the parse result.
#[derive(PartialEq, Debug)]
pub struct ParsedEntity {
    pub data: Pod,
    pub content: &'static str,
    pub excerpt: &'static str,
    pub orig: &'static str,
}

/// **ParsedEntity** stores the parse result and deserialize data to struct.
#[derive(PartialEq, Debug)]
pub struct ParsedEntityStruct<T: serde::de::DeserializeOwned> {
    pub data: T,
    pub content: &'static str,
    pub excerpt: &'static str,
    pub orig: &'static str,
}
