use std::sync::Arc;

use reqwest::Client;

use crate::auth::{Auth, Credentials, Token};
use crate::download::{DownloadAction, Downloader};
use crate::error::Result;
use crate::games::{GameRef, Games};
use crate::mods::ModRef;
use crate::reports::Reports;
use crate::request::RequestBuilder;
use crate::routing::Route;
use crate::types::id::{GameId, ModId};
use crate::user::Me;

mod builder;

pub use builder::Builder;

const DEFAULT_HOST: &str = "https://api.mod.io/v1";
const TEST_HOST: &str = "https://api.test.mod.io/v1";
const DEFAULT_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

/// Endpoint interface to interacting with the [mod.io](https://mod.io) API.
#[derive(Clone, Debug)]
pub struct Modio {
    pub(crate) inner: Arc<ClientRef>,
}

#[derive(Debug)]
pub(crate) struct ClientRef {
    pub(crate) host: String,
    pub(crate) client: Client,
    pub(crate) credentials: Credentials,
}

impl Modio {
    /// Constructs a new `Builder` to configure a `Modio` client.
    ///
    /// This is the same as `Builder::new(credentials)`.
    pub fn builder<C: Into<Credentials>>(credentials: C) -> Builder {
        Builder::new(credentials)
    }

    /// Create an endpoint to [https://api.mod.io/v1](https://docs.mod.io/restapiref/#mod-io-api-v1).
    pub fn new<C>(credentials: C) -> Result<Self>
    where
        C: Into<Credentials>,
    {
        Builder::new(credentials).build()
    }

    /// Create an endpoint to a different host.
    pub fn host<H, C>(host: H, credentials: C) -> Result<Self>
    where
        H: Into<String>,
        C: Into<Credentials>,
    {
        Builder::new(credentials).host(host).build()
    }

    /// Return an endpoint with new credentials.
    #[must_use]
    pub fn with_credentials<CR>(&self, credentials: CR) -> Self
    where
        CR: Into<Credentials>,
    {
        Self {
            inner: Arc::new(ClientRef {
                host: self.inner.host.clone(),
                client: self.inner.client.clone(),
                credentials: credentials.into(),
            }),
        }
    }

    /// Return an endpoint with a new token.
    #[must_use]
    pub fn with_token<T>(&self, token: T) -> Self
    where
        T: Into<Token>,
    {
        Self {
            inner: Arc::new(ClientRef {
                host: self.inner.host.clone(),
                client: self.inner.client.clone(),
                credentials: Credentials {
                    api_key: self.inner.credentials.api_key.clone(),
                    token: Some(token.into()),
                },
            }),
        }
    }

    /// Return a reference to an interface for requesting access tokens.
    pub fn auth(&self) -> Auth {
        Auth::new(self.clone())
    }

    /// Return a reference to an interface that provides access to game information.
    pub fn games(&self) -> Games {
        Games::new(self.clone())
    }

    /// Return a reference to a game.
    pub fn game(&self, game_id: GameId) -> GameRef {
        GameRef::new(self.clone(), game_id)
    }

    /// Return a reference to a mod.
    pub fn mod_(&self, game_id: GameId, mod_id: ModId) -> ModRef {
        ModRef::new(self.clone(), game_id, mod_id)
    }

    /// Returns [`Downloader`] for saving to file or retrieving
    /// the data via [`Stream`].
    ///
    /// The download fails with [`modio::download::Error`] as source
    /// if a primary file, a specific file or a specific version is not found.
    ///
    /// [`Downloader`]: crate::download::Downloader
    /// [`modio::download::Error`]: crate::download::Error
    /// [`Stream`]: futures_util::Stream
    ///
    /// # Example
    /// ```no_run
    /// use futures_util::{future, TryStreamExt};
    /// use modio::download::{DownloadAction, ResolvePolicy};
    /// use modio::types::id::Id;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let modio = modio::Modio::new("user-or-game-api-key")?;
    ///
    /// // Download the primary file of a mod.
    /// let action = DownloadAction::Primary {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    /// };
    /// modio
    ///     .download(action)
    ///     .await?
    ///     .save_to_file("mod.zip")
    ///     .await?;
    ///
    /// // Download the specific file of a mod.
    /// let action = DownloadAction::File {
    ///     game_id: Id::new(5),
    ///     mod_id: Id::new(19),
    ///     file_id: Id::new(101),
    /// };
    /// modio
    ///     .download(action)
    ///     .await?
    ///     .save_to_file("mod.zip")
    ///     .await?;
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
    /// modio
    ///     .download(action)
    ///     .await?
    ///     .stream()
    ///     .try_for_each(|bytes| {
    ///         println!("Bytes: {:?}", bytes);
    ///         future::ok(())
    ///     })
    ///     .await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn download<A>(&self, action: A) -> Result<Downloader>
    where
        DownloadAction: From<A>,
    {
        Downloader::new(self.clone(), action.into()).await
    }

    /// Return a reference to an interface that provides access to resources owned by the user
    /// associated with the current authentication credentials.
    pub fn user(&self) -> Me {
        Me::new(self.clone())
    }

    /// Return a reference to an interface to report games, mods and users.
    pub fn reports(&self) -> Reports {
        Reports::new(self.clone())
    }

    pub(crate) fn request(&self, route: Route) -> RequestBuilder {
        RequestBuilder::new(self.clone(), route)
    }
}
