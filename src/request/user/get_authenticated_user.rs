use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::User;

/// Get the authenticated user details.
pub struct GetAuthenticatedUser<'a> {
    http: &'a Client,
}

impl<'a> GetAuthenticatedUser<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetAuthenticatedUser<'_> {
    type Output = Output<User>;
    type IntoFuture = ResponseFuture<User>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UserAuthenticated;

        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
