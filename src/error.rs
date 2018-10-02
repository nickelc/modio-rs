//! Client errors

use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use std::time::Duration;

use failure::Backtrace;
use failure::Context;
use failure::Fail;
use http::uri::InvalidUri;
use http::Error as HttpError;
use hyper::Error as HyperError;
use hyper::StatusCode;
use serde_json::Error as SerdeError;

pub use types::ClientError;

pub type Result<T> = StdResult<T, Error>;

macro_rules! future_err {
    ($e:expr) => {
        Box::new(::futures::future::err($e))
    };
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Message(String),
    #[fail(display = "{}: {}", code, error)]
    Fault {
        code: StatusCode,
        error: ClientError,
    },
    #[fail(
        display = "API rate limit reached. Try again in {:?}.",
        reset,
    )]
    RateLimit { reset: Duration },
    #[fail(display = "Download failed: {}", _0)]
    Download(#[fail(cause)] DownloadError),
    #[fail(display = "Serde Error: {}", _0)]
    Codec(#[fail(cause)] SerdeError),
    #[fail(display = "Failed to create http request: {}", _0)]
    Http(#[fail(cause)] HttpError),
    #[fail(display = "{}", _0)]
    Hyper(#[fail(cause)] HyperError),
    #[fail(display = "IO Error: {}", _0)]
    Io(#[fail(cause)] IoError),
    #[fail(display = "Invalid Uri: {}", _0)]
    Uri(#[fail(cause)] InvalidUri),
}

#[derive(Debug, Fail)]
pub enum DownloadError {
    /// The mod has no primary file.
    #[fail(
        display = "Mod {{id: {1}, game_id: {0}}}: Mod has no primary file.",
        game_id,
        mod_id,
    )]
    NoPrimaryFile { game_id: u32, mod_id: u32 },
    /// The specific file of a mod was not found.
    #[fail(
        display = "Mod {{id: {1}, game_id: {0}}}: File {{ id: {2} }} not found.",
        game_id,
        mod_id,
        file_id,
    )]
    FileNotFound {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    /// Multiple files for a given version were found and the policy was set to
    /// [`ResolvePolicy::Fail`](../download/enum.ResolvePolicy.html#variant.Fail).
    #[fail(
        display = "Mod {{id: {1}, game_id: {0}}}: Multiple files found for version '{2}'.",
        game_id,
        mod_id,
        version,
    )]
    MultipleFilesFound {
        game_id: u32,
        mod_id: u32,
        version: String,
    },
    /// No file for a given version was found.
    #[fail(
        display = "Mod {{id: {1}, game_id: {0}}}: No file with version '{2}' found.",
        game_id,
        mod_id,
        version,
    )]
    VersionNotFound {
        game_id: u32,
        mod_id: u32,
        version: String,
    },
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref errors) = self.errors {
            writeln!(f, "{}", self.message);
            for (k, v) in errors {
                writeln!(f, "  {}: {}", k, v);
            }
            Ok(())
        } else {
            fmt::Display::fmt(&self.message, f)
        }
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
    }).into()
}

pub(crate) fn download_multiple_files<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    ErrorKind::Download(DownloadError::MultipleFilesFound {
        game_id,
        mod_id,
        version: version.into(),
    }).into()
}

pub(crate) fn download_version_not_found<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    ErrorKind::Download(DownloadError::VersionNotFound {
        game_id,
        mod_id,
        version: version.into(),
    }).into()
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

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        ErrorKind::Codec(err).into()
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
