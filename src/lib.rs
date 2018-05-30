extern crate futures;
#[macro_use]
extern crate hyper;
extern crate hyper_multipart_rfc7578 as hyper_multipart;
#[cfg(feature = "tls")]
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
extern crate url_serde;

use std::time::Duration;

use futures::{Future as StdFuture, IntoFuture, Stream as StdStream};
use hyper::client::{Connect, HttpConnector, Request};
use hyper::header::{Authorization, ContentType, Location, UserAgent};
use hyper::{Client, Method, StatusCode};
use hyper_multipart::client::multipart;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use tokio_core::reactor::Handle;
use url::Url;

pub mod comments;
pub mod errors;
pub mod files;
pub mod games;
pub mod mods;
pub mod types;

use comments::Comments;
use errors::Error;
use files::{Files, MyFiles};
use games::{Games, MyGames};
use mods::{Mods, MyMods};
use types::ModioErrorResponse;

const DEFAULT_HOST: &str = "https://api.mod.io/v1";

pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;
pub type Stream<T> = Box<StdStream<Item = T, Error = Error>>;
type MClient<T> = Client<T, multipart::Body>;

header! {
    (XRateLimitLimit, "X-RateLimit-Limit") => [u16]
}

header! {
    (XRateLimitRemaining, "X-RateLimit-Remaining") => [u16]
}

header! {
    (XRateLimitRetryAfter, "X-RateLimit-RetryAfter") => [u16]
}

#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    ApiKey(String),
    Token(String),
}

#[derive(Clone, Debug)]
pub struct Modio<C>
where
    C: Clone + Connect,
{
    host: String,
    agent: String,
    client: Client<C>,
    mclient: MClient<C>,
    credentials: Option<Credentials>,
}

#[cfg(feature = "tls")]
impl Modio<HttpsConnector<HttpConnector>> {
    pub fn new<A, C>(agent: A, credentials: C, handle: &Handle) -> Self
    where
        A: Into<String>,
        C: Into<Option<Credentials>>,
    {
        Self::host(DEFAULT_HOST, agent, credentials, handle)
    }

    pub fn host<H, A, C>(host: H, agent: A, credentials: C, handle: &Handle) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        C: Into<Option<Credentials>>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let client = Client::configure()
            .connector(connector.clone())
            .keep_alive(true)
            .build(handle);

        let mclient = Client::configure()
            .connector(connector)
            .body::<multipart::Body>()
            .keep_alive(true)
            .build(handle);

        Self::custom(host, agent, credentials, client, mclient)
    }
}

impl<C> Modio<C>
where
    C: Clone + Connect,
{
    pub fn custom<H, A, CR>(
        host: H,
        agent: A,
        credentials: CR,
        client: Client<C>,
        mclient: MClient<C>,
    ) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Option<Credentials>>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client,
            mclient,
            credentials: credentials.into(),
        }
    }

    pub fn games(&self) -> Games<C> {
        Games::new(self.clone())
    }

    pub fn my_games(&self) -> MyGames<C> {
        MyGames::new(self.clone())
    }

    pub fn my_mods(&self) -> MyMods<C> {
        MyMods::new(self.clone())
    }

    pub fn my_files(&self) -> MyFiles<C> {
        MyFiles::new(self.clone())
    }

    fn request<Out>(
        &self,
        method: Method,
        uri: String,
        body: Option<Vec<u8>>,
        content_type: Option<ContentType>,
    ) -> Future<Out>
    where
        Out: DeserializeOwned + 'static,
    {
        let url = if let Some(Credentials::ApiKey(ref api_key)) = self.credentials {
            let mut parsed = Url::parse(&uri).unwrap();
            parsed.query_pairs_mut().append_pair("api_key", api_key);
            parsed.to_string().parse().into_future()
        } else {
            uri.parse().into_future()
        };

        let instance = self.clone();
        let body2 = body.clone();
        let content_type2 = content_type.clone();
        let method2 = method.clone();

        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::new(method2, url);
            {
                let headers = req.headers_mut();
                headers.set(UserAgent::new(instance.agent.clone()));
                if let Some(Credentials::Token(token)) = instance.credentials {
                    headers.set(Authorization(format!("Bearer {}", token)));
                }
                if let Some(content_type) = content_type2 {
                    headers.set(content_type);
                }
            }
            if let Some(body) = body2 {
                req.set_body(body);
            }
            instance.client.request(req).map_err(Error::from)
        });

        let instance2 = self.clone();
        Box::new(response.and_then(move |response| {
            let remaining = response.headers().get::<XRateLimitRemaining>().map(|v| v.0);
            let reset = response
                .headers()
                .get::<XRateLimitRetryAfter>()
                .map(|v| v.0);

            let status = response.status();
            if StatusCode::MovedPermanently == status || StatusCode::TemporaryRedirect == status {
                if let Some(location) = response.headers().get::<Location>() {
                    return instance2.request(method, location.to_string(), body, content_type);
                }
            }
            Box::new(response.body().concat2().map_err(Error::from).and_then(
                move |response_body| {
                    if status.is_success() {
                        serde_json::from_slice::<Out>(&response_body)
                            .map_err(|err| Error::Codec(err).into())
                    } else {
                        let error = match (remaining, reset) {
                            (Some(remaining), Some(reset)) if remaining == 0 => Error::RateLimit {
                                reset: Duration::from_secs(reset as u64 * 60),
                            },
                            _ => {
                                let mer: ModioErrorResponse =
                                    serde_json::from_slice(&response_body)?;
                                Error::Fault {
                                    code: status,
                                    error: mer.error,
                                }
                            }
                        };
                        Err(error)
                    }
                },
            ))
        }))
    }

    fn get<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static,
    {
        self.request(Method::Get, self.host.clone() + uri, None, None)
    }
}
