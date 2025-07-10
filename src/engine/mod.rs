use crate::Pod;
use crate::Result;

#[doc(hidden)]
pub mod json;
#[cfg(feature = "toml")]
#[doc(hidden)]
pub mod toml;
#[cfg(feature = "yaml")]
#[doc(hidden)]
pub mod yaml;

#[doc(inline)]
pub use crate::engine::json::JSON;
#[cfg(feature = "toml")]
#[doc(inline)]
pub use crate::engine::toml::TOML;
#[cfg(feature = "yaml")]
#[doc(inline)]
pub use crate::engine::yaml::YAML;

/// The trait requirement used by [`Matter`](crate::Matter) when parsing the front matter.
///
/// Implementing this trait in your own engine will allow you to create a custom front matter
/// format that can be used by [gray_matter](crate).
pub trait Engine {
    fn parse(content: &str) -> Result<Pod>;
}
