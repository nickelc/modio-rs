use std::marker::PhantomData;

use futures_core::Stream;
use futures_util::{stream, TryStreamExt};
use serde::de::DeserializeOwned;

use crate::filter::Filter;
use crate::routing::Route;
use crate::types::List;
use crate::{Modio, QueryString, Result};

/// Interface for retrieving search results.
pub struct Query<T> {
    modio: Modio,
    route: Route,
    filter: Filter,
    phantom: PhantomData<T>,
}

impl<T> Query<T> {
    pub(crate) fn new(modio: Modio, route: Route, filter: Filter) -> Self {
        Self {
            modio,
            route,
            filter,
            phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Send> Query<T> {
    /// Returns the first search result page.
    pub async fn first_page(self) -> Result<Vec<T>> {
        let list = self.bulk().try_next().await;
        list.map(Option::unwrap_or_default)
    }

    /// Returns the complete search result list.
    pub async fn collect(self) -> Result<Vec<T>> {
        self.bulk().try_concat().await
    }

    /// Provides a stream over all search result items.
    pub fn iter(self) -> impl Stream<Item = Result<T>> {
        self.bulk()
            .map_ok(|list| stream::iter(list.into_iter().map(Ok)))
            .try_flatten()
    }

    /// Provides a stream over all search result pages.
    pub fn bulk(self) -> impl Stream<Item = Result<Vec<T>>> {
        struct State {
            offset: u32,
            limit: u32,
            remaining: u32,
        }
        let modio = self.modio;
        let route = self.route;
        let filter = self.filter;
        let initial = (modio, route, filter, None);
        let s = stream::try_unfold(initial, |(modio, route, filter, state)| async move {
            let (filter, remaining) = match state {
                Some(State { remaining: 0, .. }) => return Ok(None),
                None => (filter, None),
                Some(s) => {
                    let filter = filter.offset((s.offset + s.limit) as usize);
                    (filter, Some(s.remaining))
                }
            };

            let list = modio
                .request(route)
                .query(filter.to_query_string())
                .send::<List<T>>()
                .await?;

            let state = (
                modio,
                route,
                filter,
                Some(State {
                    offset: list.offset,
                    limit: list.limit,
                    remaining: remaining.unwrap_or(list.total) - list.count,
                }),
            );

            Ok(Some((list.data, state)))
        });
        Box::pin(s)
    }
}
