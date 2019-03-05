//! Modio provides a set of building blocks for interacting with the [mod.io](https://mod.io) API.
//!
//! The client uses asynchronous I/O, backed by the `futures` and `tokio` crates, and requires both
//! to be used alongside.
//!
//! # Authentication
//!
//! To access the API authentication is required and can be done via 4 ways:
//!
//! - Request an [API key (Read-only)](https://mod.io/apikey)
//! - Manually create an [OAuth 2 Access Token (Read + Write)](https://mod.io/oauth)
//! - [Email Authentication Flow](auth/struct.Auth.html#example) to create an OAuth 2 Access Token
//! (Read + Write)
//! - [Encrypted steam user auth ticket](auth/struct.Auth.html#method.steam_auth) to create an
//! OAuth 2 Access Token (Read + Write)
//!
//! # Rate Limiting
//!
//! For API requests using API key authentication are **unlimited** and for OAuth 2 authentication
//! requests are limited to **120 requests per hour**.
//!
//! A special error [ErrorKind::RateLimit](error/enum.ErrorKind.html#variant.RateLimit) will
//! be return from api operations when the rate limit associated with credentials has been
//! exhausted.
//!
//! # Example: Basic setup
//!
//! ```no_run
//! use modio::{Credentials, Error, Modio};
//! use tokio::runtime::Runtime;
//!
//! fn main() -> Result<(), Error> {
//!     let mut rt = Runtime::new().expect("new rt");
//!     let modio = Modio::new(
//!         Credentials::ApiKey(String::from("user-or-game-api-key")),
//!     )?;
//!
//!     // create some tasks and execute them
//!     // let result = rt.block_on(task)?;
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
//! use modio::{Credentials, Error, Modio};
//! use tokio::prelude::*;
//! use tokio::runtime::Runtime;
//!
//! fn main() -> Result<(), Error> {
//!     let mut rt = Runtime::new().expect("new rt");
//!     let modio = Modio::new(
//!         Credentials::ApiKey(String::from("user-or-game-api-key")),
//!     )?;
//!
//!     // OpenXcom: The X-Com Files
//!     let modref = modio.mod_(51, 158);
//!
//!     // Get mod with its dependencies and all files
//!     let mod_ = modref.get();
//!     let deps = modref.dependencies().list();
//!     let files = modref.files().list(&Default::default());
//!
//!     let task = mod_.join(deps).join(files);
//!
//!     match rt.block_on(task) {
//!         Ok(((m, deps), files)) => {
//!             println!("{}", m.name);
//!             println!(
//!                 "deps: {:?}",
//!                 deps.into_iter().map(|d| d.mod_id).collect::<Vec<_>>()
//!             );
//!             for file in files {
//!                 println!("file id: {} version: {:?}", file.id, file.version);
//!             }
//!         }
//!         Err(e) => println!("{}", e),
//!     };
//!     Ok(())
//! }
//! ```
//!
//! # Example: Downloading mods
//!
//! ```no_run
//! use std::fs::File;
//!
//! use modio::download::ResolvePolicy;
//! use modio::{Credentials, DownloadAction, Error, Modio};
//! use tokio::runtime::Runtime;
//!
//! fn main() -> Result<(), Error> {
//!     let mut rt = Runtime::new().expect("new rt");
//!     let modio = Modio::new(
//!         Credentials::ApiKey(String::from("user-or-game-api-key")),
//!     )?;
//!     let out = File::create("mod.zip").expect("new file");
//!
//!     // Download the primary file of a mod.
//!     let action = DownloadAction::Primary {
//!         game_id: 5,
//!         mod_id: 19,
//!     };
//!     let (len, out) = rt.block_on(modio.download(action, out))?;
//!
//!     // Download the specific file of a mod.
//!     let action = DownloadAction::File {
//!         game_id: 5,
//!         mod_id: 19,
//!         file_id: 101,
//!     };
//!     let (len, out) = rt.block_on(modio.download(action, out))?;
//!
//!     // Download the specific version of a mod.
//!     // if multiple files are found then the latest file is downloaded.
//!     // Set policy to `ResolvePolicy::Fail` to return with
//!     // `ErrorKind::Download(DownloadError::MultipleFilesFound)`.
//!     let action = DownloadAction::Version {
//!         game_id: 5,
//!         mod_id: 19,
//!         version: "0.1".to_string(),
//!         policy: ResolvePolicy::Latest,
//!     };
//!     let (len, out) = rt.block_on(modio.download(action, out))?;
//!     Ok(())
//! }
//! ```
#![doc(html_root_url = "https://docs.rs/modio/0.3.0")]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;
use std::marker::PhantomData;

