/*  Copyright (C) 2015 Steven Allen
 *
 *  This file is part of gazetta.
 *
 *  This program is free software: you can redistribute it and/or modify it under the terms of the
 *  GNU General Public License as published by the Free Software Foundation version 3 of the
 *  License.
 *
 *  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 *  without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
 *  the GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along with this program.  If
 *  not, see <http://www.gnu.org/licenses/>.
 */

use std::io;
use std::fmt;
use std::path::PathBuf;
use std::borrow::Cow;
use ::horrorshow;

use ::glob::PatternError;
use ::yaml::ScanError;

use std::error::Error;

macro_rules! try_annotate {
    ($e:expr, $l:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err($crate::error::AnnotatedError::new(($l).to_owned(), From::from(e))),
        }
    }
}


#[derive(Debug)]
pub enum SourceError {
    Parse(ScanError),
    Read(io::Error),
    Config(Cow<'static, str>),
}

impl Error for SourceError {
    fn description(&self) -> &str {
        use self::SourceError::*;
        match *self {
            Parse(..) => "yaml parse error",
            Read(..) => "read error",
            Config(..) => "config error",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::SourceError::*;
        match *self {
            Read(ref e) => Some(e),
            // Parse(ref e) => e, // TODO: wait for upstream
            _ => None,
        }
    }
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SourceError::*;
        match *self {
            Parse(ref e) => write!(f, "yaml parse error '{:?}'", e),
            Read(ref e) => write!(f, "read error '{}'", e),
            Config(ref e) => write!(f, "config error '{}'", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnnotatedError<E> where E: Error {
    pub location: PathBuf,
    pub error: E,
}

// Would like to make this generic but coherence...
impl From<AnnotatedError<io::Error>> for AnnotatedError<RenderError> {
    fn from(e: AnnotatedError<io::Error>) -> AnnotatedError<RenderError> {
        AnnotatedError {
            location: e.location,
            error: From::from(e.error),
        }
    }
}

impl<E> Error for AnnotatedError<E> where E: Error {
    fn description(&self) -> &str {
        self.error.description()
    }
    fn cause(&self) -> Option<&Error> {
        Some(&self.error)
    }
}

impl<E> fmt::Display for AnnotatedError<E> where E: Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "when processing {}: {}", self.location.display(), self.error)
    }
}

impl<E> AnnotatedError<E> where E: Error {
    pub fn new(location: PathBuf, error: E) -> AnnotatedError<E> {
        AnnotatedError {
            location: location,
            error: error,
        }
    }
}

impl From<io::Error> for SourceError {
    fn from(e: io::Error) -> SourceError {
        SourceError::Read(e)
    }
}

impl From<::url::ParseError> for SourceError {
    fn from(e: ::url::ParseError) -> SourceError {
        SourceError::Config(Cow::Owned(format!("{}", e)))
    }
}

impl From<ScanError> for SourceError {
    fn from(e: ScanError) -> SourceError {
        SourceError::Parse(e)
    }
}

impl From<PatternError> for SourceError {
    fn from(_: PatternError) -> SourceError {
        SourceError::from("invalid index directory pattern")
    }
}

impl From<&'static str> for SourceError {
    fn from(e: &'static str) -> SourceError {
        SourceError::Config(Cow::Borrowed(e))
    }
}

impl From<String> for SourceError {
    fn from(e: String) -> SourceError {
        SourceError::Config(Cow::Owned(e))
    }
}

pub type RenderError = horrorshow::Error;
