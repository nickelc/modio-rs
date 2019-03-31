//! Client errors
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::time::Duration;

use http::header::InvalidHeaderValue;
use http::Error as HttpError;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use serde_json::Error as JsonError;
use url::ParseError;

pub use crate::types::ClientError;

pub type Result<T> = StdResult<T, Error>;

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
            ErrorKind::Reqwest(ref e) => Some(e),
            ErrorKind::Json(ref e) => Some(e),
            ErrorKind::Io(ref e) => Some(e),
            ErrorKind::Url(ref e) => Some(e),
            ErrorKind::Auth(_)
            | ErrorKind::RateLimit { .. }
            | ErrorKind::Message(_)
            | ErrorKind::Validation(_, _) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    fn new(kind: ErrorKind) -> Error {
        Error {
            inner: Box::new(kind),
        }
    }

    pub fn is_client_error(&self) -> bool {
        match *self.inner {
            ErrorKind::Fault { .. } => true,
            ErrorKind::Validation(_, _) => true,
            ErrorKind::Reqwest(ref e) => e.is_client_error(),
            _ => false,
        }
    }

    pub fn is_server_error(&self) -> bool {
        match *self.inner {
            ErrorKind::Reqwest(ref e) => e.is_server_error(),
            _ => false,
        }
    }

    pub fn is_serialization(&self) -> bool {
        match *self.inner {
            ErrorKind::Json(_) => true,
            _ => false,
        }
    }

    pub fn is_validation(&self) -> bool {
        match *self.inner {
            ErrorKind::Validation(_, _) => true,
            _ => false,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.inner
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Message(String),
    Auth(AuthenticationError),
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    Validation(String, HashMap<String, String>),
    RateLimit {
        reset: Duration,
    },
    Download(DownloadError),
    Json(JsonError),
    Http(HttpError),
    Reqwest(ReqwestError),
    Io(IoError),
    Url(ParseError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Message(msg) => msg.fmt(fmt),
            ErrorKind::Auth(e) => e.fmt(fmt),
            ErrorKind::Fault { code, error } => write!(fmt, "{}: {}", code, error),
            ErrorKind::Validation(message, errors) => {
                write!(fmt, "Validation failed: '{}' {:?}", message, errors)
            }
            ErrorKind::RateLimit { reset } => {
                write!(fmt, "API rate limit reached. Try again in {:?}.", reset)
            }
            ErrorKind::Download(e) => write!(fmt, "Download failed: {}", e),
            ErrorKind::Json(e) => e.fmt(fmt),
            ErrorKind::Http(e) => e.fmt(fmt),
            ErrorKind::Reqwest(e) => e.fmt(fmt),
            ErrorKind::Io(e) => e.fmt(fmt),
            ErrorKind::Url(e) => e.fmt(fmt),
        }
    }
}

#[derive(Debug)]
pub enum AuthenticationError {
    ApiKeyRequired,
    TokenRequired,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthenticationError::ApiKeyRequired => f.write_str("API key is required"),
            AuthenticationError::TokenRequired => f.write_str("Authentication token is required"),
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

pub(crate) fn apikey_required() -> Error {
    Error::new(ErrorKind::Auth(AuthenticationError::ApiKeyRequired))
}

pub(crate) fn token_required() -> Error {
    Error::new(ErrorKind::Auth(AuthenticationError::TokenRequired))
}

pub(crate) fn client(code: StatusCode, error: ClientError) -> Error {
    if code == 422 {
        Error::new(ErrorKind::Validation(
            error.message,
            error.errors.unwrap_or_default(),
        ))
    } else {
        Error::new(ErrorKind::Fault { code, error })
    }
}

pub(crate) fn ratelimit(reset: u64) -> Error {
    Error::new(ErrorKind::RateLimit {
        reset: Duration::from_secs(reset * 60),
    })
}

pub(crate) fn download_no_primary(game_id: u32, mod_id: u32) -> Error {
    Error::new(ErrorKind::Download(DownloadError::NoPrimaryFile {
        game_id,
        mod_id,
    }))
}

pub(crate) fn download_file_not_found(game_id: u32, mod_id: u32, file_id: u32) -> Error {
    Error::new(ErrorKind::Download(DownloadError::FileNotFound {
        game_id,
        mod_id,
        file_id,
    }))
}

pub(crate) fn download_multiple_files<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    Error::new(ErrorKind::Download(DownloadError::MultipleFilesFound {
        game_id,
        mod_id,
        version: version.into(),
    }))
}

pub(crate) fn download_version_not_found<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    Error::new(ErrorKind::Download(DownloadError::VersionNotFound {
        game_id,
        mod_id,
        version: version.into(),
    }))
}

impl From<String> for ErrorKind {
    fn from(s: String) -> ErrorKind {
        ErrorKind::Message(s)
    }
}

impl<'a> From<&'a str> for ErrorKind {
    fn from(s: &'a str) -> ErrorKind {
        ErrorKind::Message(s.into())
    }
}

impl From<JsonError> for ErrorKind {
    fn from(err: JsonError) -> ErrorKind {
        ErrorKind::Json(err)
    }
}

impl From<InvalidHeaderValue> for ErrorKind {
    fn from(err: InvalidHeaderValue) -> ErrorKind {
        ErrorKind::Http(err.into())
    }
}

impl From<ReqwestError> for ErrorKind {
    fn from(err: ReqwestError) -> ErrorKind {
        ErrorKind::Reqwest(err)
    }
}

impl From<IoError> for ErrorKind {
    fn from(err: IoError) -> ErrorKind {
        ErrorKind::Io(err)
    }
}

impl From<ParseError> for ErrorKind {
    fn from(err: ParseError) -> ErrorKind {
        ErrorKind::Url(err)
    }
}

#[allow(missing_debug_implementations)]
pub(crate) struct InternalFrom<T>(pub T);

impl From<InternalFrom<Error>> for Error {
    fn from(err: InternalFrom<Error>) -> Error {
        err.0
    }
}

impl<T> From<InternalFrom<T>> for Error
where
    T: Into<ErrorKind>,
{
    fn from(err: InternalFrom<T>) -> Error {
        Error {
            inner: Box::new(err.0.into()),
        }
    }
}

pub(crate) fn from<T>(err: T) -> Error
where
    T: Into<ErrorKind>,
{
    InternalFrom(err).into()
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
