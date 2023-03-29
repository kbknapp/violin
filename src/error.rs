//! This module defines the `Result<T>` type alias and internal `Error` type

use crate::std::result::Result as StdResult;
#[cfg(feature = "std")]
use crate::std::{error::Error as StdError, fmt};

/// A type alias for returning `Result<T, violin::error::Error>`
pub type Result<T> = StdResult<T, Error>;

/// Defines Violin errors for fallible APIs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A component of the coordinate vector became NaN or Infinite
    InvalidCoordinate,
}

/// The Violin error type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) kind: ErrorKind,
}

impl Error {
    /// Returns true if the error is due to an invalid coordinate
    pub fn is_invalid(&self) -> bool { self.kind == ErrorKind::InvalidCoordinate }
}

#[cfg(feature = "std")]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidCoordinate => write!(f, "invalid coordinate"),
        }
    }
}

#[cfg(feature = "std")]
impl StdError for Error {}
