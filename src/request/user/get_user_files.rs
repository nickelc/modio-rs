use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Filter, Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::files::File;
use crate::types::List;
use crate::util::{Paginate, Paginator};

/// Get all modfiles the authenticated user uploaded.
pub struct GetUserFiles<'a> {
    http: &'a Client,
    filter: Option<Filter>,
}

impl<'a> GetUserFiles<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http, filter: None }
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl IntoFuture for GetUserFiles<'_> {
    type Output = Output<List<File>>;
    type IntoFuture = ResponseFuture<List<File>>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserFiles;
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

impl<'a> Paginate<'a> for GetUserFiles<'a> {
    type Output = File;

    fn paged(&'a self) -> Paginator<'a, Self::Output> {
        let route = Route::UserFiles;

        Paginator::new(self.http, route, self.filter.clone())
    }
}
