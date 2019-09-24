use std::io::{self, Write};

use futures_util::TryFutureExt;
use log::{debug, log_enabled, trace};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::multipart::Form;
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::Credentials;
use crate::download::DownloadAction;
use crate::error::{self, Result};
use crate::routing::{AuthMethod, Route};
use crate::types::ModioErrorResponse;
use crate::Modio;

pub struct RequestBuilder {
    modio: Modio,
    request: Request,
}

struct Request {
    pub(crate) route: Route,
    pub(crate) query: Option<String>,
    pub(crate) body: Option<Body>,
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
        let (method, auth_method, path) = self.request.route.pieces();

        match (auth_method, &self.modio.credentials) {
            (AuthMethod::ApiKey, Credentials::Token(_, _)) => return Err(error::apikey_required()),
            (AuthMethod::Token, Credentials::ApiKey(_)) => return Err(error::token_required()),
            _ => {}
        }
        let url = match self.request.query {
            Some(query) => format!("{}{}?{}", self.modio.host, path, query),
            None => format!("{}{}", self.modio.host, path),
        };

        let url = if let Credentials::ApiKey(ref api_key) = self.modio.credentials {
            Url::parse_with_params(&url, Some(("api_key", api_key))).map_err(error::builder)?
        } else {
            url.parse().map_err(error::builder)?
        };

        debug!("request: {} {}", method, url);
        let mut req = self.modio.client.request(method, url);

        if let Credentials::Token(ref token, _) = self.modio.credentials {
            req = req.bearer_auth(token);
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
            let remaining = response
                .headers()
                .get(super::X_RATELIMIT_REMAINING)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            let reset = response
                .headers()
                .get(super::X_RATELIMIT_RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            (remaining, reset)
        };

        let body = response.bytes().map_err(error::request).await?;

        if log_enabled!(log::Level::Trace) {
            match std::str::from_utf8(&body) {
                Ok(s) => trace!("status: {}, response: {}", status, s),
                Err(_) => trace!("status: {}, response: {:?}", status, body),
            }
        }

        if status.is_success() {
            serde_json::from_slice::<Out>(&body).map_err(error::decode)
        } else {
            match (remaining, reset) {
                (Some(remaining), Some(reset)) if remaining == 0 => {
                    debug!("ratelimit reached: reset in {} mins", reset);
                    Err(error::ratelimit(reset))
                }
                _ => serde_json::from_slice::<ModioErrorResponse>(&body)
                    .map(|mer| Err(error::error_for_status(status, mer.error)))
                    .map_err(error::decode)?,
            }
        }
    }
}

pub async fn download<A, W>(modio: &Modio, action: A, w: W) -> Result<(u64, W)>
where
    A: Into<DownloadAction>,
    W: Write + Send,
{
    match action.into() {
        DownloadAction::Primary { game_id, mod_id } => {
            let modref = modio.mod_(game_id, mod_id);
            let m = modref.get().await?;
            if let Some(file) = m.modfile {
                let url = file.download.binary_url;
                request_file(&modio.client, url, w).await
            } else {
                Err(error::download_no_primary(game_id, mod_id))
            }
        }
        DownloadAction::File(file) => {
            let url = file.download.binary_url;
            request_file(&modio.client, url, w).await
        }
        DownloadAction::FileRef {
            game_id,
            mod_id,
            file_id,
        } => {
            let fileref = modio.mod_(game_id, mod_id).file(file_id);
            let file = fileref.get().await?;
            let url = file.download.binary_url;
            request_file(&modio.client, url, w)
                .await
                .map_err(move |e| match e.kind() {
                    error::Kind::Status(StatusCode::NOT_FOUND) => {
                        error::download_file_not_found(game_id, mod_id, file_id)
                    }
                    _ => e,
                })
        }
        DownloadAction::Version {
            game_id,
            mod_id,
            version,
            policy,
        } => {
            use crate::download::ResolvePolicy::*;
            use crate::files::filters::{DateAdded, Version};
            use crate::filter::prelude::*;

            let filter = Version::eq(version.clone())
                .order_by(DateAdded::desc())
                .limit(2);

            let files = modio.mod_(game_id, mod_id).files();
            let mut list = files.list(filter).await?;

            let (file, error) = match (list.count, policy) {
                (0, _) => (
                    None,
                    Some(error::download_version_not_found(game_id, mod_id, version)),
                ),
                (1, _) => (list.shift(), None),
                (_, Latest) => (list.shift(), None),
                (_, Fail) => (
                    None,
                    Some(error::download_multiple_files(game_id, mod_id, version)),
                ),
            };

            if let Some(file) = file {
                let url = file.download.binary_url;
                request_file(&modio.client, url, w).await
            } else {
                Err(error.expect("bug in previous match!"))
            }
        }
    }
}

async fn request_file<W>(client: &Client, url: Url, mut out: W) -> Result<(u64, W)>
where
    W: Write + Send,
{
    debug!("downloading file: {}", url);

    let mut response = client
        .request(Method::GET, url)
        .send()
        .map_err(error::builder_or_request)
        .await?
        .error_for_status()
        .map_err(error::request)?;

    let mut n = 0;
    while let Some(chunk) = response.chunk().map_err(error::request).await? {
        n += io::copy(&mut io::Cursor::new(&chunk), &mut out).map_err(error::decode)?;
    }
    Ok((n, out))
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
