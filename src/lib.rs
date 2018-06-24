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
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate url;
extern crate url_serde;

use std::marker::PhantomData;
use std::time::Duration;

use futures::{future, Future as StdFuture, IntoFuture, Stream as StdStream};
use hyper::client::{Connect, HttpConnector, Request};
use hyper::header::{Authorization, Bearer, ContentType, Location, UserAgent};
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
pub mod me;
pub mod mods;
pub mod teams;
pub mod types;
pub mod users;

use comments::Comments;
use errors::Error;
use files::Files;
use games::{GameRef, Games};
use me::Me;
use mods::{ModRef, Mods};
use types::{ModioErrorResponse, ModioListResponse, ModioMessage};
use users::Users;

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

    pub fn game(&self, game_id: u32) -> GameRef<C> {
        GameRef::new(self.clone(), game_id)
    }

    pub fn mod_(&self, game_id: u32, mod_id: u32) -> ModRef<C> {
        ModRef::new(self.clone(), game_id, mod_id)
    }

    pub fn me(&self) -> Me<C> {
        Me::new(self.clone())
    }

    pub fn users(&self) -> Users<C> {
        Users::new(self.clone())
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
                    headers.set(Authorization(Bearer { token }));
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

    fn formdata<F, Out>(&self, method: Method, uri: String, data: F) -> Future<Out>
    where
        Out: DeserializeOwned + 'static,
        F: MultipartForm + Clone + 'static,
    {
        let url = if let Some(Credentials::ApiKey(ref api_key)) = self.credentials {
            let mut parsed = Url::parse(&uri).unwrap();
            parsed.query_pairs_mut().append_pair("api_key", api_key);
            parsed.to_string().parse().into_future()
        } else {
            uri.parse().into_future()
        };

        let instance = self.clone();
        let method2 = method.clone();
        let form = match data.to_form() {
            Ok(form) => form,
            Err(err) => return Box::new(future::err(err)),
        };

        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::new(method2, url);
            {
                let headers = req.headers_mut();
                headers.set(UserAgent::new(instance.agent.clone()));
                if let Some(Credentials::Token(token)) = instance.credentials {
                    headers.set(Authorization(Bearer { token }));
                }
            }
            form.set_body(&mut req);
            instance.mclient.request(req).map_err(Error::from)
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
                    return instance2.formdata(method, location.to_string(), data);
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

    fn post<D, M>(&self, uri: &str, message: M) -> Future<D>
    where
        D: DeserializeOwned + 'static,
        M: Into<Vec<u8>>,
    {
        self.request(
            Method::Post,
            self.host.clone() + uri,
            Some(message.into()),
            Some(ContentType::form_url_encoded()),
        )
    }

    fn post_form<F, D>(&self, uri: &str, data: F) -> Future<D>
    where
        D: DeserializeOwned + 'static,
        F: MultipartForm + Clone + 'static,
    {
        self.formdata(Method::Post, self.host.clone() + uri, data)
    }

    fn put<D, M>(&self, uri: &str, message: M) -> Future<D>
    where
        D: DeserializeOwned + 'static,
        M: Into<Vec<u8>>,
    {
        self.request(
            Method::Put,
            self.host.clone() + uri,
            Some(message.into()),
            Some(ContentType::form_url_encoded()),
        )
    }

    fn delete<M>(&self, uri: &str, message: M) -> Future<()>
    where
        M: Into<Vec<u8>>,
    {
        Box::new(self.request(
            Method::Delete,
            self.host.clone() + uri,
            Some(message.into()),
            Some(ContentType::form_url_encoded()),
        ).or_else(|err| match err {
            errors::Error::Codec(_) => Ok(()),
            otherwise => Err(otherwise.into()),
        }))
    }
}

pub struct Endpoint<C, Out>
where
    C: Clone + Connect,
    Out: DeserializeOwned + 'static,
{
    modio: Modio<C>,
    path: String,
    phantom: PhantomData<Out>,
}

impl<C, Out> Endpoint<C, Out>
where
    C: Clone + Connect,
    Out: DeserializeOwned + 'static,
{
    pub fn new(modio: Modio<C>, path: String) -> Endpoint<C, Out> {
        Self {
            modio,
            path,
            phantom: PhantomData,
        }
    }

    pub fn list(&self) -> Future<ModioListResponse<Out>> {
        self.modio.get(&self.path)
    }

    pub fn add<T: AddOptions + QueryParams>(&self, options: T) -> Future<ModioMessage> {
        let params = options.to_query_params();
        self.modio.post(&self.path, params)
    }

    pub fn delete<T: DeleteOptions + QueryParams>(&self, options: T) -> Future<()> {
        let params = options.to_query_params();
        self.modio.delete(&self.path, params)
    }
}

trait MultipartForm {
    fn to_form(&self) -> Result<multipart::Form, errors::Error>;
}

pub trait AddOptions {}
pub trait DeleteOptions {}

pub trait QueryParams {
    fn to_query_params(&self) -> String;
}
