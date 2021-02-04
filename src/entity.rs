use crate::value::pod::Pod;

/// **ParsedEntity** stores the parse result.
#[derive(PartialEq, Debug)]
pub struct ParsedEntity {
    pub data: Pod,
    pub content: &'static str,
    pub excerpt: &'static str,
    pub orig: &'static str,
}
