use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::UserId;

/// Unmute a previously muted user.
pub struct UnmuteUser<'a> {
    http: &'a Client,
    user_id: UserId,
}

impl<'a> UnmuteUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: UserId) -> Self {
        Self { http, user_id }
    }
}

impl IntoFuture for UnmuteUser<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::UnmuteUser {
            user_id: self.user_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
