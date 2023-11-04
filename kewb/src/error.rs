use bincode::error::{DecodeError, EncodeError};
use core::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidColor,
    InvalidEdge,
    InvalidCorner,
    InvalidScramble,
    InvalidFaceletString,
    InvalidFaceletValue,
    InvalidCubieValue,
    IOError(io::Error),
    DecodeError(DecodeError),
    EncodeError(EncodeError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<EncodeError> for Error {
    fn from(value: EncodeError) -> Self {
        Self::EncodeError(value)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColor => write!(f, "Invalid color value"),
            Self::InvalidEdge => write!(f, "Invalid edge value"),
            Self::InvalidCorner => write!(f, "Invalid corner value"),
            Self::InvalidScramble => write!(f, "Invalid scramble"),
            Self::InvalidFaceletString => write!(f, "Invalid facelet string"),
            Self::InvalidFaceletValue => write!(f, "Invalid facelet reperesentation"),
            Self::InvalidCubieValue => write!(f, "Invalid cubie reperesentation"),
            Self::IOError(value) => value.fmt(f),
            Self::DecodeError(value) => value.fmt(f),
            Self::EncodeError(value) => value.fmt(f),
        }
    }
}
