//! Downloading mod files.
use std::error::Error as StdError;
use std::fmt;
use std::path::Path;

use bytes::Bytes;
use futures_util::{SinkExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use reqwest::{Method, Response, StatusCode};
use tokio::fs::File as AsyncFile;
use tokio::io::BufWriter;
use tokio_util::codec::{BytesCodec, FramedWrite};
use tracing::debug;

use crate::error::{self, Result};
use crate::types::files::File;
use crate::types::id::{FileId, GameId, ModId};
use crate::types::mods::Mod;
use crate::Modio;

/// A `Downloader` can be used to stream a mod file or save the file to a local file.
/// Constructed with [`Modio::download`].
pub struct Downloader(Response);

impl Downloader {
    pub(crate) async fn new(modio: Modio, action: DownloadAction) -> Result<Self> {
        Ok(Self(request_file(modio, action).await?))
    }

    /// Save the mod file to a local file.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Modio::new("api-key")?;
    /// let action = modio::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// modio
    ///     .download(action)
    ///     .await?
    ///     .save_to_file("mod.zip")
    ///     .await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn save_to_file<P: AsRef<Path>>(self, file: P) -> Result<()> {
        let out = AsyncFile::create(file).map_err(error::decode).await?;
        let out = BufWriter::with_capacity(512 * 512, out);
        let out = FramedWrite::new(out, BytesCodec::new());
        let out = SinkExt::<Bytes>::sink_map_err(out, error::decode);
        self.stream().forward(out).await
    }

    /// Get the full mod file as `Bytes`.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Modio::new("api-key")?;
    /// let action = modio::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// let bytes = modio.download(action).await?.bytes().await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn bytes(self) -> Result<Bytes> {
        self.0.bytes().map_err(error::request).await
    }

    /// `Stream` of bytes of the mod file.
    ///
    /// # Example
    /// ```no_run
    /// use futures_util::TryStreamExt;
    ///
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Modio::new("api-key")?;
    /// let action = modio::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// let mut st = Box::pin(modio.download(action).await?.stream());
    /// while let Some(bytes) = st.try_next().await? {
    ///     println!("Bytes: {:?}", bytes);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn stream(self) -> impl Stream<Item = Result<Bytes>> {
        self.0.bytes_stream().map_err(error::request)
    }

    /// Get the content length from the mod file response.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Modio::new("api-key")?;
    /// let action = modio::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// let content_length = modio
    ///     .download(action)
    ///     .await?
    ///     .content_length()
    ///     .expect("mod file response should have content length");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn content_length(&self) -> Option<u64> {
        self.0.content_length()
    }
}

async fn request_file(modio: Modio, action: DownloadAction) -> Result<Response> {
    let url = match action {
        DownloadAction::Primary { game_id, mod_id } => {
            let modref = modio.mod_(game_id, mod_id);
            let m = modref
                .get()
                .map_err(|e| match e.status() {
                    Some(StatusCode::NOT_FOUND) => {
                        let source = Error::ModNotFound { game_id, mod_id };
                        error::download(source)
                    }
                    _ => e,
                })
                .await?;
            if let Some(file) = m.modfile {
                file.download.binary_url
            } else {
                let source = Error::NoPrimaryFile { game_id, mod_id };
                return Err(error::download(source));
            }
        }
        DownloadAction::FileObj(file) => file.download.binary_url,
        DownloadAction::File {
            game_id,
            mod_id,
            file_id,
        } => {
            let fileref = modio.mod_(game_id, mod_id).file(file_id);
            let file = fileref
                .get()
                .map_err(|e| match e.status() {
                    Some(StatusCode::NOT_FOUND) => {
                        let source = Error::FileNotFound {
                            game_id,
                            mod_id,
                            file_id,
                        };
                        error::download(source)
                    }
                    _ => e,
                })
                .await?;
            file.download.binary_url
        }
        DownloadAction::Version {
            game_id,
            mod_id,
            version,
            policy,
        } => {
            use crate::files::filters::Version;
            use crate::filter::prelude::*;
            use ResolvePolicy::*;

            let filter = Version::eq(version.clone())
                .order_by(DateAdded::desc())
                .limit(2);

            let files = modio.mod_(game_id, mod_id).files();
            let mut list = files
                .search(filter)
                .first_page()
                .map_err(|e| match e.status() {
                    Some(StatusCode::NOT_FOUND) => {
                        let source = Error::ModNotFound { game_id, mod_id };
                        error::download(source)
                    }
                    _ => e,
                })
                .await?;

            let (file, error) = match (list.len(), policy) {
                (0, _) => (
                    None,
                    Some(Error::VersionNotFound {
                        game_id,
                        mod_id,
                        version,
                    }),
                ),
                (1, _) | (_, Latest) => (Some(list.remove(0)), None),
                (_, Fail) => (
                    None,
                    Some(Error::MultipleFilesFound {
                        game_id,
                        mod_id,
                        version,
                    }),
                ),
            };

            if let Some(file) = file {
                file.download.binary_url
            } else {
                let source = error.expect("bug in previous match!");
                return Err(error::download(source));
            }
        }
    };

    debug!("downloading file: {}", url);
    modio
        .inner
        .client
        .request(Method::GET, url)
        .send()
        .map_err(error::builder_or_request)
        .await?
        .error_for_status()
        .map_err(error::request)
}

/// Defines the action that is performed for [`Modio::download`].
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
    Fail,
}

/// The Errors that may occur when using [`Modio::download`].
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

impl StdError for Error {}

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
