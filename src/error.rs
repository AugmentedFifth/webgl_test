use bincode;
use jpeg;
use js;
use png;
use std::{self, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Img(String),
    Png(png::DecodingError),
    Gl(String),
    Logic(String),
    Bincode(bincode::Error),
    Jpeg(jpeg::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ioe) => ioe.fmt(f),
            Error::Img(s) => f.write_str(s),
            Error::Png(pde) => pde.fmt(f),
            Error::Gl(s) => f.write_str(s),
            Error::Logic(s) => f.write_str(s),
            Error::Bincode(bce) => bce.fmt(f),
            Error::Jpeg(je) => je.fmt(f),
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

impl From<png::DecodingError> for Error {
    #[inline]
    fn from(pde: png::DecodingError) -> Self {
        Error::Png(pde)
    }
}

impl From<bincode::Error> for Error {
    #[inline]
    fn from(bce: bincode::Error) -> Self {
        Error::Bincode(bce)
    }
}

impl From<jpeg::Error> for Error {
    #[inline]
    fn from(je: jpeg::Error) -> Self {
        Error::Jpeg(je)
    }
}

#[inline]
pub fn log_and_return(res: Result<(), Error>) -> i32 {
    match res {
        Ok(_) => 0,
        Err(e) => {
            js::log(&e.to_string());
            1
        },
    }
}
