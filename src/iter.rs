use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

use futures_core::future::BoxFuture;
use futures_core::Stream;
use futures_util::stream;
use futures_util::FutureExt;
use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::filter::Filter;
use crate::routing::Route;
use crate::{List, Modio, QueryString};

pub struct Iter<'a, T> {
    modio: Modio,
    route: Route,
    filter: Filter,
    inner: Inner<'a, T>,
}

impl<'a, T: DeserializeOwned + Send + 'a> Iter<'a, T> {
    pub fn new(modio: Modio, route: Route, filter: Filter) -> Self {
        let req = modio
            .request(route.clone())
            .query(filter.to_query_string())
            .send()
            .boxed();
        Iter {
            modio: modio.clone(),
            route,
            filter,
            inner: Inner::Request(req, None),
        }
    }
}

enum Inner<'a, T> {
    Request(BoxFuture<'a, Result<List<T>>>, Option<u32>),
    List(State, stream::Iter<IntoIter<T>>),
}

#[derive(Debug)]
struct State {
    offset: u32,
    limit: u32,
    remaining: u32,
}

impl<'a, T: 'a + DeserializeOwned + Send> Stream for Iter<'a, T> {
    type Item = Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let modio = self.modio.clone();
        let route = self.route.clone();
        let filter = self.filter.clone();
        let new_value = match self.inner {
            Inner::Request(ref mut fut, remaining) => match Pin::new(fut).poll(cx) {
                Poll::Ready(Ok(list)) => {
                    let s = State {
                        offset: list.offset,
                        limit: list.limit,
                        remaining: remaining.unwrap_or(list.total),
                    };
                    Inner::List(s, stream::iter(list.data))
                }
                Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                Poll::Pending => return Poll::Pending,
            },
            Inner::List(ref mut state, ref mut st) => {
                match futures_core::ready!(Pin::new(st).poll_next(cx)) {
                    Some(elem) => {
                        state.remaining -= 1;
                        return Poll::Ready(Some(Ok(elem)));
                    }
                    None if state.remaining == 0 => return Poll::Ready(None),
                    None => {
                        let filter = filter.offset((state.offset + state.limit) as usize);
                        Inner::Request(
                            modio
                                .request(route)
                                .query(filter.to_query_string())
                                .send()
                                .boxed(),
                            Some(state.remaining),
                        )
                    }
                }
            }
        };
        self.inner = new_value;
        self.poll_next(cx)
    }
}
