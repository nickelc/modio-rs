use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::{Event, List};

/// Get events that have been fired specific to the authenticated user.
pub struct GetUserEvents<'a> {
    http: &'a Client,
    filter: Option<Filter>,
}

impl<'a> GetUserEvents<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http, filter: None }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetUserEvents<'_> {
    type Output = Output<List<Event>>;
    type IntoFuture = ResponseFuture<List<Event>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserEvents;
        let mut builder = RequestBuilder::from_route(&route);
        if let Some(filter) = self.filter {
            builder = builder.filter(filter);
        }
        match builder.empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
