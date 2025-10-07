//! Typed request builders for the different endpoints.

use bytes::Bytes;
use futures_util::TryStream;
use http::header::{HeaderName, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use http::request::Builder;
use serde::ser::Serialize;

use crate::error::{self, Error};
use crate::response::Response;

use self::body::Body;
use self::multipart::Form;
use self::routing::Parts;
pub(crate) use self::routing::Route;
use self::util::ArrayParams;

pub use self::filter::Filter;
pub use self::submit_report::SubmitReport;

pub(crate) mod body;
mod multipart;
mod routing;
mod submit_report;
mod util;

pub mod auth;
#[macro_use]
pub mod filter;
pub mod files;
pub mod games;
pub mod mods;
pub mod user;

pub(crate) type Request = http::Request<Body>;
pub(crate) type Output<T> = Result<Response<T>, Error>;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TokenRequired(pub bool);

pub(crate) struct RequestBuilder {
    inner: Builder,
}

impl RequestBuilder {
    pub fn from_route(route: &Route) -> Self {
        let Parts {
            method,
            path,
            token_required,
        } = route.into_parts();

        Self {
            inner: Builder::new()
                .uri(path)
                .method(method)
                .extension(TokenRequired(token_required)),
        }
    }

    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        <K as TryInto<HeaderName>>::Error: Into<http::Error>,
        V: TryInto<HeaderValue>,
        <V as TryInto<HeaderValue>>::Error: Into<http::Error>,
    {
        Self {
            inner: self.inner.header(key, value),
        }
    }

    pub fn filter(self, filter: Filter) -> Self {
        Self {
            inner: self.inner.extension(filter),
        }
    }

    pub fn empty(self) -> Result<Request, Error> {
        build(self.inner, Body::empty())
    }

    pub fn stream<S>(self, stream: S) -> Result<Request, Error>
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        build(self.inner, Body::from_stream(stream))
    }

    pub fn form<T: Serialize>(self, form: &T) -> Result<Request, Error> {
        let body = serde_urlencoded::to_string(form).map_err(error::builder)?;
        let builder = self.inner.header(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        build(builder, Body::from(body))
    }

    pub fn multipart(self, form: Form) -> Result<Request, Error> {
        let mut builder = match HeaderValue::from_maybe_shared(form.content_type()) {
            Ok(value) => self.inner.header(CONTENT_TYPE, value),
            Err(_) => self.inner,
        };

        builder = match form.compute_length() {
            Some(length) => builder.header(CONTENT_LENGTH, length),
            None => builder,
        };
        build(builder, Body::from_stream(form.stream()))
    }
}

#[inline]
fn build(builder: Builder, body: Body) -> Result<Request, Error> {
    builder.body(body).map_err(error::builder)
}
