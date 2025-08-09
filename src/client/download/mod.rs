//! Download interface for mod files.

use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

use bytes::Bytes;
use futures_util::StreamExt;
use http::HeaderMap;
use http_body_util::{BodyDataStream, BodyExt};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

mod action;
mod error;
mod info;

pub use action::{DownloadAction, ResolvePolicy};
pub use error::Error;
pub use info::Info;

use crate::client::service::{Body, Response};
use crate::request::body::Body as RequestBody;
use crate::Client;

impl Client {
    /// Returns [`Downloader`] for saving to file or retrieving the data chunked as `Bytes`.
    ///
    /// The download fails with [`modio::client::download::Error`] as source if a primary file, a
    /// specific file or a specific version is not found.
    ///
    /// [`Downloader`]: crate::client::download::Downloader
    /// [`modio::client::download::Error`]: crate::client::download::Error
    ///
    /// # Example
    ///
    /// ```no_run
    /// use modio::client::download::{DownloadAction, ResolvePolicy};
    /// use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let modio = modio::Client::builder("user-or-game-api-key".to_owned()).build()?;
    ///
    /// // Download the primary file of a mod.
    /// let action = DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    /// modio.download(action).save_to_file("mod.zip").await?;
    ///
    /// // Download the specific file of a mod.
    /// let action = DownloadAction::File {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    ///     file_id: Id::new(101),
    /// };
    /// modio.download(action).save_to_file("mod.zip").await?;
    ///
    /// // Download the specific version of a mod.
    /// // if multiple files are found then the latest file is downloaded.
    /// // Set policy to `ResolvePolicy::Fail` to return with
    /// // `modio::download::Error::MultipleFilesFound` as source error.
    /// let action = DownloadAction::Version {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    ///     version: "0.1".to_string(),
    ///     policy: ResolvePolicy::Latest,
    /// };
    /// let mut chunked = modio.download(action).chunked().await?;
    ///
    /// while let Some(chunk) = chunked.data().await {
    ///     println!("Bytes: {:?}", chunk?);
    /// }
    /// #    Ok(())
    /// # }
    /// ```
    pub fn download<A>(&self, action: A) -> Downloader<'_, Init<'_>>
    where
        DownloadAction: From<A>,
    {
        Downloader::<Init<'_>>::new(self, action.into())
    }
}

/// A `Downloader` can be used to stream a mod file or save the file to a local file.
/// Constructed with [`Client::download`].
pub struct Downloader<'a, State> {
    state: State,
    phantom: PhantomData<fn(&'a State) -> State>,
}

impl<T> Downloader<'_, T> {
    pub(crate) fn new(http: &Client, action: DownloadAction) -> Downloader<'_, Init<'_>> {
        Downloader {
            state: Init { http, action },
            phantom: PhantomData,
        }
    }
}

/// Downloader state where the caller must choose how the file is downloaded.
#[doc(hidden)]
pub struct Init<'a> {
    http: &'a Client,
    action: DownloadAction,
}

impl<'a> Downloader<'a, Init<'a>> {
    /// Retrieve the mod file in chunks of `Bytes`.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Client::builder("api-key".to_owned()).build()?;
    /// let action = modio::client::download::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// let mut chunked = modio.download(action).chunked().await?;
    /// while let Some(bytes) = chunked.data().await {
    ///     println!("Bytes: {:?}", bytes);
    /// }
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn chunked(self) -> Result<Downloader<'a, Chunked>, crate::error::Error> {
        let Init { http, action } = self.state;
        let info = info::download_info(http, action).await?;

        let req = http::Request::get(info.download_url.as_str())
            .body(RequestBody::empty())
            .map_err(crate::error::download)?;

        let response = http.raw_request(req).await?;

        Ok(Downloader {
            state: Chunked::new(info, response),
            phantom: PhantomData,
        })
    }

    /// Save the mod file to a local file.
    ///
    /// # Example
    /// ```no_run
    /// # use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #     let modio = modio::Client::builder("api-key".to_owned()).build()?;
    /// let action = modio::client::download::DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    ///
    /// modio.download(action).save_to_file("mod.zip").await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn save_to_file(self, path: impl AsRef<Path>) -> Result<(), crate::error::Error> {
        let mut chunked = self.chunked().await?;

        let out = File::create(path).await.map_err(crate::error::download)?;
        let mut out = BufWriter::with_capacity(512 * 512, out);

        while let Some(chunk) = chunked.data().await {
            out.write_all(&chunk?)
                .await
                .map_err(crate::error::download)?;
        }
        Ok(())
    }
}

/// Downloader state where the caller
#[doc(hidden)]
pub struct Chunked {
    info: Info,
    headers: HeaderMap,
    body: BodyDataStream<Body>,
}

impl Chunked {
    fn new(info: Info, response: Response) -> Self {
        let (parts, body) = response.into_parts();
        let headers = parts.headers;
        let body = body.into_data_stream();

        Self {
            info,
            headers,
            body,
        }
    }
}

impl Downloader<'_, Chunked> {
    pub fn info(&self) -> &Info {
        &self.state.info
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.state.headers
    }

    pub async fn data(&mut self) -> Option<Result<Bytes, crate::error::Error>> {
        let chunk = self.state.body.next().await;
        chunk.map(|c| c.map_err(crate::error::request))
    }
}

impl<'a> fmt::Debug for Downloader<'a, Init<'a>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Downloader")
            .field("action", &self.state.action)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for Downloader<'_, Chunked> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Downloader")
            .field("info", &self.state.info)
            .finish_non_exhaustive()
    }
}
