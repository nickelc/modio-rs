use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::auth::Terms;

/// Retrieve texts and links for a users agreement and consent.
pub struct GetTerms<'a> {
    http: &'a Client,
}

impl<'a> GetTerms<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetTerms<'_> {
    type Output = Output<Terms>;
    type IntoFuture = ResponseFuture<Terms>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::Terms;
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
