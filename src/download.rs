use crate::types::mods::{File, Mod};

/// Defines the action that is performed for [`Modio::download`](struct.Modio.html#method.download).
#[derive(Debug)]
pub enum DownloadAction {
    /// Download the primary modfile of a mod.
    Primary { game_id: u32, mod_id: u32 },
    /// Download a specific modfile of a mod.
    FileRef {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    /// Download a specific modfile.
    File(Box<File>),
    /// Download a specific version of a mod.
    Version {
        game_id: u32,
        mod_id: u32,
        version: String,
        policy: ResolvePolicy,
    },
}

/// Defines the policy for `DownloadAction::Version` when multiple files are found.
#[derive(Debug)]
pub enum ResolvePolicy {
    /// Download the latest file.
    Latest,
    /// Return with `ErrorKind::Download(DownloadError::MultipleFilesFound)`.
    Fail,
}

/// Convert `Mod` to [`DownloadAction::File`](enum.DownloadAction.html#variant.File) or
/// [`DownloadAction::Primary`](enum.DownloadAction.html#variant.Primary) if `Mod::modfile` is `None`
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

/// Convert `File` to [`DownloadAction::File`](enum.DownloadAction.html#variant.File)
impl From<File> for DownloadAction {
    fn from(file: File) -> DownloadAction {
        DownloadAction::File(Box::new(file))
    }
}

/// Convert `(u32, u32)` to [`DownloadAction::Primary`](enum.DownloadAction.html#variant.Primary)
impl From<(u32, u32)> for DownloadAction {
    fn from((game_id, mod_id): (u32, u32)) -> DownloadAction {
        DownloadAction::Primary { game_id, mod_id }
    }
}

/// Convert `(u32, u32, u32)` to
/// [`DownloadAction::FileRef`](enum.DownloadAction.html#variant.FileRef)
impl From<(u32, u32, u32)> for DownloadAction {
    fn from((game_id, mod_id, file_id): (u32, u32, u32)) -> DownloadAction {
        DownloadAction::FileRef {
            game_id,
            mod_id,
            file_id,
        }
    }
}

/// Convert `(u32, u32, String)` to [`DownloadAction::Version`](enum.DownloadAction.html#variant.Version)
/// with resolve policy set to `ResolvePolicy::Latest`
impl From<(u32, u32, String)> for DownloadAction {
    fn from((game_id, mod_id, version): (u32, u32, String)) -> DownloadAction {
        DownloadAction::Version {
            game_id,
            mod_id,
            version,
            policy: ResolvePolicy::Latest,
        }
    }
}

/// Convert `(u32, u32, &'a str)` to [`DownloadAction::Version`](enum.DownloadAction.html#variant.Version)
/// with resolve policy set to `ResolvePolicy::Latest`
impl<'a> From<(u32, u32, &'a str)> for DownloadAction {
    fn from((game_id, mod_id, version): (u32, u32, &'a str)) -> DownloadAction {
        DownloadAction::Version {
            game_id,
            mod_id,
            version: version.to_string(),
            policy: ResolvePolicy::Latest,
        }
    }
}
