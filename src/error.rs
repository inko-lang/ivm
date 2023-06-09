use getopts::Fail;
use std::fmt;
use std::io;

/// A generic error produced by the CLI.
///
/// An Error can be created manually using
/// [`Error::generic()`][`Error::generic()`], or by converting one of many
/// supported error types into it.
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl Error {
    /// Produces an error with a message.
    pub fn generic<S: Into<String>>(message: S) -> Self {
        Error {
            message: message.into(),
        }
    }
}

impl From<Fail> for Error {
    fn from(fail: Fail) -> Self {
        Error {
            message: fail.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
