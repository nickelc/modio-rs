use std::io::Error as IoError;
use std::time::Duration;

use hyper::Error as HttpError;
use hyper::StatusCode;
use hyper::error::UriError;
use serde_json::Error as SerdeError;

use types::ClientError;

#[derive(Debug)]
pub enum Error {
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    RateLimit {
        reset: Duration,
    },
    Codec(SerdeError),
    Http(HttpError),
    Io(IoError),
    Uri(UriError),
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

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Error::Uri(err)
    }
}
