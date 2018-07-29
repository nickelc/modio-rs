extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate hyper_tls;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate url_serde;

use std::marker::PhantomData;
use std::time::Duration;

use futures::{Future as StdFuture, IntoFuture, Stream as StdStream};
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE, LOCATION, USER_AGENT};
use hyper::{Body, Client, Method, Request, StatusCode, Uri};
use hyper_multipart::client::multipart;
use hyper_tls::HttpsConnector;
use mime::Mime;
use serde::de::DeserializeOwned;
use url::Url;

pub mod auth;
#[macro_use]
pub mod filter;
pub mod comments;
pub mod error;
pub mod files;
pub mod games;
pub mod me;
pub mod metadata;
pub mod mods;
pub mod reports;
pub mod teams;
mod types;
pub mod users;

use auth::{Auth, Credentials};
use comments::Comments;
use error::Error;
use games::{GameRef, Games};
use me::Me;
use mods::{ModRef, Mods};
use reports::Reports;
use users::Users;

pub use types::{Event, EventType, ModioErrorResponse, ModioListResponse, ModioMessage};

const DEFAULT_HOST: &str = "https://api.mod.io/v1";

pub type Future<T> = Box<StdFuture<Item = T, Error = Error> + Send>;
pub type Stream<T> = Box<StdStream<Item = T, Error = Error> + Send>;

#[allow(dead_code)]
const X_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
const X_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";
const X_RATELIMIT_RETRY_AFTER: &str = "x-ratelimit-retryafter";

/// Endpoint interface to interacting with the [mod.io](https://mod.io) API.
#[derive(Clone, Debug)]
pub struct Modio<C>
where
    C: Clone + Connect + 'static,
{
    host: String,
    agent: String,
    client: Client<C>,
    credentials: Credentials,
}

impl Modio<HttpsConnector<HttpConnector>> {
    /// Create an endpoint to [http://api.mod.io/v1](https://docs.mod.io/#mod-io-api-v1).
    pub fn new<A, C>(agent: A, credentials: C) -> Self
    where
        A: Into<String>,
        C: Into<Credentials>,
    {
        Self::host(DEFAULT_HOST, agent, credentials)
    }

    pub fn host<H, A, C>(host: H, agent: A, credentials: C) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        C: Into<Credentials>,
    {
        let connector = HttpsConnector::new(4).unwrap();
        let client = Client::builder().keep_alive(true).build(connector);

        Self::custom(host, agent, credentials, client)
    }
}

