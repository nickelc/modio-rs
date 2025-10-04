//! Client errors
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

use reqwest::StatusCode;

use crate::types::Error as ApiError;

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
    #[inline]
    pub(crate) fn new(kind: Kind) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                error_ref: None,
                source: None,
            }),
        }
    }

    #[inline]
    pub(crate) fn with<E: Into<BoxError>>(mut self, source: E) -> Self {
        self.inner.source = Some(source.into());
        self
    }

    #[inline]
    pub(crate) fn with_error_ref(mut self, error_ref: u16) -> Self {
        self.inner.error_ref = Some(error_ref);
        self
    }

    /// Returns true if the API key/access token is incorrect, revoked, expired or the request
    /// needs a different authentication method.
    pub fn is_auth(&self) -> bool {
        matches!(self.inner.kind, Kind::Unauthorized | Kind::TokenRequired)
    }

    /// Returns true if the acceptance of the Terms of Use is required before continuing external
    /// authorization.
    pub fn is_terms_acceptance_required(&self) -> bool {
        matches!(self.inner.kind, Kind::TermsAcceptanceRequired)
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
    pub fn is_response(&self) -> bool {
        matches!(self.inner.kind, Kind::Response { .. })
    }

    /// Returns true if the error contains validation errors.
    pub fn is_validation(&self) -> bool {
        matches!(self.inner.kind, Kind::Validation { .. })
    }

    /// Returns true if the error is related to serialization.
    pub fn is_decode(&self) -> bool {
        matches!(self.inner.kind, Kind::Decode)
    }

    /// Returns the API error if the error was generated from a response.
    pub fn api_error(&self) -> Option<&ApiError> {
        match &self.inner.kind {
            Kind::Response { error, .. } => Some(error),
            _ => None,
        }
    }

    /// Returns modio's error reference code.
    ///
    /// See the [Error Codes](https://docs.mod.io/restapiref/#error-codes) docs for more information.
    pub fn error_ref(&self) -> Option<u16> {
        self.inner.error_ref
    }

    /// Returns status code if the error was generated from a response.
    pub fn status(&self) -> Option<StatusCode> {
        match self.inner.kind {
            Kind::Response { status, .. } => Some(status),
            _ => None,
        }
    }

    /// Returns validation message & errors from the response.
    pub fn validation(&self) -> Option<(&String, &Vec<(String, String)>)> {
        match self.inner.kind {
            Kind::Validation {
                ref message,
                ref errors,
            } => Some((message, errors)),
            _ => None,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("modio::Error");

        builder.field("kind", &self.inner.kind);
        if let Some(ref error_ref) = self.inner.error_ref {
            builder.field("error_ref", error_ref);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }
        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.kind {
            Kind::Unauthorized => f.write_str("unauthorized")?,
            Kind::TokenRequired => f.write_str("access token is required")?,
            Kind::TermsAcceptanceRequired => f.write_str("terms acceptance is required")?,
            Kind::Builder => f.write_str("builder error")?,
            Kind::Decode => f.write_str("error decoding response body")?,
            Kind::Download => f.write_str("download error")?,
            Kind::Request => f.write_str("http request error")?,
            Kind::Response { status, .. } => {
                let prefix = if status.is_client_error() {
                    "HTTP status client error"
                } else {
                    debug_assert!(status.is_server_error());
                    "HTTP status server error"
                };
                write!(f, "{prefix} ({status})")?;
            }
            Kind::RateLimit { retry_after } => {
                write!(f, "API rate limit reached. Try again in {retry_after:?}.")?;
            }
            Kind::Validation {
                ref message,
                ref errors,
            } => {
                write!(f, "validation failed: '{message}' {errors:?}")?;
            }
        }
        if let Some(ref e) = self.inner.source {
            write!(f, ": {e}")?;
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
    /// API key/access token is incorrect, revoked or expired.
    Unauthorized,
    /// Access token is required to perform the action.
    TokenRequired,
    /// The acceptance of the Terms of Use is required.
    TermsAcceptanceRequired,
    Download,
    Validation {
        message: String,
        errors: Vec<(String, String)>,
    },
    RateLimit {
        retry_after: Duration,
    },
    Builder,
    Request,
    Response {
        status: StatusCode,
        error: ApiError,
    },
    Decode,
}

pub(crate) fn token_required() -> Error {
    Error::new(Kind::TokenRequired)
}

pub(crate) fn builder_or_request(e: reqwest::Error) -> Error {
    if e.is_builder() {
        builder(e)
    } else {
        request(e)
    }
}

pub(crate) fn builder<E: Into<BoxError>>(source: E) -> Error {
    Error::new(Kind::Builder).with(source)
}

pub(crate) fn request<E: Into<BoxError>>(source: E) -> Error {
    Error::new(Kind::Request).with(source)
}

pub(crate) fn decode<E: Into<BoxError>>(source: E) -> Error {
    Error::new(Kind::Decode).with(source)
}

pub(crate) fn error_for_status(status: StatusCode, error: ApiError) -> Error {
    let error_ref = error.error_ref;
    let kind = match status {
        StatusCode::UNPROCESSABLE_ENTITY => Kind::Validation {
            message: error.message,
            errors: error.errors,
        },
        StatusCode::UNAUTHORIZED => Kind::Unauthorized,
        StatusCode::FORBIDDEN if error_ref == 11051 => Kind::TermsAcceptanceRequired,
        _ => Kind::Response { status, error },
    };
    Error::new(kind).with_error_ref(error_ref)
}

pub(crate) fn ratelimit(retry_after: u64) -> Error {
    Error::new(Kind::RateLimit {
        retry_after: Duration::from_secs(retry_after),
    })
}

pub(crate) fn download<E: Into<BoxError>>(source: E) -> Error {
    Error::new(Kind::Download).with(source)
}