use futures::{future, stream, Future as StdFuture, IntoFuture, Stream as StdStream};
use log::{debug, log_enabled, trace};
use mime::Mime;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::r#async::multipart::Form;
use reqwest::r#async::{Body, Client, ClientBuilder};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

#[macro_use]
mod macros;

pub mod auth;
#[macro_use]
pub mod filter;
pub mod comments;
pub mod download;
pub mod error;
pub mod files;
pub mod games;
pub mod me;
pub mod metadata;
pub mod mods;
mod multipart;
pub mod reports;
pub mod teams;
mod types;
pub mod users;

use crate::auth::Auth;
use crate::comments::Comments;
use crate::games::{GameRef, Games};
use crate::me::Me;
use crate::mods::{ModRef, Mods};
use crate::reports::Reports;
use crate::types::ModioMessage;
use crate::users::Users;

pub use crate::auth::Credentials;
pub use crate::download::DownloadAction;
pub use crate::error::{Error, Result};
pub use crate::types::{ModioErrorResponse, ModioListResponse};
pub use reqwest::Proxy;

const DEFAULT_HOST: &str = "https://api.mod.io/v1";
const TEST_HOST: &str = "https://api.test.mod.io/v1";
const DEFAULT_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION"));

pub type Future<T> = Box<dyn StdFuture<Item = T, Error = Error> + Send>;
pub type Stream<T> = Box<dyn StdStream<Item = T, Error = Error> + Send>;
pub type List<T> = ModioListResponse<T>;

mod prelude {
    pub use futures::{Future as StdFuture, Stream as StdStream};
    pub use reqwest::r#async::multipart::{Form, Part};
    pub use reqwest::r#async::Body;
    pub use reqwest::StatusCode;

