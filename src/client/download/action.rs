use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};
use crate::types::mods::Mod;

#[derive(Debug)]
pub enum DownloadAction {
    /// Download the primary modfile of a mod.
    Primary { game_id: GameId, mod_id: ModId },
    /// Download a specific modfile of a mod.
    File {
        game_id: GameId,
        mod_id: ModId,
        file_id: FileId,
    },
    /// Download a specific modfile.
    FileObj(Box<File>),
    /// Download a specific version of a mod.
    Version {
        game_id: GameId,
        mod_id: ModId,
        version: String,
        policy: ResolvePolicy,
    },
}

/// Defines the policy for `DownloadAction::Version` when multiple files are found.
#[derive(Debug)]
pub enum ResolvePolicy {
    /// Download the latest file.
    Latest,
    /// Return with [`Error::MultipleFilesFound`] as source error.
    ///
    /// [`Error::MultipleFilesFound`]: super::Error
    Fail,
}

/// Convert `Mod` to [`DownloadAction::File`] or [`DownloadAction::Primary`] if `Mod::modfile` is `None`
impl From<Mod> for DownloadAction {
    fn from(m: Mod) -> DownloadAction {
        if let Some(file) = m.modfile {
            DownloadAction::from(file)
        } else {
            DownloadAction::Primary {
                game_id: m.game_id,
                mod_id: m.id,
            }
        }
    }
}

/// Convert `File` to [`DownloadAction::FileObj`]
impl From<File> for DownloadAction {
    fn from(file: File) -> DownloadAction {
        DownloadAction::FileObj(Box::new(file))
    }
}

/// Convert `(GameId, ModId)` to [`DownloadAction::Primary`]
impl From<(GameId, ModId)> for DownloadAction {
    fn from((game_id, mod_id): (GameId, ModId)) -> DownloadAction {
        DownloadAction::Primary { game_id, mod_id }
    }
}

/// Convert `(GameId, ModId, FileId)` to [`DownloadAction::File`]
impl From<(GameId, ModId, FileId)> for DownloadAction {
    fn from((game_id, mod_id, file_id): (GameId, ModId, FileId)) -> DownloadAction {
        DownloadAction::File {
            game_id,
            mod_id,
            file_id,
        }
    }
}

/// Convert `(GameId, ModId, String)` to [`DownloadAction::Version`] with resolve policy
/// set to `ResolvePolicy::Latest`
impl From<(GameId, ModId, String)> for DownloadAction {
    fn from((game_id, mod_id, version): (GameId, ModId, String)) -> DownloadAction {
        DownloadAction::Version {
            game_id,
            mod_id,
            version,
            policy: ResolvePolicy::Latest,
        }
    }
}

/// Convert `(GameId, ModId, &'a str)` to [`DownloadAction::Version`] with resolve policy
/// set to `ResolvePolicy::Latest`
impl<'a> From<(GameId, ModId, &'a str)> for DownloadAction {
    fn from((game_id, mod_id, version): (GameId, ModId, &'a str)) -> DownloadAction {
        DownloadAction::Version {
            game_id,
            mod_id,
            version: version.to_string(),
            policy: ResolvePolicy::Latest,
        }
    }
}
