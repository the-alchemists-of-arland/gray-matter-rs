use std::error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    TypeError(String),
    DeserializeError(String),
}

impl Error {
    pub fn type_error(expected: &str) -> Self {
        Error::TypeError(expected.into())
    }

    pub fn deserialize_error(msg: String) -> Self {
        Error::DeserializeError(msg)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Error::*;

        match *self {
            TypeError(ref s) => write!(f, "Type error, expected: {s}"),
            DeserializeError(ref s) => write!(f, "Deserialize error: {s}"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            TypeError(_) => "Type error",
            DeserializeError(_) => "Deserialize error",
        }
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Self {
        Error::deserialize_error(err.to_string())
    }
}