    pub use crate::List;
    pub use crate::Modio;
    pub(crate) use crate::ModioMessage;
    pub use crate::QueryParams;
    pub(crate) use crate::RequestBody;
    pub use crate::{AddOptions, DeleteOptions, Endpoint};
    pub use crate::{Future, Stream};
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
                    TlsBackend::Default => builder.use_default_tls(),
                    #[cfg(feature = "rustls-tls")]
                    TlsBackend::Rustls => builder.use_rustls_tls(),
                }

                #[cfg(not(feature = "tls"))]
                builder
            };

            let mut headers = HeaderMap::new();
            let agent = match config.agent {
                Some(agent) => HeaderValue::from_str(&agent).map_err(error::from)?,
                None => HeaderValue::from_static(DEFAULT_AGENT),
            };
            headers.insert(USER_AGENT, agent);

            for proxy in config.proxies {
                builder = builder.proxy(proxy);
            }

            builder
                .default_headers(headers)
                .build()
                .map_err(error::from)?
        };

        Ok(Modio {
            host,
            credentials,
            client,
        })
    }

    /// Configure the underlying `reqwest` client using `reqwest::async::ClientBuilder`.
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

    /// Performs a download into a writer.
    ///
    /// Fails with [`ErrorKind::Download`](error/enum.ErrorKind.html#variant.Download) if a primary file,
    /// a specific file or a specific version is not found.
    /// # Example
    /// ```no_run
    /// use std::fs::File;
    ///
    /// use modio::download::ResolvePolicy;
    /// use modio::{Credentials, DownloadAction, Error, Modio};
    /// use tokio::runtime::Runtime;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let mut rt = Runtime::new().expect("new rt");
    ///     let modio = Modio::new(
    ///         Credentials::ApiKey(String::from("user-or-game-api-key")),
    ///     )?;
    ///     let out = File::create("mod.zip").expect("new file");
    ///
    ///     // Download the primary file of a mod.
    ///     let action = DownloadAction::Primary {
    ///         game_id: 5,
    ///         mod_id: 19,
    ///     };
    ///     let (len, out) = rt.block_on(modio.download(action, out))?;
    ///
    ///     // Download the specific file of a mod.
    ///     let action = DownloadAction::File {
    ///         game_id: 5,
    ///         mod_id: 19,
    ///         file_id: 101,
    ///     };
    ///     let (len, out) = rt.block_on(modio.download(action, out))?;
    ///
    ///     // Download the specific version of a mod.
    ///     // if multiple files are found then the latest file is downloaded.
    ///     // Set policy to `ResolvePolicy::Fail` to return with
    ///     // `ErrorKind::Download(DownloadError::MultipleFilesFound)`.
    ///     let action = DownloadAction::Version {
    ///         game_id: 5,
    ///         mod_id: 19,
    ///         version: "0.1".to_string(),
    ///         policy: ResolvePolicy::Latest,
    ///     };
    ///     let (len, out) = rt.block_on(modio.download(action, out))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn download<A, W>(&self, action: A, w: W) -> Future<(u64, W)>
    where
        A: Into<DownloadAction>,
        W: Write + 'static + Send,
    {
        let instance = self.clone();
        match action.into() {
            DownloadAction::Primary { game_id, mod_id } => {
                Box::new(self.mod_(game_id, mod_id).get().and_then(move |m| {
                    if let Some(file) = m.modfile {
                        instance.request_file(&file.download.binary_url.to_string(), w)
                    } else {
                        future_err!(error::download_no_primary(game_id, mod_id))
                    }
                }))
            }
            DownloadAction::File {
                game_id,
                mod_id,
                file_id,
            } => Box::new(
                self.mod_(game_id, mod_id)
                    .file(file_id)
                    .get()
                    .and_then(move |file| {
                        instance.request_file(&file.download.binary_url.to_string(), w)
                    })
                    .map_err(move |e| match e.kind() {
                        error::ErrorKind::Fault {
                            code: StatusCode::NOT_FOUND,
                            ..
                        } => error::download_file_not_found(game_id, mod_id, file_id),
                        _ => e,
                    }),
            ),
            DownloadAction::Version {
                game_id,
                mod_id,
                version,
                policy,
            } => {
                let mut opts = files::FileListOptions::new();
                opts.version(filter::Operator::Equals, version.clone());
                opts.sort_by(files::FileListOptions::DATE_ADDED, filter::Order::Desc);
                opts.limit(2);

                Box::new(
                    self.mod_(game_id, mod_id)
                        .files()
                        .list(&opts)
                        .and_then(move |list| {
                            use crate::download::ResolvePolicy::*;

                            let (file, error) = match (list.count, policy) {
                                (0, _) => (
                                    None,
                                    Some(error::download_version_not_found(
                                        game_id, mod_id, version,
                                    )),
                                ),
                                (1, _) => (Some(&list[0]), None),
                                (_, Latest) => (Some(&list[0]), None),
                                (_, Fail) => (
                                    None,
                                    Some(error::download_multiple_files(game_id, mod_id, version)),
                                ),
                            };

                            if let Some(file) = file {
                                instance.request_file(&file.download.binary_url.to_string(), w)
                            } else {
                                future_err!(error.expect("bug in previous match!"))
                            }
                        }),
                )
            }
            DownloadAction::Url(url) => self.request_file(&url.to_string(), w),
        }
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

    fn request<B, Out>(&self, method: Method, uri: &str, body: B) -> Future<(Url, Out)>
    where
        B: Into<RequestBody> + 'static + Send,
        Out: DeserializeOwned + 'static + Send,
    {
        let url = if let Credentials::ApiKey(ref api_key) = self.credentials {
            Url::parse(&uri)
                .map(|mut url| {
                    url.query_pairs_mut().append_pair("api_key", api_key);
                    url
                })
                .map_err(error::from)
                .into_future()
        } else {
            uri.parse().map_err(error::from).into_future()
        };

        let instance = self.clone();

        let response = url.and_then(move |url| {
            debug!("request: {} {}", method, url);
            let mut req = instance.client.request(method, url.clone());

            if let Credentials::Token(ref token) = instance.credentials {
                req = req.header(AUTHORIZATION, &*format!("Bearer {}", token));
            }

            match body.into() {
                RequestBody::Body(body, mime) => {
                    if let Some(mime) = mime {
                        req = req.header(CONTENT_TYPE, &*mime.to_string());
                    }
                    req = req.body(body);
                }
                RequestBody::Form(form) => {
                    trace!("{:?}", form);
                    req = req.multipart(form);
                }
                _ => {}
            }
            req.send()
                .map_err(error::from)
                .and_then(|res| Ok((url, res)))
        });

        Box::new(response.and_then(move |(url, response)| {
            let remaining = response
                .headers()
                .get(X_RATELIMIT_REMAINING)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            let reset = response
                .headers()
                .get(X_RATELIMIT_RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());

            let status = response.status();
            Box::new(
                response
                    .into_body()
                    .concat2()
                    .map_err(error::from)
                    .and_then(move |response_body| {
                        if log_enabled!(log::Level::Trace) {
                            match std::str::from_utf8(&response_body) {
                                Ok(s) => trace!("response: {}", s),
                                Err(_) => trace!("response: {:?}", response_body),
                            }
                        }

                        if status.is_success() {
                            serde_json::from_slice::<Out>(&response_body)
                                .map(|out| (url, out))
                                .map_err(error::from)
                        } else {
                            match (remaining, reset) {
                                (Some(remaining), Some(reset)) if remaining == 0 => {
                                    debug!("ratelimit reached: reset in {} mins", reset);
                                    Err(error::ratelimit(reset))
                                }
                                _ => serde_json::from_slice::<ModioErrorResponse>(&response_body)
                                        .map(|mer| Err(error::fault(status, mer.error)))
                                        .map_err(error::from)?,
                            }
                        }
                    }),
            )
        }))
    }

    fn request_entity<B, D>(&self, method: Method, uri: &str, body: B) -> Future<D>
    where
        B: Into<RequestBody> + 'static + Send,
        D: DeserializeOwned + 'static + Send,
    {
        Box::new(self.request(method, uri, body).map(|(_, entity)| entity))
    }

    fn request_file<W>(&self, uri: &str, out: W) -> Future<(u64, W)>
    where
        W: Write + 'static + Send,
    {
        debug!("downloading file: {}", uri);
        let url = Url::parse(uri).map_err(error::from).into_future();

        let instance = self.clone();
        let response = url.and_then(move |url| {
            instance
                .client
                .request(Method::GET, url)
                .send()
                .map_err(error::from)
        });

        Box::new(response.and_then(move |response| {
            Box::new(response.into_body().map_err(error::from).fold(
                (0, out),
                |(len, mut out), chunk| {
                    io::copy(&mut io::Cursor::new(&chunk), &mut out)
                        .map(|n| (n + len, out))
                        .map_err(error::from)
                        .into_future()
                },
            ))
        }))
    }

    fn stream<D>(&self, uri: &str) -> Stream<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        struct State<D>
        where
            D: DeserializeOwned + 'static + Send,
        {
            url: Url,
            items: Vec<D>,
            offset: u32,
            limit: u32,
            count: u32,
        }

        let instance = self.clone();

        Box::new(
            self.request::<_, List<D>>(Method::GET, &(self.host.clone() + uri), RequestBody::Empty)
                .map(move |(url, list)| {
                    debug!("streaming result: {}", url);

                    let mut state = State {
                        url,
                        items: list.data,
                        offset: list.offset,
                        limit: list.limit,
                        count: list.total,
                    };
                    state.items.reverse();

                    stream::unfold::<_, _, Future<(D, State<D>)>, _>(state, move |mut state| {
                        match state.items.pop() {
                            Some(item) => {
                                state.count -= 1;
                                Some(Box::new(future::ok((item, state))))
                            }
                            _ => {
                                if state.count == 0 {
                                    return None;
                                }
                                let mut map = BTreeMap::new();
                                for (key, value) in state.url.query_pairs().into_owned() {
                                    map.insert(key, value);
                                }
                                map.insert(
                                    "_offset".to_string(),
                                    (state.offset + state.limit).to_string(),
                                );
                                state.url.query_pairs_mut().clear();
                                state.url.query_pairs_mut().extend_pairs(map.iter());

                                debug!("loading next page: {}", state.url);

                                let next = Box::new(
                                    instance
                                        .request::<_, List<D>>(
                                            Method::GET,
                                            &state.url.to_string(),
                                            RequestBody::Empty,
                                        )
                                        .map(move |(url, list)| {
                                            let mut state = State {
                                                url,
                                                items: list.data,
                                                limit: state.limit,
                                                offset: state.offset + state.limit,
                                                count: state.count - 1,
                                            };
                                            let item = state.items.remove(0);
                                            state.items.reverse();
                                            (item, state)
                                        }),
                                )
                                    as Future<(D, State<D>)>;
                                Some(next)
                            }
                        }
                    })
                })
                .into_stream()
                .flatten(),
        )
    }

    fn get<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request_entity(Method::GET, &(self.host.clone() + uri), RequestBody::Empty)
    }

    fn post<D, B>(&self, uri: &str, body: B) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        B: Into<RequestBody>,
    {
        self.request_entity(
            Method::POST,
            &(self.host.clone() + uri),
            (body.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
        )
    }

    fn post_form<D, M>(&self, uri: &str, data: M) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        M: Into<Form>,
    {
        self.request_entity(
            Method::POST,
            &(self.host.clone() + uri),
            RequestBody::Form(data.into()),
        )
    }

    fn put<D, B>(&self, uri: &str, body: B) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        B: Into<RequestBody>,
    {
        self.request_entity(
            Method::PUT,
            &(self.host.clone() + uri),
            (body.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
        )
    }

    fn delete<B>(&self, uri: &str, body: B) -> Future<()>
    where
        B: Into<RequestBody>,
    {
        Box::new(
            self.request_entity(
                Method::DELETE,
                &(self.host.clone() + uri),
                (body.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
            )
            .or_else(|err| match err.kind() {
                error::ErrorKind::Json(_) => Ok(()),
                _ => Err(err),
            }),
        )
    }
}

pub(crate) enum RequestBody {
    Empty,
    Body(Body, Option<Mime>),
    Form(Form),
}

impl From<String> for RequestBody {
    fn from(s: String) -> RequestBody {
        RequestBody::Body(Body::from(s), None)
    }
}

impl From<(RequestBody, Mime)> for RequestBody {
    fn from(body: (RequestBody, Mime)) -> RequestBody {
        match body {
            (RequestBody::Body(body, _), mime) => RequestBody::Body(body, Some(mime)),
            (RequestBody::Empty, _) => RequestBody::Empty,
            _ => body.0,
        }
    }
}

/// Generic endpoint for sub-resources
pub struct Endpoint<Out>
where
    Out: DeserializeOwned + 'static,
{
    modio: Modio,
    path: String,
    phantom: PhantomData<Out>,
}

impl<Out> Endpoint<Out>
where
    Out: DeserializeOwned + 'static + Send,
{
    pub(crate) fn new(modio: Modio, path: String) -> Endpoint<Out> {
        Self {
            modio,
            path,
            phantom: PhantomData,
        }
    }

    pub fn list(&self) -> Future<List<Out>> {
        self.modio.get(&self.path)
    }

    pub fn iter(&self) -> Stream<Out> {
        self.modio.stream(&self.path)
    }

    /// [required: token]
    pub fn add<T: AddOptions + QueryParams>(&self, options: &T) -> Future<String> {
        token_required!(self.modio);
        let params = options.to_query_params();
        Box::new(
            self.modio
                .post::<ModioMessage, _>(&self.path, params)
                .map(|m| m.message),
        )
    }

    /// [required: token]
    pub fn delete<T: DeleteOptions + QueryParams>(&self, options: &T) -> Future<()> {
        token_required!(self.modio);
        let params = options.to_query_params();
        self.modio.delete(&self.path, params)
    }
}

filter_options! {
    /// Options used to filter event listings.
    ///
    /// # Filter parameters
    /// - id
    /// - game_id
    /// - mod_id
    /// - user_id
    /// - date_added
    /// - event_type
    ///
    /// # Sorting
    /// - id
    ///
    /// See the [modio docs](https://docs.mod.io/#events) for more information.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::EventListOptions;
    /// use modio::mods::EventType;
    ///
    /// let mut opts = EventListOptions::new();
    /// opts.id(Operator::GreaterThan, 1024);
    /// opts.event_type(Operator::Equals, EventType::ModfileChanged);
    /// ```
    #[derive(Debug)]
    pub struct EventListOptions {
        Filters
        - id = "id";
        - game_id = "game_id";
        - mod_id = "mod_id";
        - user_id = "user_id";
        - date_added = "date_added";
        - event_type = "event_type";

        Sort
        - ID = "id";
    }
}

pub trait AddOptions {}
pub trait DeleteOptions {}

pub trait QueryParams {
    fn to_query_params(&self) -> String;
}
