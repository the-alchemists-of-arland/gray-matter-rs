use crate::engine::Engine;

/// **ParsedEntity** stores the parse result.
#[derive(PartialEq, Debug)]
pub struct ParsedEntity<T: Engine> {
    pub data: T::Output,
    pub content: &'static str,
    pub excerpt: &'static str,
    pub orig: &'static str,
}
