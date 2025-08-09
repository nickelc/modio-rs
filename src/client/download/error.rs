use std::fmt;

use crate::types::id::{FileId, GameId, ModId};

/// The Errors that may occur when using [`Client::download`].
///
/// [`Client::download`]: crate::client::Client::download
#[derive(Debug)]
pub enum Error {
    /// The mod has not found.
    ModNotFound { game_id: GameId, mod_id: ModId },
    /// The mod has no primary file.
    NoPrimaryFile { game_id: GameId, mod_id: ModId },
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

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ModNotFound { game_id, mod_id } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}} not found.",
            ),
            Error::FileNotFound {
                game_id,
                mod_id,
                file_id,
            } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}}: File {{ id: {file_id} }} not found.",
            ),
            Error::MultipleFilesFound {
                game_id,
                mod_id,
                version,
            } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}}: Multiple files found for version '{version}'.",
            ),
            Error::NoPrimaryFile { game_id, mod_id } => write!(
                fmt,
                "Mod {{id: {mod_id}, game_id: {game_id}}} Mod has no primary file.",
            ),
            Error::VersionNotFound {
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