impl<C> Modio<C>
where
    C: Clone + Connect + 'static,
{
    pub fn custom<H, A, CR>(host: H, agent: A, credentials: CR, client: Client<C>) -> Self
    where
        H: Into<String>,
        A: Into<String>,
        CR: Into<Credentials>,
    {
        Self {
            host: host.into(),
            agent: agent.into(),
            client,
            credentials: credentials.into(),
        }
    }

    /// Consume the endpoint and create an endpoint with new credentials.
    pub fn with_credentials<CR>(self, credentials: CR) -> Self
    where
        CR: Into<Credentials>,
    {
        Self {
            host: self.host,
            agent: self.agent,
            client: self.client,
            credentials: credentials.into(),
        }
    }

    /// Return a reference to an interface for requesting access tokens.
    pub fn auth(&self) -> Auth<C> {
        Auth::new(self.clone())
    }

    /// Return a reference to an interface that provides access to game informations.
    pub fn games(&self) -> Games<C> {
        Games::new(self.clone())
    }

    /// Return a reference to a game.
    pub fn game(&self, game_id: u32) -> GameRef<C> {
        GameRef::new(self.clone(), game_id)
    }

    /// Return a reference to a mod.
    pub fn mod_(&self, game_id: u32, mod_id: u32) -> ModRef<C> {
        ModRef::new(self.clone(), game_id, mod_id)
    }

    /// Return a reference to an interface that provides access to resources owned by the user
    /// associated with the current authentication credentials.
    pub fn me(&self) -> Me<C> {
        Me::new(self.clone())
    }

    /// Return a reference to an interface that provides access to user informations.
    pub fn users(&self) -> Users<C> {
        Users::new(self.clone())
    }

    /// Return a reference to an interface to report games, mods and users.
    pub fn reports(&self) -> Reports<C> {
        Reports::new(self.clone())
    }

    fn request<Out>(&self, method: Method, uri: &str, body: RequestBody) -> Future<Out>
    where
        Out: DeserializeOwned + 'static + Send,
    {
        let url = if let Credentials::ApiKey(ref api_key) = self.credentials {
            let mut parsed = Url::parse(&uri).unwrap();
            parsed.query_pairs_mut().append_pair("api_key", api_key);
            parsed.to_string().parse::<Uri>().into_future()
        } else {
            uri.parse().into_future()
        };

        let instance = self.clone();
        let body2 = body.clone();
        let method2 = method.clone();

        let response = url.map_err(Error::from).and_then(move |url| {
            let mut req = Request::builder();
            req.method(method2)
                .uri(url)
                .header(USER_AGENT, &*instance.agent);

            if let Credentials::Token(ref token) = instance.credentials {
                req.header(AUTHORIZATION, &*format!("Bearer {}", token));
            }

            let req = match body2 {
                RequestBody::Vec(body, mime) => {
                    req.header(CONTENT_TYPE, &*mime.to_string());
                    req.body(Body::from(body)).map_err(Error::from)
                }
                RequestBody::Form(data) => data.to_form()
                    .and_then(move |form| form.set_body(&mut req).map_err(Error::from)),
                RequestBody::Empty => req.body(Body::empty()).map_err(Error::from),
            };

            req.into_future()
                .and_then(move |req| instance.client.request(req).map_err(Error::from))
        });

        let instance2 = self.clone();
        Box::new(response.and_then(move |response| {
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
            if StatusCode::MOVED_PERMANENTLY == status || StatusCode::TEMPORARY_REDIRECT == status {
                if let Some(location) = response.headers().get(LOCATION) {
                    let location = location.to_str().unwrap().to_owned();
                    return instance2.request(method, &location, body);
                }
            }
            Box::new(
                response
                    .into_body()
                    .concat2()
                    .map_err(Error::from)
                    .and_then(move |response_body| {
                        if status.is_success() {
                            serde_json::from_slice::<Out>(&response_body).map_err(Error::from)
                        } else {
                            let error = match (remaining, reset) {
                                (Some(remaining), Some(reset)) if remaining == 0 => {
                                    Error::RateLimit {
                                        reset: Duration::from_secs(reset as u64 * 60),
                                    }
                                }
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
                    }),
            )
        }))
    }

    fn get<D>(&self, uri: &str) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
    {
        self.request(Method::GET, &(self.host.clone() + uri), RequestBody::Empty)
    }

    fn post<D, M>(&self, uri: &str, message: M) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        M: Into<Vec<u8>>,
    {
        self.request(
            Method::POST,
            &(self.host.clone() + uri),
            RequestBody::Vec(message.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
        )
    }

    fn post_form<F, D>(&self, uri: &str, data: F) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        F: MultipartForm + Clone + 'static,
    {
        self.request(
            Method::POST,
            &(self.host.clone() + uri),
            RequestBody::Form(Box::new(data)),
        )
    }

    fn put<D, M>(&self, uri: &str, message: M) -> Future<D>
    where
        D: DeserializeOwned + 'static + Send,
        M: Into<Vec<u8>>,
    {
        self.request(
            Method::PUT,
            &(self.host.clone() + uri),
            RequestBody::Vec(message.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
        )
    }

    fn delete<M>(&self, uri: &str, message: M) -> Future<()>
    where
        M: Into<Vec<u8>>,
    {
        Box::new(self.request(
            Method::DELETE,
            &(self.host.clone() + uri),
            RequestBody::Vec(message.into(), mime::APPLICATION_WWW_FORM_URLENCODED),
        ).or_else(|err| match err {
            error::Error::Codec(_) => Ok(()),
            otherwise => Err(otherwise),
        }))
    }
}

#[derive(Clone)]
enum RequestBody {
    Empty,
    Vec(Vec<u8>, Mime),
    Form(Box<MultipartForm>),
}

/// Generic endpoint for sub-resources
pub struct Endpoint<C, Out>
where
    C: Clone + Connect + 'static,
    Out: DeserializeOwned + 'static,
{
    modio: Modio<C>,
    path: String,
    phantom: PhantomData<Out>,
}

impl<C, Out> Endpoint<C, Out>
where
    C: Clone + Connect,
    Out: DeserializeOwned + 'static + Send,
{
    pub(crate) fn new(modio: Modio<C>, path: String) -> Endpoint<C, Out> {
        Self {
            modio,
            path,
            phantom: PhantomData,
        }
    }

    pub fn list(&self) -> Future<ModioListResponse<Out>> {
        self.modio.get(&self.path)
    }

    pub fn add<T: AddOptions + QueryParams>(&self, options: &T) -> Future<ModioMessage> {
        let params = options.to_query_params();
        self.modio.post(&self.path, params)
    }

    pub fn delete<T: DeleteOptions + QueryParams>(&self, options: &T) -> Future<()> {
        let params = options.to_query_params();
        self.modio.delete(&self.path, params)
    }
}

filter_options!{
    /// Options used to filter event listings.
    ///
    /// # Filter parameters
    /// - id
    /// - mod_id
    /// - user_id
    /// - date_added
    /// - event_type
    ///
    /// # Sorting
    /// - id
    ///
    /// See the [modio docs](https://docs.mod.io/#events) for more informations.
    ///
    /// By default this returns up to `100` items. You can limit the result using `limit` and
    /// `offset`.
    /// # Example
    /// ```
    /// use modio::filter::{Order, Operator};
    /// use modio::EventListOptions;
    /// use modio::EventType;
    ///
    /// let mut opts = EventListOptions::new();
    /// opts.id(Operator::GreaterThan, 1024);
    /// opts.event_type(Operator::Equals, EventType::ModfileChanged);
    /// ```
    #[derive(Debug)]
    pub struct EventListOptions {
        Filters
        - id = "id";
        - mod_id = "mod_id";
        - user_id = "user_id";
        - date_added = "date_added";
        - event_type = "event_type";

        Sort
        - ID = "id";
    }
}

trait MultipartForm: MultipartFormClone + Send {
    fn to_form(&self) -> Result<multipart::Form, error::Error>;
}

trait MultipartFormClone {
    fn clone_box(&self) -> Box<MultipartForm>;
}

impl<T> MultipartFormClone for T
where
    T: 'static + MultipartForm + Clone,
{
    fn clone_box(&self) -> Box<MultipartForm> {
        Box::new(self.clone())
    }
}

impl Clone for Box<MultipartForm> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait AddOptions {}
pub trait DeleteOptions {}

pub trait QueryParams {
    fn to_query_params(&self) -> String;
}
