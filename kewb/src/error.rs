use bincode::error::{DecodeError, EncodeError};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid color value")]
    InvalidColor,
    #[error("Invalid edge value")]
    InvalidEdge,
    #[error("Invalid corner value")]
    InvalidCorner,
    #[error("Invalid scramble string")]
    InvalidScramble,
    #[error("Invalid facelet string")]
    InvalidFaceletString,
    #[error("Invalid facelet reperesentation")]
    InvalidFaceletValue,
    #[error("Invalid cubie reperesentation")]
    InvalidCubieValue,
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    DecodeError(#[from] DecodeError),
    #[error("{0}")]
    EncodeError(#[from] EncodeError),
}
