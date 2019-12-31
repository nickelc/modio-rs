//! Modio provides a set of building blocks for interacting with the [mod.io](https://mod.io) API.
//!
//! The client uses asynchronous I/O, backed by the `futures` and `tokio` crates, and requires both
//! to be used alongside.
//!
//! # Authentication
//!
//! To access the API authentication is required and can be done via several ways:
//!
//! - Request an [API key (Read-only)](https://mod.io/apikey)
//! - Manually create an [OAuth 2 Access Token (Read + Write)](https://mod.io/oauth)
//! - [Email Authentication Flow](auth/struct.Auth.html#example) to create an OAuth 2 Access Token
//! (Read + Write)
//! - [Encrypted gog user auth ticket](auth/struct.Auth.html#method.gog_auth) to create an
//! OAuth 2 Access Token (Read + Write)
//! - [Encrypted steam user auth ticket](auth/struct.Auth.html#method.steam_auth) to create an
//! OAuth 2 Access Token (Read + Write)
//! - [Oculus user data](auth/struct.Auth.html#method.oculus_auth) to create an
//! OAuth 2 Access Token (Read + Write)
//!
//! # Rate Limiting
//!
//! For API requests using API key authentication are **unlimited** and for OAuth 2 authentication
//! requests are limited to **120 requests per hour**.
//!
//! [`Error::is_ratelimited`](struct.Error.html#method.is_ratelimited) will return true
//! if the rate limit associated with credentials has been exhausted.
//!
//! # Example: Basic setup
//!
//! ```no_run
//! use modio::{Credentials, Modio};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let modio = Modio::new(
//!         Credentials::ApiKey(String::from("user-or-game-api-key")),
//!     )?;
//!
//!     // create some tasks and execute them
//!     // let result = task.await?;
//!     Ok(())
//! }
//! ```
//!
//! For testing purposes use [`Modio::host`](struct.Modio.html#method.host) to create a client for the
//! mod.io [test environment](https://docs.mod.io/#testing).
//!
//! # Example: Chaining api requests
//!
//! ```no_run
//! use futures_util::future::try_join3;
//! # use modio::{Credentials, Modio};
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = Modio::new(
//! #        Credentials::ApiKey(String::from("user-or-game-api-key")),
//! #    )?;
//!
//! // OpenXcom: The X-Com Files
//! let modref = modio.mod_(51, 158);
//!
//! // Get mod with its dependencies and all files
//! let deps = modref.dependencies().list();
//! let files = modref.files().list(Default::default());
//! let mod_ = modref.get();
//!
//! let (m, deps, files) = try_join3(mod_, deps, files).await?;
//!
//! println!("{}", m.name);
//! println!(
//!     "deps: {:?}",
//!     deps.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
//! );
//! for file in files {
//!     println!("file id: {} version: {:?}", file.id, file.version);
//! }
//! #    Ok(())
//! # }
//! ```
//!
//! # Example: Downloading mods
//!
//! ```no_run
//! use modio::download::{DownloadAction, ResolvePolicy};
//! # use modio::{Credentials, Modio};
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #    let modio = Modio::new(
//! #        Credentials::ApiKey(String::from("user-or-game-api-key")),
//! #    )?;
//!
//! // Download the primary file of a mod.
//! let action = DownloadAction::Primary {
//!     game_id: 5,
//!     mod_id: 19,
//! };
//! modio.download(action).save_to_file("mod.zip").await?;
//!
//! // Download the specific file of a mod.
//! let action = DownloadAction::File {
//!     game_id: 5,
//!     mod_id: 19,
//!     file_id: 101,
//! };
//! modio.download(action).save_to_file("mod.zip").await?;
//!
//! // Download the specific version of a mod.
//! // if multiple files are found then the latest file is downloaded.
//! // Set policy to `ResolvePolicy::Fail` to return with
//! // `modio::download::Error::MultipleFilesFound` as source error.
//! let action = DownloadAction::Version {
//!     game_id: 5,
//!     mod_id: 19,
//!     version: "0.1".to_string(),
//!     policy: ResolvePolicy::Latest,
//! };
//! modio.download(action).save_to_file("mod.zip").await?;
//! #    Ok(())
//! # }
//! ```
#![doc(html_root_url = "https://docs.rs/modio/0.4.0")]
#![deny(rust_2018_idioms)]

use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, Proxy};
use serde::de::DeserializeOwned;

#[macro_use]
mod macros;

pub mod auth;
#[macro_use]
pub mod filter;
pub mod comments;
pub mod download;
pub mod files;
pub mod games;
pub mod me;
pub mod metadata;
pub mod mods;
pub mod reports;
pub mod teams;
pub mod users;

mod error;
mod iter;
mod multipart;
mod request;
mod routing;
mod types;

