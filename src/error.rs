//! Client errors

use std::io::Error as IoError;
use std::time::Duration;

use http::uri::InvalidUri;
use http::Error as HttpError;
use hyper::Error as HyperError;
use hyper::StatusCode;
use serde_json::Error as SerdeError;

pub use types::ClientError;

#[derive(Debug)]
pub enum Error {
    Msg(String),
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    RateLimit {
        reset: Duration,
    },
    Download(String),
    Codec(SerdeError),
    Http(HttpError),
    Hyper(HyperError),
    Io(IoError),
    Uri(InvalidUri),
}

macro_rules! future_err {
    (dl $s:expr) => {
        Box::new(::futures::future::err(::error::Error::Download($s)))
    };
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Msg(s)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Error {
        Error::Msg(s.into())
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::Codec(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::Http(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Hyper(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error::Uri(err)
    }
}
