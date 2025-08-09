use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::Message;

/// Request a security code for a user.
pub struct EmailRequest<'a> {
    http: &'a Client,
    fields: EmailRequestFields<'a>,
}

#[derive(Serialize)]
struct EmailRequestFields<'a> {
    email: &'a str,
}

impl<'a> EmailRequest<'a> {
    pub(crate) const fn new(http: &'a Client, email: &'a str) -> Self {
        Self {
            http,
            fields: EmailRequestFields { email },
        }
    }
}

impl IntoFuture for EmailRequest<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::OAuthEmailRequest;
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