use crate::auth::Auth;
use crate::comments::Comments;
use crate::download::Downloader;
use crate::games::{GameRef, Games};
use crate::iter::Iter;
use crate::me::Me;
use crate::mods::{ModRef, Mods};
use crate::reports::Reports;
use crate::request::RequestBuilder;
use crate::types::ModioMessage;
use crate::users::Users;

pub use crate::auth::Credentials;
pub use crate::download::DownloadAction;
pub use crate::error::{Error, Result};
pub use crate::types::{Deletion, Editing, List};

const DEFAULT_HOST: &str = "https://api.mod.io/v1";
const TEST_HOST: &str = "https://api.test.mod.io/v1";
const DEFAULT_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

mod prelude {
    pub use futures_core::Stream;
    pub use futures_util::{TryFutureExt, TryStreamExt};
    pub use reqwest::multipart::{Form, Part};
    pub use reqwest::StatusCode;

    pub use crate::filter::Filter;
    pub use crate::iter::Iter;
    pub use crate::routing::Route;
    pub use crate::Deletion;
    pub use crate::Editing;
    pub use crate::List;
    pub use crate::Modio;
    pub(crate) use crate::ModioMessage;
    pub(crate) use crate::QueryString;
    pub use crate::Result;
}

/// Re-exports of the used reqwest types.
pub mod client {
    pub use reqwest::header;
    pub use reqwest::redirect::Policy;
    pub use reqwest::ClientBuilder;
    #[cfg(feature = "tls")]
    pub use reqwest::{Certificate, Identity};
    pub use reqwest::{Proxy, Url};
}

#[allow(dead_code)]
const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RETRY_AFTER: &str = "x-ratelimit-retryafter";

/// A `Builder` can be used to create a `Modio` client with custom configuration.
pub struct Builder {
    config: Config,
}

struct Config {
    host: Option<String>,
    agent: Option<String>,
    credentials: Credentials,
    builder: Option<ClientBuilder>,
    proxies: Vec<Proxy>,
    #[cfg(feature = "tls")]
    tls: TlsBackend,
}

#[cfg(feature = "tls")]
enum TlsBackend {
    #[cfg(feature = "default-tls")]
    Default,
    #[cfg(feature = "rustls-tls")]
    Rustls,
}

#[cfg(feature = "tls")]
impl Default for TlsBackend {
    fn default() -> TlsBackend {
        #[cfg(feature = "default-tls")]
        {
            TlsBackend::Default
        }
        #[cfg(all(feature = "rustls-tls", not(feature = "default-tls")))]
        {
            TlsBackend::Rustls
        }
    }
}

impl Builder {
    /// Constructs a new `Builder`.
    ///
    /// This is the same as `Modio::builder(credentials)`.
    pub fn new<C: Into<Credentials>>(credentials: C) -> Builder {
        Builder {
            config: Config {
                host: None,
                agent: None,
                credentials: credentials.into(),
                builder: None,
                proxies: Vec::new(),
                #[cfg(feature = "tls")]
                tls: TlsBackend::default(),
            },
        }
    }

    /// Returns a `Modio` client that uses this `Builder` configuration.
    pub fn build(self) -> Result<Modio> {
        let config = self.config;
        let host = config.host.unwrap_or_else(|| DEFAULT_HOST.to_string());
        let credentials = config.credentials;

        let client = {
            let mut builder = {
                let builder = config.builder.unwrap_or_else(Client::builder);
                #[cfg(feature = "tls")]
                match config.tls {
                    #[cfg(feature = "default-tls")]
                    TlsBackend::Default => builder.use_native_tls(),
                    #[cfg(feature = "rustls-tls")]
                    TlsBackend::Rustls => builder.use_rustls_tls(),
                }

                #[cfg(not(feature = "tls"))]
                builder
            };

            let mut headers = HeaderMap::new();
            let agent = match config.agent {
                Some(agent) => HeaderValue::from_str(&agent).map_err(error::builder)?,
                None => HeaderValue::from_static(DEFAULT_AGENT),
            };
            headers.insert(USER_AGENT, agent);

            for proxy in config.proxies {
                builder = builder.proxy(proxy);
            }

            builder
                .default_headers(headers)
                .build()
                .map_err(error::builder)?
        };

        Ok(Modio {
            host,
            credentials,
            client,
        })
    }

    /// Configure the underlying `reqwest` client using `reqwest::ClientBuilder`.
    pub fn client<F>(mut self, f: F) -> Builder
    where
        F: FnOnce(ClientBuilder) -> ClientBuilder,
    {
        self.config.builder = Some(f(Client::builder()));
        self
    }

    /// Set the mod.io api host.
    ///
    /// Defaults to `"https://api.mod.io/v1"`
    pub fn host<S: Into<String>>(mut self, host: S) -> Builder {
        self.config.host = Some(host.into());
        self
    }

    /// Use the mod.io api test host.
    pub fn use_test(mut self) -> Builder {
        self.config.host = Some(TEST_HOST.into());
        self
    }

