use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use http::{header, StatusCode};

use super::{ErrorResponseFuture, Output, Response};
use crate::client;
use crate::error::{self, Error};

/// A `Future` that will resolve to a [`Response`].
pub struct ResponseFuture<T> {
    state: State,
    phantom: PhantomData<T>,
}

enum State {
    InFlight(client::service::ResponseFuture),
    ErrorResponse(StatusCode, ErrorResponseFuture),
    Failed(Error),
    Completed,
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(fut: client::service::ResponseFuture) -> Self {
        Self {
            state: State::InFlight(fut),
            phantom: PhantomData,
        }
    }

    pub(crate) fn failed(error: Error) -> Self {
        Self {
            state: State::Failed(error),
            phantom: PhantomData,
        }
    }
}

impl<T: Unpin> Future for ResponseFuture<T> {
    type Output = Output<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let state = std::mem::replace(&mut self.state, State::Completed);

            match state {
                State::InFlight(mut fut) => {
                    let resp = match Pin::new(&mut fut).poll(cx) {
                        Poll::Ready(Ok(resp)) => Response::new(resp),
                        Poll::Ready(Err(err)) => return Poll::Ready(Err(error::request(err))),
                        Poll::Pending => {
                            self.state = State::InFlight(fut);
                            return Poll::Pending;
                        }
                    };

                    if resp.status().is_success() {
                        return Poll::Ready(Ok(resp));
                    }

                    let retry_after = resp
                        .headers()
                        .get(header::RETRY_AFTER)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse().ok());

                    if let Some(retry_after) = retry_after {
                        return Poll::Ready(Err(error::ratelimit(retry_after)));
                    }

                    self.state = State::ErrorResponse(resp.status(), resp.error());
                }
                State::ErrorResponse(status, mut fut) => {
                    let error = match Pin::new(&mut fut).poll(cx) {
                        Poll::Ready(Ok(resp)) => resp.error,
                        Poll::Ready(Err(err)) => return Poll::Ready(Err(error::request(err))),
                        Poll::Pending => {
                            self.state = State::ErrorResponse(status, fut);
                            return Poll::Pending;
                        }
                    };

                    return Poll::Ready(Err(error::error_for_status(status, error)));
                }
                State::Failed(err) => return Poll::Ready(Err(err)),
                State::Completed => panic!("future is already completed"),
            }
        }
    }
}
