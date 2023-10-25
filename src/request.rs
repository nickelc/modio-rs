use futures_util::TryFutureExt;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::multipart::Form;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tracing::{debug, level_enabled, trace};
use url::Url;

use crate::auth::Token;
use crate::error::{self, Result};
use crate::routing::{Parts, Route};
use crate::types::ErrorResponse;
use crate::Modio;

#[allow(dead_code)]
mod headers {
    const X_MODIO_ERROR_REF: &str = "x-modio-error-ref";
    const X_MODIO_REQUEST_ID: &str = "x-modio-request-id";

    use http::header::{HeaderMap, RETRY_AFTER};

    pub fn retry_after(headers: &HeaderMap) -> Option<u64> {
        headers
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
    }
}

pub struct RequestBuilder {
    modio: Modio,
    request: Result<reqwest::RequestBuilder>,
}

impl RequestBuilder {
    pub fn new(modio: Modio, route: Route) -> Self {
        let Parts {
            method,
            path,
            token_required,
        } = route.into_parts();

        if let (true, None) = (token_required, &modio.inner.credentials.token) {
            return Self {
                modio,
                request: Err(error::token_required()),
            };
        }

        let url = format!("{}{}", modio.inner.host, path);
        let params = [("api_key", &modio.inner.credentials.api_key)];
        let request = Url::parse_with_params(&url, &params)
            .map(|url| {
                let mut req = modio.inner.client.request(method, url);

                if let (true, Some(Token { value, .. })) =
                    (token_required, &modio.inner.credentials.token)
                {
                    req = req.bearer_auth(value);
                }
                req
            })
            .map_err(error::builder);

        Self { modio, request }
    }

    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        Self {
            request: self.request.map(|r| r.query(query)),
            ..self
        }
    }

    pub fn form<T: Serialize + ?Sized>(self, form: &T) -> Self {
        Self {
            request: self.request.map(|r| r.form(form)),
            ..self
        }
    }

    pub fn multipart(self, form: Form) -> Self {
        Self {
            request: self.request.map(|r| r.multipart(form)),
            ..self
        }
    }

    pub async fn send<Out>(self) -> Result<Out>
    where
        Out: DeserializeOwned + Send,
    {
        let mut req = self.request?.build().map_err(error::builder)?;
        if !req.headers().contains_key(CONTENT_TYPE) {
            req.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }

        debug!("request: {} {}", req.method(), req.url());
        let response = self
            .modio
            .inner
            .client
            .execute(req)
            .map_err(error::request)
            .await?;

        let status = response.status();

        let retry_after = if status.is_success() {
            None
        } else {
            headers::retry_after(response.headers())
        };

        trace!("response headers: {:?}", response.headers());

        let body = response.bytes().map_err(error::request).await?;

        if level_enabled!(tracing::Level::TRACE) {
            match std::str::from_utf8(&body) {
                Ok(s) => trace!("status: {}, response: {}", status, s),
                Err(_) => trace!("status: {}, response: {:?}", status, body),
            }
        }

        if status == StatusCode::NO_CONTENT {
            serde_json::from_str("null").map_err(error::decode)
        } else if status.is_success() {
            serde_json::from_slice(&body).map_err(error::decode)
        } else if let Some(retry_after) = retry_after {
            debug!("ratelimit reached: retry after {retry_after} seconds");
            Err(error::ratelimit(retry_after))
        } else {
            serde_json::from_slice::<ErrorResponse>(&body)
                .map(|mer| Err(error::error_for_status(status, mer.error)))
                .map_err(error::decode)?
        }
    }
}