    /// Set the user agent used for every request.
    ///
    /// Defaults to `"modio/{version}"`
    pub fn agent<S: Into<String>>(mut self, agent: S) -> Builder {
        self.config.agent = Some(agent.into());
        self
    }

    /// Add a `Proxy` to the list of proxies the client will use.
    pub fn proxy(mut self, proxy: Proxy) -> Builder {
        self.config.proxies.push(proxy);
        self
    }

    /// Use native TLS backend.
    #[cfg(feature = "default-tls")]
    pub fn use_default_tls(mut self) -> Builder {
        self.config.tls = TlsBackend::Default;
        self
    }

    /// Use rustls TLS backend.
    #[cfg(feature = "rustls-tls")]
    pub fn use_rustls_tls(mut self) -> Builder {
        self.config.tls = TlsBackend::Rustls;
        self
    }
}

/// Endpoint interface to interacting with the [mod.io](https://mod.io) API.
#[derive(Clone, Debug)]
pub struct Modio {
    host: String,
    client: Client,
    pub(crate) credentials: Credentials,
}

impl Modio {
    /// Constructs a new `Builder` to configure a `Modio` client.
    ///
    /// This is the same as `Builder::new(credentials)`.
    pub fn builder<C: Into<Credentials>>(credentials: C) -> Builder {
        Builder::new(credentials)
    }

    /// Create an endpoint to [https://api.mod.io/v1](https://docs.mod.io/#mod-io-api-v1).
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

    /// Consume the endpoint and create an endpoint with new credentials.
    pub fn with_credentials<CR>(self, credentials: CR) -> Self
    where
        CR: Into<Credentials>,
    {
        Self {
            host: self.host,
            client: self.client,
            credentials: credentials.into(),
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
    pub fn game(&self, game_id: u32) -> GameRef {
        GameRef::new(self.clone(), game_id)
    }

    /// Return a reference to a mod.
    pub fn mod_(&self, game_id: u32, mod_id: u32) -> ModRef {
        ModRef::new(self.clone(), game_id, mod_id)
    }

    /// Returns [`Downloader`](download/struct.Downloader.html) for saving to file or retrieving
    /// the data via [`Stream`](futures_core::Stream).
    ///
    /// The download fails with [`modio::download::Error`](download/enum.Error.html) as source
    /// if a primary file, a specific file or a specific version is not found.
    ///
    /// # Example
    /// ```no_run
    /// use futures_util::{future, TryStreamExt};
    /// use modio::download::{DownloadAction, ResolvePolicy};
    /// # use modio::{Credentials, Modio};
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// #    let modio = modio::Modio::new(
    /// #        modio::Credentials::ApiKey(String::from("user-or-game-api-key")),
    /// #    )?;
    ///
    /// // Download the primary file of a mod.
    /// let action = DownloadAction::Primary {
    ///     game_id: 5,
    ///     mod_id: 19,
    /// };
    /// modio.download(action).save_to_file("mod.zip").await?;
    ///
    /// // Download the specific file of a mod.
    /// let action = DownloadAction::File {
    ///     game_id: 5,
    ///     mod_id: 19,
    ///     file_id: 101,
    /// };
    /// modio.download(action).save_to_file("mod.zip").await?;
    ///
    /// // Download the specific version of a mod.
    /// // if multiple files are found then the latest file is downloaded.
    /// // Set policy to `ResolvePolicy::Fail` to return with
    /// // `modio::download::Error::MultipleFilesFound` as source error.
    /// let action = DownloadAction::Version {
    ///     game_id: 5,
    ///     mod_id: 19,
    ///     version: "0.1".to_string(),
    ///     policy: ResolvePolicy::Latest,
    /// };
    /// modio.download(action)
    ///     .stream()
    ///     .try_for_each(|bytes| {
    ///         println!("Bytes: {:?}", bytes);
    ///         future::ok(())
    ///     })
    ///     .await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub fn download<A>(&self, action: A) -> Downloader
    where
        DownloadAction: From<A>,
    {
        Downloader::new(self.clone(), action.into())
    }

    /// Return a reference to an interface that provides access to resources owned by the user
    /// associated with the current authentication credentials.
    pub fn me(&self) -> Me {
        Me::new(self.clone())
    }

    /// Return a reference to an interface that provides access to user information.
    pub fn users(&self) -> Users {
        Users::new(self.clone())
    }

    /// Return a reference to an interface to report games, mods and users.
    pub fn reports(&self) -> Reports {
        Reports::new(self.clone())
    }

    fn request(&self, route: routing::Route) -> RequestBuilder {
        RequestBuilder::new(self.clone(), route)
    }

    fn stream<'a, T>(self, route: routing::Route, filter: filter::Filter) -> Iter<'a, T>
    where
        T: DeserializeOwned + Send + 'a,
    {
        Iter::new(self, route, filter)
    }
}

trait QueryString {
    fn to_query_string(&self) -> String;
}
