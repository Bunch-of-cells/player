use std::error;
use std::fmt::Display;
use std::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ExecutionError(i32, Box<dyn error::Error>),
    InvalidFileData(Box<dyn error::Error>),
    Other(Box<dyn error::Error>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "{}", err),
            Error::ExecutionError(code, e) => {
                write!(f, "{} (Error code {})", e, code)
            }
            Error::InvalidFileData(e) => {
                write!(f, "InvalidFileData: {}", e)
            }
            Error::Other(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {}
