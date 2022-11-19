use std::io::Error as IOError;
use std::{error::Error as StdError, fmt::Display};

/// The error type for [`endf`](crate::data::endf) module.
#[derive(Debug)]
pub enum EndfError {
    /// Invalid data.
    Data,
    /// Invalid encoding.
    Encoding,
    /// Reached end of file.
    EndOfFile,
    /// Invalid format.
    Format,
    /// I/O error.
    IO(IOError),
}

impl Display for EndfError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndfError::Data => write!(fmt, "invalid ENDF data"),
            EndfError::Encoding => write!(fmt, "ENDF encoding error"),
            EndfError::EndOfFile => write!(fmt, "reached end of ENDF file"),
            EndfError::Format => write!(fmt, "invalid ENDF format"),
            EndfError::IO(_) => write!(fmt, "ENDF I/O error"),
        }
    }
}

impl StdError for EndfError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            EndfError::IO(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IOError> for EndfError {
    fn from(error: IOError) -> Self {
        EndfError::IO(error)
    }
}
