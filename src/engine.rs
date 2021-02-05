use crate::value::pod::Pod;

pub mod json;
pub mod toml;
pub mod yaml;

pub trait Engine {
    fn new() -> Self;
    fn parse(&self, content: &str) -> Pod;
}
