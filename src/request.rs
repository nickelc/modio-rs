use futures_util::TryFutureExt;
use log::{debug, log_enabled, trace};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::multipart::Form;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::Token;
use crate::error::{self, Result};
use crate::routing::{AuthMethod, Route};
use crate::types::ErrorResponse;
use crate::Modio;

#[allow(dead_code)]
mod headers {
    const X_MODIO_ERROR_REF: &str = "x-modio-error-ref";
    const X_MODIO_REQUEST_ID: &str = "x-modio-request-id";
    const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
    const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
    const X_RATELIMIT_RETRY_AFTER: &str = "x-ratelimit-retryafter";

    use http::HeaderMap;

    pub fn parse_headers(headers: &HeaderMap) -> (Option<u64>, Option<u64>) {
        fn to_str<'a>(headers: &'a HeaderMap, name: &'static str) -> Option<&'a str> {
            headers.get(name).and_then(|v| v.to_str().ok())
        }
        fn parse<T: std::str::FromStr>(headers: &HeaderMap, name: &'static str) -> Option<T> {
            to_str(headers, name).and_then(|v| v.parse().ok())
        }

        let remaining = parse(headers, X_RATELIMIT_REMAINING);
        let reset_after = parse(headers, X_RATELIMIT_RETRY_AFTER);
        (remaining, reset_after)
    }
}

pub struct RequestBuilder {
    modio: Modio,
    request: Request,
}

struct Request {
    route: Route,
    query: Option<String>,
    body: Option<Body>,
}

pub enum Body {
    Form(String),
    Multipart(Form),
}

impl RequestBuilder {
    pub(crate) fn new(modio: Modio, route: Route) -> Self {
        Self {
            modio,
            request: Request {
                route,
                query: None,
                body: None,
            },
        }
    }

    pub fn query(mut self, query: String) -> Self {
        self.request.query = Some(query);
        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        Body: From<T>,
    {
        self.request.body = Some(body.into());
        self
    }

    pub async fn send<Out>(self) -> Result<Out>
    where
        Out: DeserializeOwned + Send,
    {
        let (method, path, auth_method) = self.request.route.pieces();

        if let (AuthMethod::Token, None) = (&auth_method, &self.modio.credentials.token) {
            return Err(error::token_required());
        }
        let url = match self.request.query {
            Some(query) => format!("{}{}?{}", self.modio.host, path, query),
            None => format!("{}{}", self.modio.host, path),
        };

        let params = Some(("api_key", self.modio.credentials.api_key));
        let url = Url::parse_with_params(&url, params).map_err(error::builder)?;

        debug!("request: {} {}", method, url);
        let mut req = self.modio.client.request(method, url);

        if let (AuthMethod::Token, Some(Token { value, .. })) =
            (&auth_method, &self.modio.credentials.token)
        {
            req = req.bearer_auth(value);
        }

        match self.request.body {
            Some(Body::Form(s)) => {
                trace!("body: {}", s);
                req = req.header(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
                req = req.body(s);
            }
            Some(Body::Multipart(mp)) => {
                trace!("{:?}", mp);
                req = req.multipart(mp);
            }
            None => {
                req = req.header(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
            }
        }

        let response = req.send().map_err(error::builder_or_request).await?;

        let status = response.status();

        let (remaining, reset) = if status.is_success() {
            (None, None)
        } else {
            headers::parse_headers(response.headers())
        };

        let body = response.bytes().map_err(error::request).await?;

        if log_enabled!(log::Level::Trace) {
            match std::str::from_utf8(&body) {
                Ok(s) => trace!("status: {}, response: {}", status, s),
                Err(_) => trace!("status: {}, response: {:?}", status, body),
            }
        }

        if status == StatusCode::NO_CONTENT {
            serde_json::from_str("null").map_err(error::decode)
        } else if status.is_success() {
            serde_json::from_slice(&body).map_err(error::decode)
        } else {
            match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    debug!("ratelimit reached: reset in {} seconds", reset);
                    Err(error::ratelimit(reset))
                }
                _ => serde_json::from_slice::<ErrorResponse>(&body)
                    .map(|mer| Err(error::error_for_status(status, mer.error)))
                    .map_err(error::decode)?,
            }
        }
    }
}

impl From<String> for Body {
    fn from(s: String) -> Body {
        Body::Form(s)
    }
}

impl From<Form> for Body {
    fn from(form: Form) -> Body {
        Body::Multipart(form)
    }
}
