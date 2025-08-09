use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::mods::Mod;
use crate::types::List;

/// Get all mod the authenticated user is subscribed to.
pub struct GetUserSubscriptions<'a> {
    http: &'a Client,
    filter: Option<Filter>,
}

impl<'a> GetUserSubscriptions<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http, filter: None }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetUserSubscriptions<'_> {
    type Output = Output<List<Mod>>;
    type IntoFuture = ResponseFuture<List<Mod>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserSubscriptions;
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
