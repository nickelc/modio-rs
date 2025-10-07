use std::fmt;

type Source = Box<dyn std::error::Error + Send + Sync>;

pub struct Error {
    kind: ErrorKind,
    source: Option<Source>,
}

impl Error {
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Error {
    #[inline]
    pub(crate) fn request<E: Into<Source>>(source: E) -> Self {
        Self {
            kind: ErrorKind::Request,
            source: Some(source.into()),
        }
    }

    #[inline]
    pub(crate) fn body<E: Into<Source>>(source: E) -> Self {
        Self {
            kind: ErrorKind::Body,
            source: Some(source.into()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Error");

        s.field("kind", &self.kind);
        if let Some(ref source) = self.source {
            s.field("source", source);
        }
        s.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Request => f.write_str("request failed"),
            ErrorKind::Body => f.write_str("failed to load response body"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Request,
    Body,
}
