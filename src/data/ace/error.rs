use std::error::Error as StdError;
use std::fmt::Display;
use std::io::Error as IOError;

/// The error type for [`ace`](crate::data::ace) module.
#[derive(Debug)]
pub enum AceError {
    /// Invalid data.
    Data,
    /// Reached end of file.
    EndOfFile,
    /// Invalid format.
    Format,
    /// I/O error.
    IO(IOError),
}

impl Display for AceError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AceError::Data => write!(fmt, "invalid ACE data"),
            AceError::EndOfFile => write!(fmt, "reached end of ACE file"),
            AceError::Format => write!(fmt, "invalid ACE format"),
            AceError::IO(_) => write!(fmt, "ACE I/O error"),
        }
    }
}

impl StdError for AceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AceError::IO(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IOError> for AceError {
    fn from(error: IOError) -> Self {
        AceError::IO(error)
    }
}
