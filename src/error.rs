//! Client errors
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

use reqwest::StatusCode;

use crate::auth::Error as AuthError;
use crate::download::Error as DownloadError;
use crate::types::Error as ModioError;

/// A `Result` alias where the `Err` case is `modio::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The Errors that may occur when using `Modio`.
pub struct Error {
    inner: Box<Inner>,
}

type BoxError = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    error_ref: Option<u16>,
    source: Option<BoxError>,
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                error_ref: None,
                source: source.map(Into::into),
            }),
        }
    }

    pub(crate) fn new_error_ref<E>(kind: Kind, error_ref: u16, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                error_ref: Some(error_ref),
                source: source.map(Into::into),
            }),
        }
    }

    /// Returns true if the API key/access token is incorrect, revoked, expired or the request
    /// needs a different authentication method.
    pub fn is_auth(&self) -> bool {
        matches!(self.inner.kind, Kind::Auth)
    }

    /// Returns true if the error is from a type Builder.
    pub fn is_builder(&self) -> bool {
        matches!(self.inner.kind, Kind::Builder)
    }

    /// Returns true if the error is from a [`DownloadAction`](crate::download::DownloadAction).
    pub fn is_download(&self) -> bool {
        matches!(self.inner.kind, Kind::Download)
    }

    /// Returns true if the rate limit associated with credentials has been exhausted.
    pub fn is_ratelimited(&self) -> bool {
        matches!(self.inner.kind, Kind::RateLimit { .. })
    }

    /// Returns true if the error was generated from a response.
    pub fn is_status(&self) -> bool {
        matches!(self.inner.kind, Kind::Status(_))
    }

    /// Returns true if the error contains validation errors.
    pub fn is_validation(&self) -> bool {
        matches!(self.inner.kind, Kind::Validation(_, _))
    }

    /// Returns true if the error is related to serialization.
    pub fn is_decode(&self) -> bool {
        matches!(self.inner.kind, Kind::Decode)
    }

    /// Returns modio's error reference code.
    ///
    /// See the [Error Codes](https://docs.mod.io/#error-codes) docs for more information.
    pub fn error_ref(&self) -> Option<u16> {
        self.inner.error_ref
    }

    /// Returns status code if the error was generated from a response.
    pub fn status(&self) -> Option<StatusCode> {
        match self.inner.kind {
            Kind::Status(code) => Some(code),
            _ => None,
        }
    }

    /// Returns validation message & errors from the response.
    pub fn validation(&self) -> Option<(&String, &HashMap<String, String>)> {
        match self.inner.kind {
            Kind::Validation(ref msg, ref errors) => Some((msg, errors)),
            _ => None,
        }
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.inner.kind
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("modio::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }
        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.kind {
            Kind::Auth => f.write_str("authentication error")?,
            Kind::Builder => f.write_str("builder error")?,
            Kind::Decode => f.write_str("error decoding response body")?,
            Kind::Download => f.write_str("download error")?,
            Kind::Request => f.write_str("http request error")?,
            Kind::Status(code) => {
                let prefix = if code.is_client_error() {
                    "HTTP status client error"
                } else {
                    debug_assert!(code.is_server_error());
                    "HTTP status server error"
                };
                write!(f, "{} ({})", prefix, code)?;
            }
            Kind::RateLimit { reset } => {
                write!(f, "API rate limit reached. Try again in {:?}.", reset)?;
            }
            Kind::Validation(ref message, ref errors) => {
                write!(f, "validation failed: '{}' {:?}", message, errors)?;
            }
        };
        if let Some(ref e) = self.inner.source {
            write!(f, ": {}", e)?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    Auth,
    Download,
    Validation(String, HashMap<String, String>),
    RateLimit { reset: Duration },
    Builder,
    Request,
    Decode,
    Status(StatusCode),
}

impl StdError for ModioError {}

impl fmt::Display for ModioError {
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

pub(crate) fn token_required() -> Error {
    Error::new(Kind::Auth, Some(AuthError::TokenRequired))
}

pub(crate) fn unauthorized(error_ref: u16) -> Error {
    Error::new_error_ref(Kind::Auth, error_ref, Some(AuthError::Unauthorized))
}

pub(crate) fn builder_or_request(e: reqwest::Error) -> Error {
    if e.is_builder() {
        builder(e)
    } else {
        request(e)
    }
}

pub(crate) fn builder<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Builder, Some(e))
}

pub(crate) fn request<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Request, Some(e))
}

pub(crate) fn decode<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Decode, Some(e))
}

pub(crate) fn error_for_status(status: StatusCode, error: ModioError) -> Error {
    match status {
        StatusCode::UNPROCESSABLE_ENTITY => Error::new_error_ref(
            Kind::Validation(error.message, error.errors.unwrap_or_default()),
            error.error_ref,
            None::<Error>,
        ),
        StatusCode::UNAUTHORIZED => unauthorized(error.error_ref),
        _ => Error::new_error_ref(Kind::Status(status), error.error_ref, Some(error)),
    }
}

pub(crate) fn ratelimit(reset: u64) -> Error {
    Error::new(
        Kind::RateLimit {
            reset: Duration::from_secs(reset * 60),
        },
        None::<Error>,
    )
}

pub(crate) fn download_mod_not_found(game_id: u32, mod_id: u32) -> Error {
    Error::new(
        Kind::Download,
        Some(DownloadError::ModNotFound { game_id, mod_id }),
    )
}

pub(crate) fn download_no_primary(game_id: u32, mod_id: u32) -> Error {
    Error::new(
        Kind::Download,
        Some(DownloadError::NoPrimaryFile { game_id, mod_id }),
    )
}

pub(crate) fn download_file_not_found(game_id: u32, mod_id: u32, file_id: u32) -> Error {
    Error::new(
        Kind::Download,
        Some(DownloadError::FileNotFound {
            game_id,
            mod_id,
            file_id,
        }),
    )
}

pub(crate) fn download_multiple_files<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    Error::new(
        Kind::Download,
        Some(DownloadError::MultipleFilesFound {
            game_id,
            mod_id,
            version: version.into(),
        }),
    )
}

pub(crate) fn download_version_not_found<S>(game_id: u32, mod_id: u32, version: S) -> Error
where
    S: Into<String>,
{
    Error::new(
        Kind::Download,
        Some(DownloadError::VersionNotFound {
            game_id,
            mod_id,
            version: version.into(),
        }),
    )
}
