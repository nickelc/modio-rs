//! Client errors

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::time::Duration;

use http::uri::InvalidUri;
use http::Error as HttpError;
use hyper::Error as HyperError;
use hyper::StatusCode;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use url::ParseError;

pub use crate::types::ClientError;

pub type Result<T> = StdResult<T, Error>;

macro_rules! future_err {
    ($e:expr) => {
        Box::new(::futures::future::err($e))
    };
}

#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorKind>,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self.inner {
            ErrorKind::Download(ref e) => Some(e),
            ErrorKind::Fault { ref error, .. } => Some(error),
            ErrorKind::Http(ref e) => Some(e),
            ErrorKind::Hyper(ref e) => Some(e),
            ErrorKind::Reqwest(ref e) => Some(e),
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::Uri(ref e) => Some(e),
            ErrorKind::Url(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.inner
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(kind),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Message(String),
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    RateLimit {
        reset: Duration,
    },
    Download(DownloadError),
    Json(JsonError),
    Http(HttpError),
    Hyper(HyperError),
    Reqwest(ReqwestError),
    Io(IoError),
    Uri(InvalidUri),
    Url(ParseError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Message(msg) => msg.fmt(fmt),
            ErrorKind::Fault { code, error } => write!(fmt, "{}: {}", code, error),
            ErrorKind::RateLimit { reset } => {
                write!(fmt, "API rate limit reached. Try again in {:?}.", reset)
            }
            ErrorKind::Download(e) => write!(fmt, "Download failed: {}", e),
            ErrorKind::Json(e) => e.fmt(fmt),
            ErrorKind::Http(e) => write!(fmt, "Failed to create http request: {}", e),
            ErrorKind::Hyper(e) => e.fmt(fmt),
            ErrorKind::Reqwest(e) => e.fmt(fmt),
            ErrorKind::Io(e) => e.fmt(fmt),
            ErrorKind::Uri(e) => e.fmt(fmt),
            ErrorKind::Url(e) => e.fmt(fmt),
        }
    }
}

#[derive(Debug)]
pub enum DownloadError {
    /// The mod has no primary file.
    NoPrimaryFile { game_id: u32, mod_id: u32 },
    /// The specific file of a mod was not found.
    FileNotFound {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    /// Multiple files for a given version were found and the policy was set to
    /// [`ResolvePolicy::Fail`](../download/enum.ResolvePolicy.html#variant.Fail).
    MultipleFilesFound {
        game_id: u32,
        mod_id: u32,
        version: String,
    },
    /// No file for a given version was found.
    VersionNotFound {
        game_id: u32,
        mod_id: u32,
        version: String,
    },
}

impl StdError for DownloadError {}

impl fmt::Display for DownloadError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::FileNotFound {
                game_id,
                mod_id,
                file_id,
            } => write!(
                fmt,
                "Mod {{id: {1}, game_id: {0}}}: File {{ id: {2} }} not found.",
                game_id, mod_id, file_id,
            ),
            DownloadError::MultipleFilesFound {
                game_id,
                mod_id,
                version,
            } => write!(
                fmt,
                "Mod {{id: {1}, game_id: {0}}}: Multiple files found for version '{2}'.",
                game_id, mod_id, version,
            ),
            DownloadError::NoPrimaryFile { game_id, mod_id } => write!(
                fmt,
                "Mod {{id: {1}, game_id: {0}}} Mod has no primary file.",
                game_id, mod_id,
            ),
            DownloadError::VersionNotFound {
                game_id,
                mod_id,
                version,
            } => write!(
                fmt,
                "Mod {{id: {1}, game_id: {0}}}: No file with version '{2}' found.",
                game_id, mod_id, version,
            ),
        }
    }
}

impl StdError for ClientError {}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&self.message);
        if let Some(ref errors) = self.errors {
            for (k, v) in errors {
                buf.push('\n');
                buf.push_str("  ");
                buf.push_str(&k);
                buf.push_str(": ");
                buf.push_str(&v);
            }
        }
        fmt::Display::fmt(&buf, f)
    }
}

pub(crate) fn download_no_primary(game_id: u32, mod_id: u32) -> Error {
    ErrorKind::Download(DownloadError::NoPrimaryFile { game_id, mod_id }).into()
}

pub(crate) fn download_file_not_found(game_id: u32, mod_id: u32, file_id: u32) -> Error {
    ErrorKind::Download(DownloadError::FileNotFound {
        game_id,
        mod_id,
        file_id,
    })
    .into()
}

pub(crate) fn download_multiple_files<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    ErrorKind::Download(DownloadError::MultipleFilesFound {
        game_id,
        mod_id,
        version: version.into(),
    })
    .into()
}

pub(crate) fn download_version_not_found<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    ErrorKind::Download(DownloadError::VersionNotFound {
        game_id,
        mod_id,
        version: version.into(),
    })
    .into()
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        ErrorKind::Message(s).into()
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Error {
        ErrorKind::Message(s.into()).into()
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        ErrorKind::Json(err).into()
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        ErrorKind::Http(err).into()
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        ErrorKind::Hyper(err).into()
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        ErrorKind::Reqwest(err).into()
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        ErrorKind::Uri(err).into()
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        ErrorKind::Url(err).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn display_client_error() {
        let e = ClientError {
            code: 1,
            message: "Message".to_string(),
            errors: None,
        };
        assert_eq!(e.to_string(), "Message");

        let e = ClientError {
            errors: Some(HashMap::new()),
            ..e
        };
        assert_eq!(e.to_string(), "Message");

        let mut map = HashMap::new();
        map.insert("A".to_string(), "1".to_string());

        let e = ClientError {
            errors: Some(map),
            ..e
        };
        assert_eq!(e.to_string(), "Message\n  A: 1");
    }
}
