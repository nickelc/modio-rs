use std::sync::Arc;

use http::header::USER_AGENT;
use http::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, Proxy};

use crate::auth::Credentials;
use crate::error::{self, Error, Result};
use crate::types::id::{GameId, UserId};
use crate::{TargetPlatform, TargetPortal};

use super::{ClientRef, Modio};
use super::{DEFAULT_AGENT, DEFAULT_HOST, TEST_HOST};

/// A `Builder` can be used to create a `Modio` client with custom configuration.
#[must_use]
pub struct Builder {
    config: Config,
}

struct Config {
    host: Option<String>,
    credentials: Credentials,
    builder: Option<ClientBuilder>,
    headers: HeaderMap,
    proxies: Vec<Proxy>,
    #[cfg(feature = "__tls")]
    tls: TlsBackend,
    error: Option<Error>,
}

#[cfg(feature = "__tls")]
enum TlsBackend {
    #[cfg(feature = "default-tls")]
    Default,
    #[cfg(feature = "rustls-tls")]
    Rustls,
}

#[cfg(feature = "__tls")]
#[allow(clippy::derivable_impls)]
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
                credentials: credentials.into(),
                builder: None,
                headers: HeaderMap::new(),
                proxies: Vec::new(),
                #[cfg(feature = "__tls")]
                tls: TlsBackend::default(),
                error: None,
            },
        }
    }

    /// Returns a `Modio` client that uses this `Builder` configuration.
    pub fn build(self) -> Result<Modio> {
        let config = self.config;

        if let Some(e) = config.error {
            return Err(e);
        }

        let host = config.host.unwrap_or_else(|| DEFAULT_HOST.to_string());
        let credentials = config.credentials;

        let client = {
            let mut builder = {
                let builder = config.builder.unwrap_or_else(Client::builder);
                #[cfg(feature = "__tls")]
                match config.tls {
                    #[cfg(feature = "default-tls")]
                    TlsBackend::Default => builder.use_native_tls(),
                    #[cfg(feature = "rustls-tls")]
                    TlsBackend::Rustls => builder.use_rustls_tls(),
                }

                #[cfg(not(feature = "__tls"))]
                builder
            };

            let mut headers = config.headers;
            if !headers.contains_key(USER_AGENT) {
                headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_AGENT));
            }

            for proxy in config.proxies {
                builder = builder.proxy(proxy);
            }

            builder
                .default_headers(headers)
                .build()
                .map_err(error::builder)?
        };

        Ok(Modio {
            inner: Arc::new(ClientRef {
                host,
                client,
                credentials,
            }),
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

    /// Set the mod.io API host to "https://g-{id}.modapi.io/v1".
    pub fn game_host(mut self, game_id: GameId) -> Self {
        self.config.host = Some(format!("https://g-{game_id}.modapi.io/v1"));
        self
    }

    /// Set the mod.io API host to "https://u-{id}.modapi.io/v1".
    pub fn user_host(mut self, user_id: UserId) -> Self {
        self.config.host = Some(format!("https://u-{user_id}.modapi.io/v1"));
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
    pub fn user_agent<V>(mut self, value: V) -> Builder
    where
        V: TryInto<HeaderValue>,
        V::Error: Into<http::Error>,
    {
        match value.try_into() {
            Ok(value) => {
                self.config.headers.insert(USER_AGENT, value);
            }
            Err(e) => {
                self.config.error = Some(error::builder(e.into()));
            }
        }
        self
    }

    /// Add a `Proxy` to the list of proxies the client will use.
    pub fn proxy(mut self, proxy: Proxy) -> Builder {
        self.config.proxies.push(proxy);
        self
    }

    /// Set the target platform.
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#targeting-a-platform) for more information.
    pub fn target_platform(mut self, platform: TargetPlatform) -> Builder {
        match HeaderValue::from_str(platform.as_str()) {
            Ok(value) => {
                self.config.headers.insert("X-Modio-Platform", value);
            }
            Err(e) => {
                self.config.error = Some(error::builder(e));
            }
        }
        self
    }

    /// Set the target portal.
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#targeting-a-portal) for more information.
    pub fn target_portal(mut self, portal: TargetPortal) -> Builder {
        match HeaderValue::from_str(portal.as_str()) {
            Ok(value) => {
                self.config.headers.insert("X-Modio-Portal", value);
            }
            Err(e) => {
                self.config.error = Some(error::builder(e));
            }
        }
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
