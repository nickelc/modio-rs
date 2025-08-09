use std::fmt;
use std::str::Utf8Error;

use hyper::body::Bytes;

type Source = Box<dyn std::error::Error + Send + Sync>;

/// Failure when processing a response body.
pub struct BodyError {
    inner: Box<ErrorImpl>,
}

impl BodyError {
    #[inline]
    pub(super) fn new(kind: BodyErrorKind, source: Option<Source>) -> Self {
        Self {
            inner: Box::new(ErrorImpl { kind, source }),
        }
    }

    pub(crate) fn decode(bytes: Bytes, source: serde_json::Error) -> Self {
        Self::new(BodyErrorKind::Decode { bytes }, Some(Box::new(source)))
    }

    pub(crate) fn invalid_utf8(bytes: Vec<u8>, source: Utf8Error) -> Self {
        Self::new(BodyErrorKind::InvalidUtf8 { bytes }, Some(Box::new(source)))
    }

    /// Returns true if the body could not be loaded.
    pub fn is_loading(&self) -> bool {
        matches!(self.inner.kind, BodyErrorKind::Loading)
    }

    /// Returns true if the response body could not be deserialized.
    pub fn is_decode(&self) -> bool {
        matches!(self.inner.kind, BodyErrorKind::Decode { .. })
    }

    /// Returns true if the response body contains invalid utf8 data.
    pub fn is_invalid_utf8(&self) -> bool {
        matches!(self.inner.kind, BodyErrorKind::InvalidUtf8 { .. })
    }

    /// Returns a slice of `u8`s bytes that were attempted to deserialize or convert to a `String`.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match &self.inner.kind {
            BodyErrorKind::Decode { bytes } => Some(bytes),
            BodyErrorKind::InvalidUtf8 { bytes } => Some(bytes),
            _ => None,
        }
    }
}

impl fmt::Debug for BodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("BodyError");
        f.field("kind", &self.inner.kind);
        if let Some(ref source) = self.inner.source {
            f.field("source", source);
        }
        f.finish()
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.kind {
            BodyErrorKind::Loading => f.write_str("failed to retrieve response body"),
            BodyErrorKind::Decode { .. } => f.write_str("failed to decode response body"),
            BodyErrorKind::InvalidUtf8 { .. } => {
                f.write_str("response body is not a valid utf8 string")
            }
        }
    }
}

impl std::error::Error for BodyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

struct ErrorImpl {
    kind: BodyErrorKind,
    source: Option<Source>,
}

#[derive(Debug)]
#[non_exhaustive]
pub(super) enum BodyErrorKind {
    Loading,
    Decode { bytes: Bytes },
    InvalidUtf8 { bytes: Vec<u8> },
}
