use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    CannotUseLessStdin,
    InvalidCliOptionValue(&'static str),
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "IO error: {}", error),
            Error::CannotUseLessStdin => write!(f, "Cannot open stdin stream for 'less' process"),
            Error::InvalidCliOptionValue(opt) => write!(
                f,
                "Invalid value provided for command line option '{}'",
                opt
            ),
        }
    }
}
