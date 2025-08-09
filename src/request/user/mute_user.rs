use std::future::IntoFuture;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::{NoContent, ResponseFuture};
use crate::types::id::UserId;

/// Mute a user.
pub struct MuteUser<'a> {
    http: &'a Client,
    user_id: UserId,
}

impl<'a> MuteUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: UserId) -> Self {
        Self { http, user_id }
    }
}

impl IntoFuture for MuteUser<'_> {
    type Output = Output<NoContent>;
    type IntoFuture = ResponseFuture<NoContent>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::MuteUser {
            user_id: self.user_id,
        };
        match RequestBuilder::from_route(&route).empty() {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
