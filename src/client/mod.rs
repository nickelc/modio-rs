//! HTTP client for the mod.io API.

use std::fmt;

use http::header::{Entry, HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use http::uri::Uri;
use serde::ser::Serialize;

use crate::error::{self, Error};
use crate::request::{Filter, Request, TokenRequired};
use crate::response::ResponseFuture;

mod builder;
mod conn;
mod host;
mod methods;

pub(crate) mod service;

pub use self::builder::Builder;
use self::host::Host;

pub const DEFAULT_HOST: &str = host::DEFAULT_HOST;
pub const TEST_HOST: &str = host::TEST_HOST;
const API_VERSION: u8 = 1;

const HDR_X_MODIO_PLATFORM: &str = "X-Modio-Platform";
const HDR_X_MODIO_PORTAL: &str = "X-Modio-Portal";
const HDR_FORM_URLENCODED: HeaderValue =
    HeaderValue::from_static("application/x-www-form-urlencoded");

/// HTTP client for the mod.io API.
pub struct Client {
    http: service::Svc,
    host: Host,
    api_key: Box<str>,
    token: Option<Box<str>>,
    headers: HeaderMap,
}

impl Client {
    /// Create a new builder with an API key.
    pub fn builder(api_key: String) -> Builder {
        Builder::new(api_key)
    }

    /// Retrieve the API key used by the client.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Retrieve the token used by the client.
    pub fn token(&self) -> Option<&str> {
        self.token.as_ref().and_then(|s| s.strip_prefix("Bearer "))
    }

    /// Create a new client from the current instance with the given token.
    pub fn with_token(&self, token: String) -> Self {
        Self {
            http: self.http.clone(),
            host: self.host.clone(),
            api_key: self.api_key.clone(),
            token: Some(builder::create_token(token)),
            headers: self.headers.clone(),
        }
    }

    pub(crate) fn raw_request(&self, req: Request) -> service::ResponseFuture {
        self.http.request(req)
    }

    pub(crate) fn request<T>(&self, req: Request) -> ResponseFuture<T> {
        match self.try_request(req) {
            Ok(fut) => fut,
            Err(err) => ResponseFuture::failed(err),
        }
    }

    fn try_request<T>(&self, req: Request) -> Result<ResponseFuture<T>, Error> {
        let (mut parts, body) = req.into_parts();

        let game_id = parts.extensions.get().copied();
        let mut uri = UriBuilder::new(self.host.display(game_id), &parts.uri);

        let token_required = parts.extensions.get();
        match (token_required, &self.token) {
            (Some(TokenRequired(false)) | None, _) => {
                uri.api_key(&self.api_key);
            }
            (Some(TokenRequired(true)), Some(token)) => match HeaderValue::from_str(token) {
                Ok(mut value) => {
                    value.set_sensitive(true);
                    parts.headers.insert(AUTHORIZATION, value);
                }
                Err(e) => return Err(error::request(e)),
            },
            (Some(TokenRequired(true)), None) => return Err(error::token_required()),
        }

        if let Some(filter) = parts.extensions.get::<Filter>() {
            uri.filter(filter)?;
        }

        parts.uri = uri.build()?;

        for (key, value) in &self.headers {
            if let Entry::Vacant(entry) = parts.headers.entry(key) {
                entry.insert(value.clone());
            }
        }

        if let Entry::Vacant(entry) = parts.headers.entry(CONTENT_TYPE) {
            entry.insert(HDR_FORM_URLENCODED);
        }

        let fut = self.http.request(Request::from_parts(parts, body));

        Ok(ResponseFuture::new(fut))
    }
}

struct UriBuilder<'a> {
    serializer: form_urlencoded::Serializer<'a, String>,
}

impl<'a> UriBuilder<'a> {
    fn new(host: impl fmt::Display, path: &'a Uri) -> UriBuilder<'a> {
        let mut uri = format!("https://{host}/v{API_VERSION}{path}");

        let query_start = if let Some(start) = uri.find('?') {
            start
        } else {
            uri.push('?');
            uri.len()
        };

        Self {
            serializer: form_urlencoded::Serializer::for_suffix(uri, query_start),
        }
    }

    fn api_key(&mut self, value: &str) {
        self.serializer.append_pair("api_key", value);
    }

    fn filter(&mut self, filter: &Filter) -> Result<(), Error> {
        filter
            .serialize(serde_urlencoded::Serializer::new(&mut self.serializer))
            .map_err(error::request)?;

        Ok(())
    }

    fn build(mut self) -> Result<Uri, Error> {
        self.serializer
            .finish()
            .trim_end_matches('?')
            .parse()
            .map_err(error::request)
    }
}

#[cfg(test)]
mod tests {
    use super::host::Host;
    use super::*;

    #[test]
    fn basic_uri() {
        let host = Host::Default.display(None);
        let path = Uri::from_static("/games/1/mods/2");
        let uri = UriBuilder::new(host, &path);

        let uri = uri.build().unwrap();
        assert_eq!("https://api.mod.io/v1/games/1/mods/2", uri);
    }

    #[test]
    fn uri_with_api_key() {
        let host = Host::Default.display(None);
        let path = Uri::from_static("/games/1/mods/2");
        let mut uri = UriBuilder::new(host, &path);

        uri.api_key("FOOBAR");

        let uri = uri.build().unwrap();
        assert_eq!("https://api.mod.io/v1/games/1/mods/2?api_key=FOOBAR", uri);
    }

    #[test]
    fn uri_with_filter() {
        let host = Host::Default.display(None);
        let path = Uri::from_static("/games/1/mods/2");
        let mut uri = UriBuilder::new(host, &path);

        uri.filter(&Filter::with_limit(123)).unwrap();

        let uri = uri.build().unwrap();
        assert_eq!("https://api.mod.io/v1/games/1/mods/2?_limit=123", uri);
    }

    #[test]
    fn uri_with_path_and_query() {
        let host = Host::Default.display(None);
        let path = Uri::from_static("/games/1/mods/2?foo=bar");
        let mut uri = UriBuilder::new(host, &path);

        uri.filter(&Filter::with_limit(123)).unwrap();

        let uri = uri.build().unwrap();
        assert_eq!(
            "https://api.mod.io/v1/games/1/mods/2?foo=bar&_limit=123",
            uri
        );
    }
}
