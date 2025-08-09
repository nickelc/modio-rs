use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use hyper::body::Incoming;
use hyper_util::client::legacy::{Builder, Client as LegacyClient};
use hyper_util::rt::TokioExecutor;
use tower::buffer::{self, Buffer};
use tower::{Service, ServiceBuilder};
#[cfg(feature = "trace")]
use tower_http::classify::{NeverClassifyEos, ServerErrorsFailureClass};
use tower_http::compression::{Compression, CompressionBody};
use tower_http::decompression::{Decompression, DecompressionBody};
use tower_http::follow_redirect;
use tower_http::follow_redirect::policy::{And, FilterCredentials, Limited};
#[cfg(feature = "trace")]
use tower_http::trace::{HttpMakeClassifier, ResponseBody, Trace, TraceLayer};
use tower_http::ServiceBuilderExt;

use super::conn::{self, Connector};

use crate::error::{self, Error};
use crate::request::{body, Request};

#[cfg(not(feature = "trace"))]
type Client = LegacyClient<Connector, body::Body>;
#[cfg(feature = "trace")]
type Client = Trace<LegacyClient<Connector, body::Body>, HttpMakeClassifier>;

#[cfg(not(feature = "trace"))]
type InnerBody = Incoming;
#[cfg(feature = "trace")]
type InnerBody = ResponseBody<Incoming, NeverClassifyEos<ServerErrorsFailureClass>>;

pub type Body = CompressionBody<DecompressionBody<InnerBody>>;
pub type Response = http::Response<Body>;

type StackResponseFuture = follow_redirect::ResponseFuture<
    Compression<Decompression<Client>>,
    body::Body,
    And<Limited, FilterCredentials>,
>;
type BufferedService = Buffer<Request, StackResponseFuture>;
type BufferedResponseFuture = buffer::future::ResponseFuture<StackResponseFuture>;

#[derive(Clone)]
pub struct Svc {
    inner: BufferedService,
}

impl Svc {
    pub fn new() -> Self {
        let conn = conn::create_connector();
        let client = Builder::new(TokioExecutor::new()).build(conn);

        let service = ServiceBuilder::new()
            .buffer(1024)
            .follow_redirects()
            .compression()
            .decompression();

        #[cfg(feature = "trace")]
        let service = service.layer(TraceLayer::new_for_http());

        Self {
            inner: service.service(client),
        }
    }

    pub fn request(&self, request: Request) -> ResponseFuture {
        let svc = self.inner.clone();

        ResponseFuture {
            inner: State::Ready(svc, Box::new(request)),
        }
    }
}

pub struct ResponseFuture {
    inner: State,
}

enum State {
    Ready(BufferedService, Box<Request>),
    Call(Box<BufferedResponseFuture>),
    Completed,
}

impl Future for ResponseFuture {
    type Output = Result<Response, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let state = std::mem::replace(&mut self.inner, State::Completed);

            match state {
                State::Ready(mut svc, request) => match svc.poll_ready(cx) {
                    Poll::Pending => {
                        self.inner = State::Ready(svc, request);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(error::service(e))),
                    Poll::Ready(Ok(())) => {
                        self.inner = State::Call(Box::new(svc.call(*request)));
                    }
                },
                State::Call(mut fut) => match Pin::new(&mut fut).poll(cx) {
                    Poll::Pending => {
                        self.inner = State::Call(fut);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(error::service(e))),
                    Poll::Ready(Ok(resp)) => {
                        return Poll::Ready(Ok(resp));
                    }
                },
                State::Completed => panic!("future already completed"),
            }
        }
    }
}
