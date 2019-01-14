use url::Url;

use crate::types::mods::{File, Mod};

/// Defines the action that is performed for [`Modio::download`](struct.Modio.html#method.download).
#[derive(Debug)]
pub enum DownloadAction {
    /// Download the primary modfile of a mod.
    Primary { game_id: u32, mod_id: u32 },
    /// Download a specific modfile of a mod.
    File {
        game_id: u32,
        mod_id: u32,
        file_id: u32,
    },
    /// Download a specific version of a mod.
    Version {
        game_id: u32,
        mod_id: u32,
        version: String,
        policy: ResolvePolicy,
    },
    /// Url to download.
    Url(Url),
}

/// Defines the policy for `DownloadAction::Version` when multiple files are found.
#[derive(Debug)]
pub enum ResolvePolicy {
    /// Download the latest file.
    Latest,
    /// Return with `Error::Download(DownloadError::MultipleFilesFound)`.
    Fail,
}

/// Convert `Mod` to [`DownloadAction::Url`](enum.DownloadAction.html#variant.Url) or
/// [`DownloadAction::Primary`](enum.DownloadAction.html#variant.Primary) if `Mod::modfile` is `None`
impl From<Mod> for DownloadAction {
    fn from(m: Mod) -> DownloadAction {
        if let Some(file) = m.modfile {
            DownloadAction::Url(file.download.binary_url)
        } else {
            DownloadAction::Primary {
                game_id: m.game_id,
                mod_id: m.id,
            }
        }
    }
}

/// Convert `File` to [`DownloadAction::Url`](enum.DownloadAction.html#variant.Url)
impl From<File> for DownloadAction {
    fn from(file: File) -> DownloadAction {
        DownloadAction::Url(file.download.binary_url)
    }
}

/// Convert `(u32, u32)` to [`DownloadAction::Primary`](enum.DownloadAction.html#variant.Primary)
impl From<(u32, u32)> for DownloadAction {
    fn from(val: (u32, u32)) -> DownloadAction {
        DownloadAction::Primary {
            game_id: val.0,
            mod_id: val.1,
        }
    }
}

/// Convert `(u32, u32, u32)` to [`DownloadAction::File`](enum.DownloadAction.html#variant.File)
impl From<(u32, u32, u32)> for DownloadAction {
    fn from(val: (u32, u32, u32)) -> DownloadAction {
        DownloadAction::File {
            game_id: val.0,
            mod_id: val.1,
            file_id: val.2,
        }
    }
}

/// Convert `(u32, u32, String)` to [`DownloadAction::Version`](enum.DownloadAction.html#variant.Version)
/// with resolve policy set to `ResolvePolicy::Latest`
impl From<(u32, u32, String)> for DownloadAction {
    fn from(val: (u32, u32, String)) -> DownloadAction {
        DownloadAction::Version {
            game_id: val.0,
            mod_id: val.1,
            version: val.2,
            policy: ResolvePolicy::Latest,
        }
    }
}

/// Convert `(u32, u32, &'a str)` to [`DownloadAction::Version`](enum.DownloadAction.html#variant.Version)
/// with resolve policy set to `ResolvePolicy::Latest`
impl<'a> From<(u32, u32, &'a str)> for DownloadAction {
    fn from(val: (u32, u32, &'a str)) -> DownloadAction {
        DownloadAction::Version {
            game_id: val.0,
            mod_id: val.1,
            version: val.2.to_string(),
            policy: ResolvePolicy::Latest,
        }
    }
}

impl From<Url> for DownloadAction {
    fn from(url: Url) -> DownloadAction {
        DownloadAction::Url(url)
    }
}
