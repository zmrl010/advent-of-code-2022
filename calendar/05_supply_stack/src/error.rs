use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum ParseError {
    ParseStringError { from: String },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParseStringError { from } => {
                write!(f, "ParseError: cannot parse string `{from}`",)
            }
        }
    }
}

impl error::Error for ParseError {}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
}

impl error::Error for Error {}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(err) => {
                write!(f, "Error: {err}")
            }
        }
    }
}
