use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use futures_util::{stream, StreamExt, TryStreamExt};
use pin_project_lite::pin_project;
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
    /// Returns the first search result.
    pub async fn first(mut self) -> Result<Option<T>> {
        self.filter = self.filter.limit(1);
        let list = self.first_page().await;
        list.map(|l| l.into_iter().next())
    }

    /// Returns the first search result page.
    pub async fn first_page(self) -> Result<Vec<T>> {
        let list = self.paged().await?.map_ok(|p| p.0.data).try_next().await;
        list.map(Option::unwrap_or_default)
    }

    /// Returns the complete search result list.
    pub async fn collect(self) -> Result<Vec<T>> {
        self.paged().await?.map_ok(|p| p.0.data).try_concat().await
    }

    /// Provides a stream over all search result items.
    pub async fn iter(self) -> Result<impl Stream<Item = Result<T>>> {
        let (st, (total, _)) = stream(self.modio, self.route, self.filter).await?;
        let st = st
            .map_ok(|list| stream::iter(list.into_iter().map(Ok)))
            .try_flatten();
        Ok(Box::pin(ResultStream::new(total as usize, st)))
    }

    /// Provides a stream over all search result pages.
    pub async fn paged(self) -> Result<impl Stream<Item = Result<Page<T>>>> {
        let (st, (total, limit)) = stream(self.modio, self.route, self.filter).await?;
        let size_hint = if total == 0 {
            0
        } else {
            (total - 1) / limit + 1
        };
        Ok(Box::pin(ResultStream::new(size_hint as usize, st)))
    }
}

async fn stream<T>(
    modio: Modio,
    route: Route,
    filter: Filter,
) -> Result<(impl Stream<Item = Result<Page<T>>>, (u32, u32))>
where
    T: DeserializeOwned + Send,
{
    struct State {
        offset: u32,
        limit: u32,
        remaining: u32,
    }
    let list = modio
        .request(route)
        .query(filter.to_query_string())
        .send::<List<T>>()
        .await?;

    let state = State {
        offset: list.offset,
        limit: list.limit,
        remaining: list.total - list.count,
    };
    let initial = (modio, route, filter, state);
    let stats = (list.total, list.limit);

    let first = stream::once(async { Ok::<_, crate::Error>(Page(list)) });

    let others = stream::try_unfold(initial, |(modio, route, filter, state)| async move {
        if let State { remaining: 0, .. } = state {
            return Ok(None);
        }
        let filter = filter.offset((state.offset + state.limit) as usize);
        let remaining = state.remaining;

        let list = modio
            .request(route)
            .query(filter.to_query_string())
            .send::<List<T>>()
            .await?;

        let state = (
            modio,
            route,
            filter,
            State {
                offset: list.offset,
                limit: list.limit,
                remaining: remaining - list.count,
            },
        );

        Ok(Some((Page(list), state)))
    });

    Ok((first.chain(others), stats))
}

/// A `Page` returned by the [`Query::paged`] stream for a search result.
pub struct Page<T>(List<T>);

impl<T> Page<T> {
    pub fn data(&self) -> &Vec<T> {
        &self.0.data
    }

    pub fn into_data(self) -> Vec<T> {
        self.0.data
    }

    /// Returns the current page number.
    pub fn current(&self) -> usize {
        self.0.offset as usize / self.page_size() + 1
    }

    /// Returns the number of pages.
    pub fn page_count(&self) -> usize {
        if self.0.total == 0 {
            0
        } else {
            (self.total() - 1) / self.page_size() + 1
        }
    }

    /// Returns the size of a page.
    pub fn page_size(&self) -> usize {
        self.0.limit as usize
    }

    /// Returns the total number of the search result.
    pub fn total(&self) -> usize {
        self.0.total as usize
    }
}

// Impl IntoIterator & Deref for Page<T> {{{
impl<T> std::ops::Deref for Page<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<'a, T> std::iter::IntoIterator for &'a Page<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> std::slice::Iter<'a, T> {
        self.0.data.iter()
    }
}

impl<T> std::iter::IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.0.data.into_iter()
    }
}
// }}}

pin_project! {
    struct ResultStream<St> {
        total: usize,
        #[pin]
        stream: St,
    }
}

impl<St: Stream> ResultStream<St> {
    fn new(total: usize, stream: St) -> ResultStream<St> {
        Self { total, stream }
    }
}

impl<St: Stream> Stream for ResultStream<St> {
    type Item = St::Item;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total as usize, None)
    }

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}

// vim: fdm=marker
