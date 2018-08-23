use actix_web;
use std::{self, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ActixWeb(actix_web::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ioe) => ioe.fmt(f),
            Error::ActixWeb(awe) => awe.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(ioe: io::Error) -> Self {
        Error::Io(ioe)
    }
}

impl From<actix_web::error::Error> for Error {
    #[inline]
    fn from(awe: actix_web::error::Error) -> Self {
        Error::ActixWeb(awe)
    }
}
