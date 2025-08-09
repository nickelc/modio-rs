use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::{List, User};

/// Get all users muted by the authenticated user.
pub struct GetMutedUsers<'a> {
    http: &'a Client,
    filter: Option<Filter>,
}

impl<'a> GetMutedUsers<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http, filter: None }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetMutedUsers<'_> {
    type Output = Output<List<User>>;
    type IntoFuture = ResponseFuture<List<User>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserMuted;
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
