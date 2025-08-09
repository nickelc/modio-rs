use http::header::{HeaderMap, HeaderValue, USER_AGENT};
use http::uri::Authority;

use crate::error::{self, Error};
use crate::types::id::{GameId, UserId};
use crate::types::{TargetPlatform, TargetPortal};

use super::{Client, DEFAULT_HOST, HDR_X_MODIO_PLATFORM, HDR_X_MODIO_PORTAL, TEST_HOST};

use super::service::Svc;

/// A builder for [`Client`].
pub struct Builder {
    host: Option<Box<str>>,
    api_key: Box<str>,
    token: Option<Box<str>>,
    headers: HeaderMap,
    error: Option<Error>,
}

impl Builder {
    /// Create a new builder with an API key.
    pub fn new(api_key: String) -> Self {
        Self {
            host: None,
            api_key: api_key.into_boxed_str(),
            token: None,
            headers: HeaderMap::new(),
            error: None,
        }
    }

    /// Build the [`Client`].
    pub fn build(self) -> Result<Client, Error> {
        if let Some(e) = self.error {
            return Err(e);
        }

        let http = Svc::new();

        let host = self.host.unwrap_or_else(|| Box::from(DEFAULT_HOST));

        Ok(Client {
            http,
            host,
            api_key: self.api_key,
            token: self.token,
            headers: self.headers,
        })
    }

    /// Set the token to use for HTTP requests.
    pub fn token(mut self, token: String) -> Self {
        self.token = Some(create_token(token));
        self
    }

    /// Use the default mod.io API host (`"api.mod.io"`).
    pub fn use_default_env(mut self) -> Self {
        self.host = Some(Box::from(DEFAULT_HOST));
        self
    }

    /// Use the mod.io API test host (`"api.test.mod.io"`).
    pub fn use_test_env(mut self) -> Self {
        self.host = Some(Box::from(TEST_HOST));
        self
    }

    /// Set the mod.io API host to "g-{id}.modapi.io".
    pub fn game_host(mut self, game_id: GameId) -> Self {
        self.host = Some(format!("g-{game_id}.modapi.io").into_boxed_str());
        self
    }

    /// Set the mod.io API host to "u-{id}.modapi.io".
    pub fn user_host(mut self, user_id: UserId) -> Self {
        self.host = Some(format!("u-{user_id}.modapi.io").into_boxed_str());
        self
    }

    /// Set the mod.io API host.
    ///
    /// Defaults to `"api.mod.io"` if not set.
    pub fn host<V>(mut self, host: V) -> Self
    where
        V: TryInto<Authority>,
        V::Error: Into<http::Error>,
    {
        match host.try_into() {
            Ok(host) => {
                self.host = Some(Box::from(host.as_str()));
            }
            Err(err) => {
                self.error = Some(error::builder(err.into()));
            }
        }
        self
    }

    /// Set the user agent used for every request.
    pub fn user_agent<V>(mut self, value: V) -> Self
    where
        V: TryInto<HeaderValue>,
        V::Error: Into<http::Error>,
    {
        match value.try_into() {
            Ok(value) => {
                self.headers.insert(USER_AGENT, value);
            }
            Err(err) => {
                self.error = Some(error::builder(err.into()));
            }
        }
        self
    }

    /// Set the target platform.
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#targeting-a-platform) for more information.
    pub fn target_platform(mut self, platform: TargetPlatform) -> Self {
        match HeaderValue::from_str(platform.as_str()) {
            Ok(value) => {
                self.headers.insert(HDR_X_MODIO_PLATFORM, value);
            }
            Err(err) => {
                self.error = Some(error::builder(err));
            }
        }
        self
    }

    /// Set the target portal.
    ///
    /// See the [mod.io docs](https://docs.mod.io/restapiref/#targeting-a-portal) for more information.
    pub fn target_portal(mut self, portal: TargetPortal) -> Self {
        match HeaderValue::from_str(portal.as_str()) {
            Ok(value) => {
                self.headers.insert(HDR_X_MODIO_PORTAL, value);
            }
            Err(err) => {
                self.error = Some(error::builder(err));
            }
        }
        self
    }
}

pub(super) fn create_token(mut token: String) -> Box<str> {
    if !token.starts_with("Bearer ") {
        token.insert_str(0, "Bearer ");
    }
    token.into_boxed_str()
}

#[cfg(test)]
mod tests {
    use super::create_token;

    #[test]
    fn test_create_token() {
        assert_eq!("Bearer token", &*create_token("token".to_owned()));
        assert_eq!("Bearer token", &*create_token("Bearer token".to_owned()));
    }
}
