use std;
use rusqlite;

#[derive(Debug)]
pub enum Error {
    IOFailure(std::io::Error),
    RusqliteFailure(rusqlite::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOFailure(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::RusqliteFailure(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::IOFailure(ref err) => err.fmt(f),
            Error::RusqliteFailure(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IOFailure(ref err) => err.description(),
            Error::RusqliteFailure(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::IOFailure(ref err) => Some(err),
            Error::RusqliteFailure(ref err) => Some(err),
        }
    }
}
