use crate::value::pod::Pod;

#[doc(hidden)]
pub mod json;
#[doc(hidden)]
pub mod toml;
#[doc(hidden)]
pub mod yaml;

#[doc(inline)]
pub use crate::engine::yaml::YAML;
#[doc(inline)]
pub use crate::engine::toml::TOML;
#[doc(inline)]
pub use crate::engine::json::JSON;

pub trait Engine {
    fn new() -> Self;
    fn parse(&self, content: &str) -> Pod;
}
