use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Type error, expected: {0}")]
    TypeError(String),
    #[error("Deserialize error: {0}")]
    DeserializeError(String),
    #[error("Value is missin")]
    ValueMissingError,
    #[error("Unsupported error: {0}")]
    UnsupportedError(String),
}

impl Error {
    pub fn type_error(expected: &str) -> Self {
        Error::TypeError(expected.into())
    }

    pub fn deserialize_error(msg: &str) -> Self {
        Error::DeserializeError(msg.into())
    }

    pub fn value_missing() -> Self {
        Error::ValueMissingError
    }

    pub fn unsupported(msg: &str) -> Self {
        Error::UnsupportedError(msg.into())
    }
}
