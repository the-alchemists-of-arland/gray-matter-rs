pub mod engine;

#[doc(hidden)]
pub mod entity;
#[doc(inline)]
pub use entity::{ParsedEntity, ParsedEntityStruct};

#[doc(hidden)]
pub mod matter;
#[doc(inline)]
pub use matter::Matter;

#[doc(hidden)]
pub mod value;
#[doc(inline)]
pub use value::{pod::Pod, error::Error};

#[cfg(test)]
mod tests;
