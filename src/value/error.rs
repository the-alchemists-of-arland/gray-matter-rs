use std::error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    TypeError(String),
}

impl Error {
    pub fn type_error(expected: &str) -> Self {
        Error::TypeError(expected.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Error::*;

        match *self {
            TypeError(ref s) => write!(f, "Type error, expected: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            TypeError(_) => "Type error",
        }
    }
}
