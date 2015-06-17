use std::io;
use std::borrow::Cow;

use ::glob::PatternError;
use ::yaml::ScanError;

#[derive(Debug)]
pub enum Error {
    Yaml(ScanError),
    Read(io::Error),
    Config(Cow<'static, str>),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Read(e)
    }
}

impl From<ScanError> for Error {
    fn from(e: ScanError) -> Error {
        Error::Yaml(e)
    }
}

impl From<PatternError> for Error {
    fn from(_: PatternError) -> Error {
        Error::from("invalid index directory pattern")
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Error {
        Error::Config(Cow::Borrowed(e))
    }
}


impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::Config(Cow::Owned(e))
    }
}

