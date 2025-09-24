use std::fmt;

use crate::types::id::{FileId, GameId, ModId};

type Source = Box<dyn std::error::Error + Send + Sync>;

/// The Errors that may occur when using [`Download::download`].
///
/// [`Download::download`]: crate::util::Download::download
pub struct Error {
    kind: ErrorKind,
    source: Option<Source>,
}

impl Error {
    #[inline]
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, source: None }
    }

    #[inline]
    pub(crate) fn with<E: Into<Source>>(mut self, source: E) -> Self {
        self.source = Some(source.into());
        self
    }

    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Error {
    #[inline]
    pub(crate) fn request<E: Into<Source>>(source: E) -> Self {
        Self::new(ErrorKind::Request).with(source)
    }

    #[inline]
    pub(crate) fn body<E: Into<Source>>(source: E) -> Self {
        Self::new(ErrorKind::Body).with(source)
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Request => fmt.write_str("request failed"),
            ErrorKind::Body => fmt.write_str("failed to load response body"),
            ErrorKind::Io => fmt.write_str("IO error"),
            ErrorKind::ModNotFound { game_id, mod_id } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}} not found.",
            ),
            ErrorKind::FileNotFound {
                game_id,
                mod_id,
                file_id,
            } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}}: File {{ id: {file_id} }} not found.",
            ),
            ErrorKind::MultipleFilesFound {
                game_id,
                mod_id,
                version,
            } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}}: Multiple files found for version '{version}'.",
            ),
            ErrorKind::NoPrimaryFile { game_id, mod_id } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}} Mod has no primary file.",
            ),
            ErrorKind::VersionNotFound {
                game_id,
                mod_id,
                version,
            } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}}: No file with version '{version}' found.",
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Request,
    Body,
    Io,
    /// The mod has not found.
    ModNotFound {
        game_id: GameId,
        mod_id: ModId,
    },
    /// The mod has no primary file.
    NoPrimaryFile {
        game_id: GameId,
        mod_id: ModId,
    },
    /// The specific file of a mod was not found.
    FileNotFound {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    /// Multiple files for a given version were found and the policy was set to
    /// [`ResolvePolicy::Fail`].
    ///
    /// [`ResolvePolicy::Fail`]: super::ResolvePolicy::Fail
    MultipleFilesFound {
        game_id: GameId,
        mod_id: ModId,
        version: String,
    },
    /// No file for a given version was found.
    VersionNotFound {
        game_id: GameId,
        mod_id: ModId,
        version: String,
    },
}
