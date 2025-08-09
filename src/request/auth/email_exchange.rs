use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::auth::AccessToken;

/// Request an access token in exchange for a security code.
pub struct EmailExchange<'a> {
    http: &'a Client,
    fields: EmailExchangeFields<'a>,
}

#[derive(Serialize)]
struct EmailExchangeFields<'a> {
    security_code: &'a str,
    #[serde(rename = "date_expires", skip_serializing_if = "Option::is_none")]
    expired_at: Option<u64>,
}

impl<'a> EmailExchange<'a> {
    pub(crate) const fn new(http: &'a Client, security_code: &'a str) -> Self {
        Self {
            http,
            fields: EmailExchangeFields {
                security_code,
                expired_at: None,
            },
        }
    }

    pub const fn expired_at(mut self, expired_at: u64) -> Self {
        self.fields.expired_at = Some(expired_at);
        self
    }
}

impl IntoFuture for EmailExchange<'_> {
    type Output = Output<AccessToken>;
    type IntoFuture = ResponseFuture<AccessToken>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::OAuthEmailExchange;
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
