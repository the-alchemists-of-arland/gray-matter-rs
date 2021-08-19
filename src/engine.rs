use crate::Pod;

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

/// The trait requirement used by [`Matter`](crate::Matter) when parsing the front matter.
///
/// Implementing this trait in your own engine will allow you to create a custom front matter
/// format that can be used by [gray_matter](crate).
pub trait Engine {
    fn parse(content: &str) -> Pod;
}
